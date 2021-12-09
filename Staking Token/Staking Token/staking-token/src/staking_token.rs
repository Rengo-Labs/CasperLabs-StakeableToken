use alloc::string::ToString;
use alloc::{string::String, vec::Vec};

use crate::data::{self};

use crate::config::*;
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_contract::contract_api::runtime;
use casper_types::{runtime_args, ApiError, ContractHash, Key, RuntimeArgs, U256, bytesrepr::{ToBytes, FromBytes}};
use contract_utils::{ContractContext, ContractStorage};

pub trait STAKINGTOKEN<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(
        &mut self,
        declaration_hash: Key,
        timing_hash: Key,
        helper_hash: Key,
        globals_hash: Key,
        bep20_hash: Key,
        snapshot_hash: Key,
        referral_token_hash: Key,
        contract_hash: Key,
    ) {
        data::set_hash(contract_hash);
        data::set_declaration_hash(declaration_hash);
        data::set_timing_hash(timing_hash);
        data::set_helper_hash(helper_hash);
        data::set_globals_hash(globals_hash);
        data::set_bep20_hash(bep20_hash);
        data::set_snapshot_hash(snapshot_hash);
        data::set_referral_token_hash(referral_token_hash);
    }
    fn create_stake_bulk(
        &mut self,
        staked_amount: Vec<U256>,
        lock_days: Vec<u64>,
        referrer: Vec<Key>,
    ) {
        for i in 0..staked_amount.len() {
            let (stake_id, start_day, referral_id) =
                self.create_stake(staked_amount[i], lock_days[i], referrer[i]);
        }
    }

    fn create_stake(
        &mut self,
        staked_amount: U256,
        lock_days: u64,
        referrer: Key,
    ) -> (Vec<u32>, U256, Vec<u32>) {

        if self.get_caller() == referrer || Self::_is_contract(referrer) == true {
            runtime::revert(ApiError::UnexpectedKeyVariant);
        }

        let declaration_contract_hash = self.convert_to_contract_hash(data::get_declaration_hash());
        let constants: Vec<u8> = runtime::call_contract(
            declaration_contract_hash,
            "get_declaration_constants",
            runtime_args! {},
        );
        let constants: parameters::ConstantParameters = parameters::ConstantParameters::from_bytes(&constants).unwrap().0;

        if lock_days < constants.min_lock_days.into() || lock_days > constants.max_lock_days.into() {
            runtime::revert(ApiError::UnexpectedKeyVariant);
        }

        if (staked_amount < constants.min_stake_amount) {
            runtime::revert(ApiError::UnexpectedKeyVariant);
        } 
        
        let helper_contract_hash = self.convert_to_contract_hash(data::get_helper_hash());
        let (new_stake, stake_id, start_day) = self._create_stake(self.get_caller(), staked_amount, lock_days, referrer);
        
        if new_stake.referrer_shares > 0.into() {
            
            let referrer_link = structs::ReferrerLink {
                staker: self.get_caller(), 
                stake_id: stake_id.clone(), 
                reward_amount: 0.into(), 
                processed_days: 0.into(), 
                is_active: true,
            };

            let referral_id: Vec<u32> = runtime::call_contract(
                helper_contract_hash,
                "generate_referral_id",
                runtime_args! {"referrer" => referrer},
            );
            let struct_key: String = Self::_generate_key_for_dictionary(&referrer, &referral_id); 
            //generate key and pass it to Declaration contract which will return the struct stored against this key in that contract
            let () = runtime::call_contract(
                declaration_contract_hash,
                "set_struct_from_key",
                runtime_args! {"key" => struct_key,"value"=>referrer_link.clone().into_bytes().unwrap(), "struct_name" => structs::REFERRER_LINK}, // convert structure to json string and save
            );

            let () = runtime::call_contract(
                helper_contract_hash,
                "increase_referral_count",
                runtime_args! {"referrer" => referrer},
            );
            let referral_token_contract_hash = self.convert_to_contract_hash(data::get_referral_token_hash());
            let () = runtime::call_contract(
                referral_token_contract_hash,
                "add_referrer_shares_to_end",
                runtime_args! {"final_day" => new_stake.final_day,"shares" => new_stake.referrer_shares},
            );
        }

        let struct_key0: String = Self::_generate_key_for_dictionary(&self.get_caller(), &stake_id); // generate key and pass it to Declaration contract which will return the struct stored against this key in that contract
        let () = runtime::call_contract(
            declaration_contract_hash,
            "set_struct_from_key",
            runtime_args! {"key" => struct_key0,"value"=>new_stake.clone().into_bytes().unwrap(), "struct_name" => structs::STAKES}, // convert structure to json string and save
        );
        
        let () = runtime::call_contract(
            helper_contract_hash,
            "increase_stake_count",
            runtime_args! {"staker" => self.get_caller()},
        );

        let helper_contract_hash = self.convert_to_contract_hash(data::get_helper_hash());
        let referral_id: Vec<u32> = runtime::call_contract(
            helper_contract_hash,
            "generate_referral_id",
            runtime_args! {"referrer" => referrer},
        );

        let global_contract_hash = self.convert_to_contract_hash(data::get_globals_hash());
        let () = runtime::call_contract(
            global_contract_hash,
            "increase_globals",
            runtime_args! {"staked" => new_stake.staked_amount,"shared" => new_stake.stakes_shares,"r_shareds" => new_stake.referrer_shares},
        );
        //self.add_scheduled_shared(new_stake.final_day, new_stake.stakes_shares)

        (stake_id, U256::from(start_day), referral_id)
    }

    fn _create_stake(
        &mut self,
        staker: Key,
        staked_amount: U256,
        lock_days: u64,
        referrer: Key,
    ) -> (structs::Stake, Vec<u32>, u64) {

        // Get constants from the Declaration contracts, will be used later in this function
        let declaration_contract_hash = self.convert_to_contract_hash(data::get_declaration_hash());
        let constants: Vec<u8> = runtime::call_contract(
            declaration_contract_hash,
            "get_declaration_constants",
            runtime_args! {},
        );
        let constants: parameters::ConstantParameters = parameters::ConstantParameters::from_bytes(&constants).unwrap().0;


        // _burn function
        let bep20_contract_hash = self.convert_to_contract_hash(data::get_bep20_hash());
        let () = runtime::call_contract(
            bep20_contract_hash,
            "_burn",
            runtime_args! {"account" => staker,"amount"=>staked_amount},
        );
        let timing_contract_hash = self.convert_to_contract_hash(data::get_timing_hash());
        let _start_day: u64 = runtime::call_contract(timing_contract_hash, "_next_wise_day", runtime_args! {});
        let helper_contract_hash = self.convert_to_contract_hash(data::get_helper_hash());

        let stake_id: Vec<u32> = runtime::call_contract(
            helper_contract_hash,
            "generate_stake_id",
            runtime_args! {"staker" => staker},
        );

        //get share_price value from shares
        let global = self.convert_to_contract_hash(data::get_globals_hash());
        let share_price: U256 = runtime::call_contract(global, "get_globals", runtime_args!{"field" => String::from("share_price")});

        let mut new_stake = structs::Stake{
            stakes_shares: self._stakes_shares(staked_amount, U256::from(lock_days), referrer, share_price),
            staked_amount: staked_amount,
            reward_amount: 0.into(),
            start_day: _start_day,
            lock_days: lock_days,
            final_day: _start_day + lock_days,
            close_day: 0,
            scrape_day: 0.into(),
            dai_equivalent: 0.into(),
            referrer_shares: 0.into(),
            referrer: Key::Hash([0u8; 32]),
            is_active: false
        };

        let referral_token_contract_hash = self.convert_to_contract_hash(data::get_referral_token_hash());
        let busd_equivalent: U256 = runtime::call_contract(
            referral_token_contract_hash,
            "get_busd_equivalent",
            runtime_args! {},
        );

        new_stake.dai_equivalent = (busd_equivalent.checked_mul(new_stake.staked_amount).unwrap_or_revert()).checked_div(constants.yodas_per_wise).unwrap_or_revert();

        if (self._non_zero_address(referrer)) {
            new_stake.referrer = referrer;
            let () = runtime::call_contract(
                referral_token_contract_hash,
                "add_critical_mass",
                runtime_args! {"referrer"=>new_stake.referrer,"dai_equivalent"=>new_stake.dai_equivalent},
            );

            new_stake.referrer_shares = self._referrer_shares(staked_amount, U256::from(lock_days), referrer);
        }
        return (new_stake, stake_id, _start_day);
    }

    fn end_stake(&mut self, _stake_id: Vec<u32>) -> U256
    {
        let globals_hash = self.convert_to_contract_hash(data::get_globals_hash());
        let declaration_hash = self.convert_to_contract_hash(data::get_declaration_hash());
        let referral_token_hash = self.convert_to_contract_hash(data::get_referral_token_hash());

        let (ended_stake, penalty_amount): (structs::Stake, U256) = self._end_stake(self.get_caller(), _stake_id.clone());
 
        //decrease_globals
        let _ : () = runtime::call_contract(
                globals_hash, 
                "decrease_globals", 
                runtime_args! {"_staked"=>ended_stake.staked_amount,"_shares"=>ended_stake.stakes_shares,"_rshares"=>ended_stake.referrer_shares});
        
        // removed_scheduled_shares
        self._remove_scheduled_shares(U256::from(ended_stake.final_day), ended_stake.stakes_shares);
        
        // remove_referrer_shares_to_end
        let _:() = runtime::call_contract(
                referral_token_hash,
                "remove_referrer_shares_to_end",
                runtime_args! {"final_day" => ended_stake.final_day, "shares" => ended_stake.referrer_shares},
        );

        // remove_critical_mass
        let _:() = runtime::call_contract(
                referral_token_hash,
                "remove_critical_mass",
                runtime_args! {"referrer" => ended_stake.referrer, "dai_equivalent" => ended_stake.dai_equivalent, "start_day" => ended_stake.start_day},
        );

        // _store_penalty
        self._store_penalty(ended_stake.close_day, penalty_amount);

        // _share_price_update
        let _staked_amount = if (ended_stake.staked_amount > penalty_amount) { ended_stake.staked_amount.checked_sub(penalty_amount).unwrap_or_revert()} else {0.into()};
        
        let scrape_key: String = Self::_generate_key_for_dictionary(&self.get_caller(), &_stake_id);
        let _scrapes: U256 = runtime::call_contract(declaration_hash, "get_scrapes", runtime_args!{"key" => scrape_key});

        self._share_price_update(
            _staked_amount,
            ended_stake.reward_amount.checked_add(_scrapes).unwrap_or_revert(),
            ended_stake.referrer,
            U256::from(ended_stake.lock_days),
            ended_stake.stakes_shares
        );

        /*
        emit StakeEnd(
            _stakeID,
            msg.sender,
            endedStake.referrer,
            endedStake.stakedAmount,
            endedStake.stakesShares,
            endedStake.referrerShares,
            endedStake.rewardAmount,
            endedStake.closeDay,
            penaltyAmount
        );
        */

        return ended_stake.reward_amount;
    }

    fn _end_stake(&mut self, _staker: Key,  _stake_id: Vec<u32>) -> (structs::Stake, U256)
    {
        let declaration_hash = self.convert_to_contract_hash(data::get_declaration_hash());
        let timing_hash = self.convert_to_contract_hash(data::get_timing_hash());
        let bep_20_hash = self.convert_to_contract_hash(data::get_bep20_hash());

        let key: String = Self::_generate_key_for_dictionary(&_staker, &_stake_id);
        let stake_string: Vec<u8> = runtime::call_contract(declaration_hash, "get_struct_from_key", runtime_args!{"key" => key, "struct_name" => String::from(structs::STAKES)});
        let mut stake: structs::Stake = structs::Stake::from_bytes(&stake_string).unwrap().0;

        if stake.is_active == false {
            runtime::revert(ApiError::InvalidArgument)
        }
        let current_wise_day: u64 = runtime::call_contract(timing_hash, "_current_wise_day", runtime_args!{});
        stake.close_day = current_wise_day;
        stake.reward_amount = self._calculate_reward_amount(&stake);
        let penalty: U256 = self._calculate_penalty_amount(&stake);
        stake.is_active = false;

        // mint
        let _ : () = runtime::call_contract(bep_20_hash, "_mint", runtime_args!{});                 // need to fill the parameters
        let _ : () = runtime::call_contract(bep_20_hash, "_mint", runtime_args!{});                 // need to fill the parameters

        (stake, penalty)
    }

    fn scrape_interest(&mut self, _stake_id: Vec<u32>, _scrape_days: u64) -> (U256, U256, U256, U256, U256)
    {
        let declaration_hash = self.convert_to_contract_hash(data::get_declaration_hash());
        let helper_hash = self.convert_to_contract_hash(data::get_helper_hash());

        let globals_hash = self.convert_to_contract_hash(data::get_globals_hash());
        let referral_token_hash = self.convert_to_contract_hash(data::get_referral_token_hash());
        let bep_20_hash = self.convert_to_contract_hash(data::get_bep20_hash());

        let key: String = Self::_generate_key_for_dictionary(&self.get_caller(), &_stake_id);
        let stake_str: Vec<u8> = runtime::call_contract(declaration_hash, "get_struct_from_key", runtime_args!{"key" => key, "struct_name" => String::from(structs::STAKES)});
        let mut stake: structs::Stake = structs::Stake::from_bytes(&stake_str).unwrap().0;

        if stake.is_active == false {
            runtime::revert(ApiError::InvalidArgument)
        }

        let starting_day: U256 = runtime::call_contract(helper_hash, "starting_day", runtime_args!{"stake" => stake_str.clone()});
        let calculation_day: U256 = runtime::call_contract(helper_hash, "calculation_day", runtime_args!{"stake" => stake_str.clone()});


        let mut scrape_day: U256 = if _scrape_days > 0 {
            starting_day.checked_add(U256::from(_scrape_days)).unwrap_or_revert()
        } 
        else {
            calculation_day
        };

        scrape_day = if scrape_day > stake.final_day.into() { calculation_day } else { scrape_day };
        let scrape_amount: U256 = self._loop_reward_amount(stake.stakes_shares, starting_day.clone(), scrape_day.clone());

        let mut referrer_penalty: U256 = 0.into();
        let mut remaining_days: U256 = 0.into();
        let mut stakers_penalty: U256 = 0.into();
        
        let is_mature_stake: bool = runtime::call_contract(helper_hash, "is_mature_stake", runtime_args!{"stake" => stake_str.clone()});
        if is_mature_stake == false 
        {
            remaining_days = runtime::call_contract(helper_hash, "days_left", runtime_args!{"stake" => stake_str.clone() });
            
            let share_price: U256 = runtime::call_contract(globals_hash, "get_globals", runtime_args!{"field" => String::from(structs::SHARE_PRICE)});
            stakers_penalty = self._stakes_shares(scrape_amount, remaining_days, self.get_caller(), share_price.clone());
            stake.stakes_shares = stake.stakes_shares.checked_sub(stakers_penalty).unwrap_or_revert();

            self._remove_scheduled_shares(U256::from(stake.final_day), stakers_penalty);

            if stake.referrer_shares > 0.into() {
                let zero_addr: Key = Key::Hash([0u8; 32]);
                referrer_penalty =  self._stakes_shares(scrape_amount, remaining_days, zero_addr, share_price);

                stake.referrer_shares = stake.referrer_shares.checked_sub(referrer_penalty).unwrap_or_revert();

                // remove_referrer_shares_to_end
                let _:() = runtime::call_contract(
                    referral_token_hash,
                    "remove_referrer_shares_to_end",
                    runtime_args! {"final_day" => stake.final_day, "shares" => referrer_penalty},
                );
            }
            
            //decrease_globals
            let _ : () = runtime::call_contract(
                globals_hash, 
                "decrease_globals", 
                runtime_args! {"_staked"=>U256::from(0),"_shares"=>stakers_penalty,"_rshares"=>referrer_penalty}
            );
            self._share_price_update(stake.staked_amount, scrape_amount, stake.referrer, U256::from(stake.lock_days), stake.stakes_shares);
        }
        else
        {
            let scrape_key: String = Self::_generate_key_for_dictionary(&self.get_caller(), &_stake_id);
            let mut _scrapes: U256 = runtime::call_contract(declaration_hash, "get_scrapes", runtime_args!{"key" => scrape_key.clone()});
            _scrapes = _scrapes.checked_add(scrape_amount).unwrap_or_revert();

            let _: () = runtime::call_contract(declaration_hash, "set_scrapes", runtime_args!{"key" => scrape_key.clone(), "value" => _scrapes});
        
            self._share_price_update(stake.staked_amount, _scrapes, stake.referrer, U256::from(stake.lock_days), stake.stakes_shares);
        }

        stake.scrape_day = scrape_day;

        let key: String = Self::_generate_key_for_dictionary(&self.get_caller(), &_stake_id);
        let mut stake_str: Vec<u8> = stake.clone().into_bytes().unwrap();
        let _: () = runtime::call_contract(declaration_hash, "set_struct_from_key", runtime_args!{"key" => key, "value" => stake_str, "struct_name" => String::from(structs::STAKES)});

        let _ : () = runtime::call_contract(bep_20_hash, "_mint", runtime_args!{});         // need to fill the parameters

        /*
            emit InterestScraped(
            _stakeID,
            msg.sender,
            scrapeAmount,
            scrapeDay,
            stakersPenalty,
            referrerPenalty,
            _currentWiseDay()
        );
        */

        (scrape_day, scrape_amount, remaining_days, stakers_penalty, referrer_penalty)
    }

    fn _add_scheduled_shares(&self, _final_day: U256, _shares: U256)
    {
        let declaration_hash = self.convert_to_contract_hash(data::get_declaration_hash());
        let mut _scheduled_to_end: U256 = runtime::call_contract(declaration_hash, "get_scheduled_to_end", runtime_args!{"key" => _final_day});
        _scheduled_to_end = _scheduled_to_end.checked_add(_shares).unwrap_or_revert();
        let _ : () = runtime::call_contract(declaration_hash, "set_scheduled_to_end", runtime_args!{"key" => _final_day, "value" => _scheduled_to_end});
    }

    fn _remove_scheduled_shares(&self, _final_day: U256, _shares: U256)
    {
        let helper_hash = self.convert_to_contract_hash(data::get_helper_hash());
        let declaration_hash = self.convert_to_contract_hash(data::get_declaration_hash());
        let timing_hash = self.convert_to_contract_hash(data::get_timing_hash());
        let snapshot_hash = self.convert_to_contract_hash(data::get_snapshot_hash());

        let _not_past: bool = runtime::call_contract(helper_hash, "not_past", runtime_args!{"day" => _final_day});
        let _scheduled_to_end: U256 = runtime::call_contract(declaration_hash, "get_scheduled_to_end", runtime_args!{"key" => _final_day});

        if _not_past {
            let updated_scheduled_to_end: U256 = if _scheduled_to_end > _shares {_scheduled_to_end.checked_sub(_shares).unwrap_or_revert()} else {0.into()};
            let _:() = runtime::call_contract(declaration_hash, "set_scheduled_to_end", runtime_args!{"key" => _final_day, "value" => updated_scheduled_to_end});
        }
        else {
            let day: u64 = runtime::call_contract(timing_hash, "_previous_wise_day", runtime_args!{});
            let day: U256 = U256::from(day);
            let snapshot_str: Vec<u8> = runtime::call_contract(snapshot_hash, "get_struct_from_key", runtime_args!{"key" => day, "struct_name" => String::from("snapshots_dicts")});
            let mut snapshot_struct: structs::Snapshot = structs::Snapshot::from_bytes(&snapshot_str).unwrap().0;

            snapshot_struct.scheduled_to_end = if snapshot_struct.scheduled_to_end >_shares {snapshot_struct.scheduled_to_end.checked_sub(_shares).unwrap_or_revert()} else {0.into()};
            
            let snapshot_str: Vec<u8> = snapshot_struct.clone().into_bytes().unwrap();
            let _:() = runtime::call_contract(snapshot_hash, "set_struct_from_key", runtime_args!{"key" => day, "struct_name" => String::from("snapshots_dicts"), "value" => snapshot_str.clone()});
        }
    }

    fn _share_price_update(&mut self,
        _staked_amount: U256,
        _reward_amount: U256,
        _referrer: Key,
        _lock_days: U256,
        _stake_shares: U256
    )
    {
        let globals_hash = self.convert_to_contract_hash(data::get_timing_hash());
        let declaration_hash = self.convert_to_contract_hash(data::get_timing_hash());
        let timing_hash = self.convert_to_contract_hash(data::get_timing_hash());
        let mut current_wise_day: u64 = runtime::call_contract(timing_hash, "_current_wise_day", runtime_args!{});
        let declaration_constants_string : Vec<u8> = runtime::call_contract(declaration_hash, "get_declaration_constants", runtime_args!{});
        let declaration_constants_struct : parameters::ConstantParameters = parameters::ConstantParameters::from_bytes(&declaration_constants_string).unwrap().0;
        let mut globals_share_price: U256 = runtime::call_contract(globals_hash, "get_globals", runtime_args!{"field"=>structs::SHARE_PRICE});
       
        if _stake_shares > 0.into() && current_wise_day > (declaration_constants_struct.formula_day).into(){
            let mut new_share_price: U256 = self._get_new_share_price(_staked_amount, _reward_amount, _stake_shares, _lock_days, _referrer);
        
            if new_share_price > globals_share_price {

            if new_share_price >= (globals_share_price*U256::from(110) / U256::from(100)){
                    new_share_price = globals_share_price*U256::from(110) / U256::from(100);
                }

                // TODO emit NewSharePrice event here
                // emit NewSharePrice(
                //     newSharePrice,
                //     globals.sharePrice,
                //     _currentWiseDay()
                // );
                
                let () =runtime::call_contract(globals_hash, "set_globals", runtime_args!{
                    "field"=>structs::SHARE_PRICE,
                    "value"=>new_share_price
                });
            }
            return;
        }

        current_wise_day = runtime::call_contract(timing_hash, "_current_wise_day", runtime_args!{});
        if current_wise_day == (declaration_constants_struct.formula_day as u64) {
            let () = runtime::call_contract(globals_hash, "set_globals", runtime_args!{
                "field"=>structs::SHARE_PRICE,
                "value"=>U256::from(110).checked_pow(U256::from(15)).unwrap_or_revert()
            });
        }
    }

    fn _get_new_share_price(&mut self,
        _staked_amount: U256,
        _reward_amount: U256,
        _stake_shares: U256,
        _lock_days: U256,
        _referrer: Key
    )->U256
    {
        let base10: U256 = U256::from(10).checked_pow(U256::from(9)).unwrap_or_revert();
        let base11: U256 = U256::from(11).checked_pow(U256::from(9)).unwrap_or_revert();
        let base1: U256 = U256::from(1).checked_pow(U256::from(8)).unwrap_or_revert();

        let _bonus_amount = self._get_bonus(_lock_days, if self._non_zero_address(_referrer){
            base11
        }else{
            base10
        });

        ((_staked_amount + _reward_amount) * _bonus_amount * base1) / _stake_shares
    }

    fn check_mature_stake(&mut self,
        _staker: Key,
        _stake_id: Vec<u32>
    )->bool
    {
        let helper_hash = self.convert_to_contract_hash(data::get_helper_hash());
        let declaration_hash = self.convert_to_contract_hash(data::get_declaration_hash());
        let stake_string: Vec<u8> =  runtime::call_contract(declaration_hash, "get_struct_from_key", runtime_args!{
            "struct_name"=>"stakes",
            "key"=>Self::_generate_key_for_dictionary(&_staker, &_stake_id)
        });
        let is_mature : bool = runtime::call_contract(helper_hash, "is_mature_stake", runtime_args!{
            "stake"=>stake_string
        });

        is_mature
    }

    fn check_stake_by_id(&mut self,
        _staker: Key,
        _stake_id: Vec<u32>
    )->(Vec<u8>, U256, bool)
    {
        let declaration_hash = self.convert_to_contract_hash(data::get_declaration_hash());
        let helper_hash = self.convert_to_contract_hash(data::get_helper_hash());
        let mut stake: Vec<u8> = runtime::call_contract(declaration_hash, "get_struct_from_key", runtime_args!{
            "struct_name"=>"stakes",
            "key"=>Self::_generate_key_for_dictionary(&_staker, &_stake_id)
        });
        let is_mature : bool = runtime::call_contract(helper_hash, "is_mature_stake", runtime_args!{
            "stake"=>stake.clone()
        });

        let mut stake_struct:structs::Stake = structs::Stake::from_bytes(&stake).unwrap().0;
        stake_struct.reward_amount = self._check_reward_amount(&stake_struct);
        let penalty_amount: U256 = self._calculate_penalty_amount(&stake_struct);
        
        stake = stake_struct.clone().into_bytes().unwrap();

        (stake, penalty_amount, is_mature)
    }

    fn _stakes_shares(&mut self,
        _staked_amount:U256,
        _lock_days:U256,
        _referrer: Key,
        _share_price:U256
    )->U256
    {
        let constant: U256 = U256::from(10).checked_pow(U256::from(9)).unwrap_or_revert();
        if self._non_zero_address(_referrer){
            self._shares_amount(_staked_amount, _lock_days, _share_price, constant)
        }else{
            self._shares_amount(_staked_amount, _lock_days, _share_price, constant)
        }
    }

    fn _shares_amount(&mut self,
        _staked_amount: U256,
        _lock_days: U256,
        _share_price: U256,
        _extra_bonus: U256
    )->U256
    {
        let bonus: U256 = self._get_bonus(_lock_days, _extra_bonus);
        let base_amount = self._base_amount(_staked_amount, _share_price);
        let constant: U256 = U256::from(10).checked_pow(U256::from(9)).unwrap_or_revert();

        base_amount.checked_mul(bonus).unwrap_or_revert().checked_div(constant).unwrap_or_revert()
    }

    fn _get_bonus(&mut self,
        _lock_days: U256,
        _extra_bonus: U256
    )-> U256
    {
        let declaration_hash = self.convert_to_contract_hash(data::get_declaration_hash());
        let constants_string: Vec<u8> = runtime::call_contract(declaration_hash, "get_declaration_constants", runtime_args!{});
        let constants_struct: parameters::ConstantParameters = parameters::ConstantParameters::from_bytes(&constants_string).unwrap().0;
        
        self._regular_bonus(_lock_days, constants_struct.daily_bonus_a, U256::from(constants_struct.max_bonus_days_a)) + if _lock_days>U256::from(constants_struct.max_bonus_days_a){ 
            self._regular_bonus(_lock_days - constants_struct.max_bonus_days_a, U256::from(0), U256::from(0))
        }else{
            self._regular_bonus(U256::from(0), constants_struct.daily_bonus_b, U256::from(constants_struct.max_bonus_days_b))
        } + _extra_bonus
    }

    fn _regular_bonus(&self,
        _lock_days: U256,
        _daily: U256,
        _max_days: U256
    ) ->U256
    {
        let ret: U256 = if _lock_days>_max_days{
            _max_days * _daily
        }
        else{
            _lock_days * _daily
        };
        ret.checked_div(U256::from(10).checked_pow(U256::from(9)).unwrap()).unwrap()
    }

    fn _base_amount(&mut self,
        _staked_amount: U256,
        _share_price: U256
    )->U256
    {
        let declaration_hash: Key = data::get_declaration_hash();
        let parameters_string: Vec<u8> = runtime::call_contract(self.convert_to_contract_hash(declaration_hash), "get_declaration_constants", runtime_args!{});
        let parameters_struct: parameters::ConstantParameters = parameters::ConstantParameters::from_bytes(&parameters_string).unwrap().0;
       
        _staked_amount.checked_mul(U256::from(parameters_struct.precision_rate)).unwrap().checked_div(_share_price).unwrap()
    }

    fn _referrer_shares(&mut self,
        _staked_amount: U256,
        _lock_days: U256,
        _referrer: Key
    )->U256
    {
        let helper_hash = data::get_helper_hash();
        let globals_hash = data::get_globals_hash();
        let declaration_hash: Key = data::get_declaration_hash();
       
        let parameters_string: Vec<u8> = runtime::call_contract(self.convert_to_contract_hash(declaration_hash), "get_declaration_constants", runtime_args!{});
        let parameters_struct: parameters::ConstantParameters = parameters::ConstantParameters::from_bytes(&parameters_string).unwrap().0;
        let share_price: U256 = runtime::call_contract(self.convert_to_contract_hash(globals_hash), "get_globals", runtime_args!{
            "field"=>"share_price"
        });
        let critical_mass_referrer: bool = runtime::call_contract(self.convert_to_contract_hash(helper_hash), "not_critical_mass_referrer", runtime_args!{
            "referrer"=>_referrer
        });
        let constant: U256 = U256::from(10).checked_pow(U256::from(9)).unwrap_or_revert();

        if critical_mass_referrer || _lock_days < parameters_struct.min_referral_days.into(){
            U256::from(0)
        }else{
            self._shares_amount(_staked_amount, _lock_days, share_price, constant)
        } 
    }

    

    fn _check_reward_amount(&self, _stake: &structs::Stake) ->U256 {
        if _stake.is_active {
            self._detect_reward(&_stake)
        }else{
            _stake.reward_amount
        }
    }

    fn _detect_reward(&self, _stake: &structs::Stake) ->U256 {
        let helper_hash = data::get_helper_hash();

        let stake_status : bool = runtime::call_contract(self.convert_to_contract_hash(helper_hash), "stake_not_started", runtime_args!{
            "stake"=>_stake.clone().into_bytes().unwrap()
        });
        
        if stake_status{U256::from(0)} else {self._calculate_reward_amount(_stake)} 
    }

    fn _store_penalty(&mut self,
        _store_day: u64,
        _penalty: U256
    )
    {
        if _penalty > 0.into() {
            let declaration_hash = self.convert_to_contract_hash(data::get_declaration_hash());
            let mut total_penalty : U256 = runtime::call_contract(declaration_hash, "get_struct_from_key", runtime_args!{
                "struct_name"=>"total_penalties",
                "key"=>U256::from(_store_day)
            });

            total_penalty = total_penalty+_penalty;

            let () = runtime::call_contract(declaration_hash, "set_struct_from_key", runtime_args!{
                "struct_name"=>"total_penalties",
                "key"=>U256::from(_store_day),
                "value"=>total_penalty
            });
        }
    }

    fn _calculate_penalty_amount(&mut self,
        _stake: &structs::Stake
   )->U256
   {
       let helper_hash = data::get_helper_hash();
       let stake_string: Vec<u8> = _stake.clone().into_bytes().unwrap();
       let stake_status : bool = runtime::call_contract(self.convert_to_contract_hash(helper_hash), "stake_not_started", runtime_args!{
           "stake"=>stake_string.clone()
       });
       let stake_maturity : bool = runtime::call_contract(self.convert_to_contract_hash(helper_hash), "is_stake_mature", runtime_args!{
           "stake"=>stake_string.clone()
        });

       if stake_status || stake_maturity{
           U256::from(0)
       }else {
           self._get_penalties(&_stake)
       }
   }

   fn _get_penalties(&mut self, _stake: &structs::Stake)->U256
   {
       let stake_string: Vec<u8> = _stake.clone().into_bytes().unwrap();
       let helper_hash = self.convert_to_contract_hash(data::get_helper_hash());
       let days_left: U256= runtime::call_contract(helper_hash, "days_left", runtime_args!{"stake"=>stake_string.clone()});
       let locked_days: U256= runtime::call_contract(helper_hash, "get_locked_days", runtime_args!{"stake"=>stake_string.clone()});

       _stake.staked_amount * (U256::from(100) + (U256::from(800) * (days_left - U256::from(1)) / locked_days)) / U256::from(1000)
   }

    fn _calculate_reward_amount(&self,
        _stake: &structs::Stake
    )->U256
    {
        let starting_day: U256= runtime::call_contract(self.convert_to_contract_hash(data::get_helper_hash()), "starting_day", runtime_args!{
            "stake"=>_stake.clone().into_bytes().unwrap()
        });
        let calculation_day: U256 = runtime::call_contract(self.convert_to_contract_hash(data::get_helper_hash()), "calculation_day", runtime_args!{
            "stake"=>_stake.clone().into_bytes().unwrap()
        });

        self._loop_reward_amount(
            _stake.stakes_shares,
           starting_day,
           calculation_day
        )
    }

    fn _loop_reward_amount(&self,
        _stake_shares: U256,
        _start_day: U256,
        _final_day: U256
    )->U256
    {
        let constants_string: Vec<u8> = runtime::call_contract(self.convert_to_contract_hash(data::get_declaration_hash()), "get_declaration_constants", runtime_args!{});
        let constants_struct: parameters::ConstantParameters = parameters::ConstantParameters::from_bytes(&constants_string).unwrap().0;
        let mut reward_amount: U256=U256::from(0);
        let mut res: U256=U256::from(0);
        
        let snapshot_hash = data::get_snapshot_hash();
        for _day in _start_day.as_u64().._final_day.as_u64(){
            // get snapshot struct and convert to struct type
            let snapshot_str: Vec<u8> = runtime::call_contract(self.convert_to_contract_hash(snapshot_hash), "get_struct_from_key", runtime_args!{
                "struct_name" =>"snapshot",
                "key"=>_day
            });
            let snapshot_struct: structs::Snapshot = structs::Snapshot::from_bytes(&snapshot_str).unwrap().0;


            // calc stuff
            res = (_stake_shares.checked_mul(constants_struct.precision_rate)).unwrap_or_revert().checked_div(snapshot_struct.inflation_amount).unwrap_or_revert();
            // add to reward_amount
            reward_amount= reward_amount+res;
        }
        
        reward_amount
    }

    fn _generate_key_for_dictionary(key: &Key, id: &Vec<u32>) -> String
    {
        let mut result: String = String::from("");
        result.push_str(&key.to_formatted_string());
        result.push_str("-");
        result.push_str(&Self::_convert_vec_to_string(id));

        result
    }
    fn _convert_vec_to_string(data: &Vec<u32>) -> String
    {
        let mut result: String = String::from("");
        for value in data {
            result.push_str(&value.to_string());
        }
        result
    }

    // fn _generate_key_for_dictionary(key: &Key, id: &u32) -> String {
    //     let mut result: String = String::from("");
    //     result.push_str(&key.to_formatted_string());
    //     result.push_str("-");
    //     result.push_str(&id.to_string());

    //     result
    // }
    fn convert_to_contract_hash(&self, contract_hash: Key) -> ContractHash {
        let contract_hash_add_array = match contract_hash {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        return ContractHash::new(contract_hash_add_array);
    }
    fn _non_zero_address(&self, key: Key) -> bool {
        let zero_addr: Key = Key::Hash([0u8; 32]);
        return key != zero_addr;
    }
    fn _is_contract(address: Key) -> bool{                  // need to work on it
        true
    }
}