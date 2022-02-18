use alloc::string::ToString;
use alloc::{string::String, vec::Vec};

use erc20_crate::ERC20;

use declaration_crate::Declaration;
use globals_crate::Globals;
use helper_crate::Helper;
use referral_token_crate::ReferralToken;
use snapshot_crate::Snapshot;
use timing_crate::Timing;

use wise_token_utils::{commons::key_names::*, declaration, error_codes, events::*, snapshot};

use casper_contract::contract_api::runtime;
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    ApiError, Key, U256,
};
use contract_utils::{ContractContext, ContractStorage};

pub trait StakingToken<Storage: ContractStorage>:
    ContractContext<Storage>
    + Declaration<Storage>
    + Globals<Storage>
    + Timing<Storage>
    + Helper<Storage>
    + Snapshot<Storage>
    + ReferralToken<Storage>
{
    fn init(&self) {}

    fn create_stake_bulk(&self, staked_amount: Vec<U256>, lock_days: Vec<u64>, referrer: Vec<Key>) {
        for i in 0..staked_amount.len() {
            let _ = self.create_stake(staked_amount[i], lock_days[i], referrer[i]);
        }
    }

    fn create_stake(
        &self,
        staked_amount: U256,
        lock_days: u64,
        referrer: Key,
    ) -> (Vec<u32>, U256, Vec<u32>) {
        if self.get_caller() == referrer || Self::_is_contract(referrer) == true {
            runtime::revert(ApiError::UnexpectedKeyVariant);
        }

        let constants: Vec<u8> = Declaration::get_declaration_constants(self);
        let constants: declaration::parameters::ConstantParameters =
            declaration::parameters::ConstantParameters::from_bytes(&constants)
                .unwrap()
                .0;

        if lock_days < constants.min_lock_days.into() || lock_days > constants.max_lock_days.into()
        {
            runtime::revert(ApiError::UnexpectedKeyVariant);
        }

        if staked_amount < constants.min_stake_amount {
            runtime::revert(ApiError::UnexpectedKeyVariant);
        }
        let (new_stake, stake_id, start_day) =
            self._create_stake(self.get_caller(), staked_amount, lock_days, referrer);
        let mut referral_id: Vec<u32> = Vec::new();
        if new_stake.referrer_shares > 0.into() {
            let referrer_link = declaration::structs::ReferrerLink {
                staker: self.get_caller(),
                stake_id: stake_id.clone(),
                reward_amount: 0.into(),
                processed_days: 0.into(),
                is_active: true,
            };

            referral_id = Helper::generate_referral_id(self, referrer);
            let struct_key: String =
                Declaration::_generate_key_for_dictionary(self, &referrer, &referral_id);

            //generate key and pass it to Declaration contract which will return the struct stored against this key in that contract
            Declaration::set_struct_from_key(
                self,
                struct_key,
                referrer_link.clone().into_bytes().unwrap(),
                DECLARATION_REFERRER_LINK_DICT.to_string(),
            );
            Helper::_increase_referral_count(self, referrer);
            ReferralToken::_add_referrer_shares_to_end(
                self,
                U256::from(new_stake.final_day),
                U256::from(new_stake.referrer_shares),
            );
        }

        let struct_key0: String =
            Declaration::_generate_key_for_dictionary(self, &self.get_caller(), &stake_id); // generate key and pass it to Declaration contract which will return the struct stored against this key in that contract
        Declaration::set_struct_from_key(
            self,
            struct_key0,
            new_stake.clone().into_bytes().unwrap(),
            DECLARATION_STAKES_DICT.to_string(),
        );
        Helper::_increase_stake_count(self, self.get_caller());
        Globals::increase_globals(
            self,
            new_stake.staked_amount,
            new_stake.stakes_shares,
            new_stake.referrer_shares,
        );
        Self::_add_scheduled_shares(
            self,
            U256::from(new_stake.final_day),
            new_stake.stakes_shares,
        );

        (stake_id, U256::from(start_day), referral_id)
    }

    fn _create_stake(
        &self,
        staker: Key,
        staked_amount: U256,
        lock_days: u64,
        referrer: Key,
    ) -> (declaration::structs::Stake, Vec<u32>, u64) {
        // Get constants from the Declaration contracts, will be used later in this function
        let constants: Vec<u8> = Declaration::get_declaration_constants(self);
        let constants: declaration::parameters::ConstantParameters =
            declaration::parameters::ConstantParameters::from_bytes(&constants)
                .unwrap()
                .0;

        let _: () = ERC20::burn(self, staker, staked_amount);
        let _start_day: u64 = Timing::_next_wise_day(self);
        let stake_id: Vec<u32> = Helper::generate_stake_id(self, staker);

        //get share_price value from shares
        let share_price: U256 = Globals::get_globals(self, GLOBALS_SHARE_PRICE.to_string());
        let mut new_stake = declaration::structs::Stake {
            stakes_shares: self._stakes_shares(
                staked_amount,
                U256::from(lock_days),
                referrer,
                share_price,
            ),

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
            is_active: false,
        };

        // NOTE: for unit testing, hard code stable_usd_equivalent with non zero value
        let stable_usd_equivalent: U256 = ReferralToken::get_stable_usd_equivalent(self);

        new_stake.dai_equivalent = (stable_usd_equivalent
            .checked_mul(new_stake.staked_amount)
            .unwrap_or_revert())
        .checked_div(constants.yodas_per_wise)
        .unwrap_or_revert();

        if Self::_non_zero_address(referrer) {
            new_stake.referrer = referrer;
            ReferralToken::_add_critical_mass(self, new_stake.referrer, new_stake.dai_equivalent);
            new_stake.referrer_shares =
                self._referrer_shares(staked_amount, U256::from(lock_days), referrer);
        }

        emit(&WiseEvents::StakeStart {
            stake_id: stake_id.clone(),
            staker_address: self.get_caller(),
            referral_address: referrer,
            staked_amount: new_stake.staked_amount,
            stakes_shares: new_stake.stakes_shares,
            referral_shares: new_stake.referrer_shares,
            start_day: new_stake.start_day,
            lock_days: new_stake.lock_days,
            dai_equivalent: new_stake.dai_equivalent,
        });

        return (new_stake, stake_id, _start_day);
    }

    fn end_stake(&self, _stake_id: Vec<u32>) -> U256 {
        let (ended_stake, penalty_amount): (declaration::structs::Stake, U256) =
            Self::_end_stake(self, self.get_caller(), _stake_id.clone());

        //decrease_globals
        Globals::decrease_globals(
            self,
            ended_stake.staked_amount,
            ended_stake.stakes_shares,
            ended_stake.referrer_shares,
        );

        // removed_scheduled_shares
        Self::_remove_scheduled_shares(
            self,
            U256::from(ended_stake.final_day),
            ended_stake.stakes_shares,
        );

        // remove_referrer_shares_to_end
        ReferralToken::_remove_referrer_shares_to_end(
            self,
            U256::from(ended_stake.final_day),
            ended_stake.referrer_shares,
        );

        // remove_critical_mass
        ReferralToken::_remove_critical_mass(
            self,
            ended_stake.referrer,
            ended_stake.dai_equivalent,
            U256::from(ended_stake.start_day),
        );

        // _store_penalty
        Self::_store_penalty(self, ended_stake.close_day, penalty_amount);

        // _share_price_update
        let _staked_amount = if ended_stake.staked_amount > penalty_amount {
            ended_stake
                .staked_amount
                .checked_sub(penalty_amount)
                .unwrap_or_revert()
        } else {
            0.into()
        };
        let scrape_key: String =
            Declaration::_generate_key_for_dictionary(self, &self.get_caller(), &_stake_id);
        let _scrapes = Declaration::get_scrapes(self, scrape_key);

        Self::_share_price_update(
            self,
            _staked_amount,
            ended_stake
                .reward_amount
                .checked_add(_scrapes)
                .unwrap_or_revert(),
            ended_stake.referrer,
            U256::from(ended_stake.lock_days),
            ended_stake.stakes_shares,
        );

        emit(&WiseEvents::StakeEnd {
            stake_id: _stake_id,
            staker_address: self.get_caller(),
            referral_address: ended_stake.referrer,
            staked_amount: ended_stake.staked_amount,
            stakes_shares: ended_stake.stakes_shares,
            referral_shares: ended_stake.referrer_shares,
            reward_amount: ended_stake.reward_amount,
            close_day: ended_stake.close_day,
            penalty_amount,
        });
        return ended_stake.reward_amount;
    }

    fn _end_stake(&self, _staker: Key, _stake_id: Vec<u32>) -> (declaration::structs::Stake, U256) {
        let key: String = Declaration::_generate_key_for_dictionary(self, &_staker, &_stake_id);
        let stake_string: Vec<u8> = Declaration::get_struct_from_key(
            self,
            key.clone(),
            String::from(DECLARATION_STAKES_DICT),
        );

        let mut stake: declaration::structs::Stake =
            declaration::structs::Stake::from_bytes(&stake_string.clone())
                .unwrap()
                .0;

        if stake.is_active == false {
            runtime::revert(ApiError::User(
                error_codes::ErrorCodes::StakeInactive as u16,
            ))
        }
        let current_wise_day: u64 = Timing::_current_wise_day(self);
        stake.close_day = current_wise_day;
        stake.reward_amount = self._calculate_reward_amount(&stake);
        let penalty: U256 = self._calculate_penalty_amount(&stake);
        stake.is_active = false;

        let _: () = ERC20::mint(
            self,
            _staker,
            if stake.staked_amount > penalty {
                // stake.staked_amount - penalty
                stake
                    .staked_amount
                    .checked_sub(penalty)
                    .ok_or(ApiError::User(error_codes::ErrorCodes::Underflow as u16))
                    .unwrap_or_revert()
            } else {
                U256::from(0)
            },
        );

        let _: () = ERC20::mint(self, _staker, stake.reward_amount);
        let stake_string: Vec<u8> = stake.clone().into_bytes().unwrap();
        // set ended stake to dictionary
        Declaration::set_struct_from_key(
            self,
            key,
            stake_string,
            DECLARATION_STAKES_DICT.to_string(),
        );
        (stake, penalty)
    }

    fn scrape_interest(
        &self,
        _stake_id: Vec<u32>,
        _scrape_days: u64,
    ) -> (U256, U256, U256, U256, U256) {
        Snapshot::_snapshot_trigger(self);

        let key: String =
            Declaration::_generate_key_for_dictionary(self, &self.get_caller(), &_stake_id);
        let stake_str: Vec<u8> =
            Declaration::get_struct_from_key(self, key, String::from(DECLARATION_STAKES_DICT));

        let mut stake: declaration::structs::Stake =
            declaration::structs::Stake::from_bytes(&stake_str)
                .unwrap()
                .0;

        if stake.is_active == false {
            runtime::revert(ApiError::InvalidArgument)
        }

        let starting_day: U256 = Helper::starting_day(self, stake_str.clone());
        let calculation_day: U256 = Helper::calculation_day(self, stake_str.clone());

        let mut scrape_day: U256 = if _scrape_days > 0 {
            starting_day
                .checked_add(U256::from(_scrape_days))
                .unwrap_or_revert()
        } else {
            calculation_day
        };

        scrape_day = if scrape_day > stake.final_day.into() {
            calculation_day
        } else {
            scrape_day
        };
        let scrape_amount: U256 = self._loop_reward_amount(
            stake.stakes_shares,
            starting_day.clone(),
            scrape_day.clone(),
        );

        let mut referrer_penalty: U256 = 0.into();
        let mut remaining_days: U256 = 0.into();
        let mut stakers_penalty: U256 = 0.into();

        let is_mature_stake: bool = Helper::is_mature_stake(self, stake_str.clone());
        if is_mature_stake == false {
            remaining_days = Helper::days_left(self, stake_str.clone());
            let share_price: U256 = Globals::get_globals(self, String::from(GLOBALS_SHARE_PRICE));

            stakers_penalty = self._stakes_shares(
                scrape_amount,
                remaining_days,
                self.get_caller(),
                share_price.clone(),
            );

            stake.stakes_shares = stake
                .stakes_shares
                .checked_sub(stakers_penalty)
                .unwrap_or_revert();
            self._remove_scheduled_shares(U256::from(stake.final_day), stakers_penalty);

            if stake.referrer_shares > 0.into() {
                let zero_addr: Key = Key::Hash([0u8; 32]);
                referrer_penalty =
                    self._stakes_shares(scrape_amount, remaining_days, zero_addr, share_price);

                stake.referrer_shares = stake
                    .referrer_shares
                    .checked_sub(referrer_penalty)
                    .unwrap_or_revert();

                // remove_referrer_shares_to_end
                ReferralToken::_remove_referrer_shares_to_end(
                    self,
                    U256::from(stake.final_day),
                    referrer_penalty,
                );
            }

            //decrease_globals
            Globals::decrease_globals(self, U256::from(0), stakers_penalty, referrer_penalty);
            self._share_price_update(
                stake.staked_amount,
                scrape_amount,
                stake.referrer,
                U256::from(stake.lock_days),
                stake.stakes_shares,
            );
        } else {
            let scrape_key: String =
                Declaration::_generate_key_for_dictionary(self, &self.get_caller(), &_stake_id);
            let mut _scrapes: U256 = Declaration::get_scrapes(self, scrape_key.clone());
            _scrapes = _scrapes.checked_add(scrape_amount).unwrap_or_revert();

            Declaration::set_scrapes(self, scrape_key.clone(), _scrapes);

            self._share_price_update(
                stake.staked_amount,
                _scrapes,
                stake.referrer,
                U256::from(stake.lock_days),
                stake.stakes_shares,
            );
        }

        stake.scrape_day = scrape_day;

        let key: String =
            Declaration::_generate_key_for_dictionary(self, &self.get_caller(), &_stake_id);
        let stake_str: Vec<u8> = stake.clone().into_bytes().unwrap();
        Declaration::set_struct_from_key(
            self,
            key,
            stake_str,
            String::from(DECLARATION_STAKES_DICT),
        );

        let _: () = ERC20::mint(self, self.get_caller(), scrape_amount);

        emit(&WiseEvents::InterestScraped {
            stake_id: _stake_id,
            staker_address: self.get_caller(),
            scrape_amount,
            scrape_day,
            stakers_penalty,
            referrer_penalty,
            current_wise_day: Timing::_current_wise_day(self),
        });

        (
            scrape_day,
            scrape_amount,
            remaining_days,
            stakers_penalty,
            referrer_penalty,
        )
    }

    fn _add_scheduled_shares(&self, _final_day: U256, _shares: U256) {
        let mut _scheduled_to_end: U256 = Declaration::get_scheduled_to_end(self, _final_day);
        _scheduled_to_end = _scheduled_to_end.checked_add(_shares).unwrap_or_revert();

        Declaration::set_scheduled_to_end(self, _final_day, _scheduled_to_end);
    }

    // fn _add_scheduled_shared(&self, _final_day: U256, _shares: U256) {
    //     let mut _scheduled_to_end: U256 = Declaration::get_scheduled_to_end(self, _final_day);
    //     _scheduled_to_end = _scheduled_to_end.checked_sub(_shares).unwrap_or_revert();

    //     Declaration::set_scheduled_to_end(self, _final_day, _scheduled_to_end);
    // }

    fn _remove_scheduled_shares(&self, _final_day: U256, _shares: U256) {
        let _not_past: bool = Helper::not_past(self, _final_day);
        let _scheduled_to_end: U256 = Declaration::get_scheduled_to_end(self, _final_day);

        if _not_past {
            let updated_scheduled_to_end: U256 = if _scheduled_to_end > _shares {
                _scheduled_to_end
                    .checked_sub(_shares)
                    .ok_or(ApiError::User(error_codes::ErrorCodes::Underflow as u16))
                    .unwrap_or_revert()
            } else {
                0.into()
            };

            Declaration::set_scheduled_to_end(self, _final_day, updated_scheduled_to_end);
        } else {
            let day: u64 = Timing::_previous_wise_day(self);
            let day: U256 = U256::from(day);
            let snapshot_str: Vec<u8> =
                Snapshot::get_struct_from_key(self, &day, String::from(SNAPSHOT_SNAPSHOTS_DICT));

            let mut snapshot_struct: snapshot::structs::Snapshot =
                snapshot::structs::Snapshot::from_bytes(&snapshot_str)
                    .unwrap()
                    .0;

            snapshot_struct.scheduled_to_end = if snapshot_struct.scheduled_to_end > _shares {
                snapshot_struct
                    .scheduled_to_end
                    .checked_sub(_shares)
                    .ok_or(ApiError::User(error_codes::ErrorCodes::Underflow as u16))
                    .unwrap_or_revert()
            } else {
                0.into()
            };
            let snapshot_str: Vec<u8> = snapshot_struct.clone().into_bytes().unwrap();
            Snapshot::set_struct_from_key(
                self,
                &day,
                snapshot_str.clone(),
                String::from(SNAPSHOT_SNAPSHOTS_DICT),
            );
        }
    }

    fn _share_price_update(
        &self,
        _staked_amount: U256,
        _reward_amount: U256,
        _referrer: Key,
        _lock_days: U256,
        _stake_shares: U256,
    ) {
        let mut current_wise_day: u64 = Timing::_current_wise_day(self);
        let declaration_constants_string: Vec<u8> = Declaration::get_declaration_constants(self);

        let declaration_constants_struct: declaration::parameters::ConstantParameters =
            declaration::parameters::ConstantParameters::from_bytes(&declaration_constants_string)
                .unwrap()
                .0;

        let globals_share_price: U256 =
            Globals::get_globals(self, String::from(GLOBALS_SHARE_PRICE));

        if _stake_shares > 0.into()
            && current_wise_day > (declaration_constants_struct.formula_day).into()
        {
            let mut new_share_price: U256 = self._get_new_share_price(
                _staked_amount,
                _reward_amount,
                _stake_shares,
                _lock_days,
                _referrer,
            );

            if new_share_price > globals_share_price {
                if new_share_price
                    >= (globals_share_price
                        .checked_mul(110.into())
                        .ok_or(ApiError::User(error_codes::ErrorCodes::Overflow as u16))
                        .unwrap_or_revert()
                        / U256::from(100))
                {
                    // new_share_price = globals_share_price * U256::from(110) / U256::from(100);
                    new_share_price = globals_share_price
                        .checked_mul(110.into())
                        .ok_or(ApiError::User(error_codes::ErrorCodes::Overflow as u16))
                        .unwrap_or_revert()
                        / U256::from(100)
                }

                emit(&WiseEvents::NewSharePrice {
                    new_share_price,
                    old_share_price: globals_share_price,
                    current_wise_day: Timing::_current_wise_day(self),
                });

                Globals::set_globals(self, String::from(GLOBALS_SHARE_PRICE), new_share_price);
            }
            return;
        }

        current_wise_day = Timing::_current_wise_day(self);

        if current_wise_day == (declaration_constants_struct.formula_day as u64) {
            Globals::set_globals(
                self,
                String::from(GLOBALS_SHARE_PRICE),
                U256::from(110)
                    .checked_mul(
                        U256::from(10)
                            .checked_pow(U256::from(15))
                            .unwrap_or_revert(),
                    )
                    .ok_or(ApiError::User(error_codes::ErrorCodes::Overflow as u16))
                    .unwrap_or_revert(),
            );
        }
    }

    fn _get_new_share_price(
        &self,
        _staked_amount: U256,
        _reward_amount: U256,
        _stake_shares: U256,
        _lock_days: U256,
        _referrer: Key,
    ) -> U256 {
        let const1: U256 = U256::from(10)
            .checked_mul(
                U256::from(10)
                    .checked_pow(U256::from(9))
                    .ok_or(ApiError::User(error_codes::ErrorCodes::Overflow as u16))
                    .unwrap_or_revert(),
            )
            .ok_or(ApiError::User(error_codes::ErrorCodes::Overflow as u16))
            .unwrap_or_revert();
        let const2: U256 = U256::from(11)
            .checked_mul(
                U256::from(10)
                    .checked_pow(U256::from(9))
                    .ok_or(ApiError::User(error_codes::ErrorCodes::Overflow as u16))
                    .unwrap_or_revert(),
            )
            .ok_or(ApiError::User(error_codes::ErrorCodes::Overflow as u16))
            .unwrap_or_revert();
        let const3: U256 = U256::from(1)
            .checked_mul(
                U256::from(10)
                    .checked_pow(U256::from(8))
                    .ok_or(ApiError::User(error_codes::ErrorCodes::Overflow as u16))
                    .unwrap_or_revert(),
            )
            .ok_or(ApiError::User(error_codes::ErrorCodes::Overflow as u16))
            .unwrap_or_revert();
        let _bonus_amount = self._get_bonus(
            _lock_days,
            if Self::_non_zero_address(_referrer) {
                const2
            } else {
                const1
            },
        );

        // ((_staked_amount + _reward_amount) * _bonus_amount * const3) / _stake_shares
        _staked_amount
            .checked_add(_reward_amount)
            .ok_or(ApiError::User(error_codes::ErrorCodes::Overflow as u16))
            .unwrap_or_revert()
            .checked_mul(_bonus_amount)
            .ok_or(ApiError::User(error_codes::ErrorCodes::Overflow as u16))
            .unwrap_or_revert()
            .checked_mul(const3)
            .ok_or(ApiError::User(error_codes::ErrorCodes::Overflow as u16))
            .unwrap_or_revert()
            .checked_div(_stake_shares)
            .ok_or(ApiError::User(
                error_codes::ErrorCodes::DivisionByZero as u16,
            ))
            .unwrap_or_revert()
    }

    fn check_mature_stake(&self, _staker: Key, _stake_id: Vec<u32>) -> bool {
        let key: String = Declaration::_generate_key_for_dictionary(self, &_staker, &_stake_id);
        let stake: Vec<u8> =
            Declaration::get_struct_from_key(self, key, DECLARATION_STAKES_DICT.to_string());

        let is_mature: bool = Helper::is_mature_stake(self, stake);

        is_mature
    }

    fn check_stake_by_id(&self, _staker: Key, _stake_id: Vec<u32>) -> (Vec<u8>, U256, bool) {
        let key: String = Declaration::_generate_key_for_dictionary(self, &_staker, &_stake_id);
        let mut stake: Vec<u8> =
            Declaration::get_struct_from_key(self, key, DECLARATION_STAKES_DICT.to_string());
        let is_mature: bool = Helper::is_mature_stake(self, stake.clone());

        let mut stake_struct: declaration::structs::Stake =
            declaration::structs::Stake::from_bytes(&stake).unwrap().0;
        stake_struct.reward_amount = self._check_reward_amount(&stake_struct);
        let penalty_amount: U256 = self._calculate_penalty_amount(&stake_struct);
        stake = stake_struct.clone().into_bytes().unwrap();
        (stake, penalty_amount, is_mature)
    }

    fn _stakes_shares(
        &self,
        _staked_amount: U256,
        _lock_days: U256,
        _referrer: Key,
        _share_price: U256,
    ) -> U256 {
        let constant_a: U256 = U256::from(10)
            .checked_mul(
                U256::from(10)
                    .checked_pow(U256::from(9))
                    .ok_or(ApiError::User(error_codes::ErrorCodes::Overflow as u16))
                    .unwrap_or_revert(),
            )
            .ok_or(ApiError::User(error_codes::ErrorCodes::Overflow as u16))
            .unwrap_or_revert();
        let constant_b: U256 = U256::from(11)
            .checked_mul(
                U256::from(10)
                    .checked_pow(U256::from(9))
                    .ok_or(ApiError::User(error_codes::ErrorCodes::Overflow as u16))
                    .unwrap_or_revert(),
            )
            .ok_or(ApiError::User(error_codes::ErrorCodes::Overflow as u16))
            .unwrap_or_revert();
        if Self::_non_zero_address(_referrer) {
            self._shares_amount(_staked_amount, _lock_days, _share_price, constant_b)
        } else {
            self._shares_amount(_staked_amount, _lock_days, _share_price, constant_a)
        }
    }

    fn _shares_amount(
        &self,
        _staked_amount: U256,
        _lock_days: U256,
        _share_price: U256,
        _extra_bonus: U256,
    ) -> U256 {
        let bonus: U256 = self._get_bonus(_lock_days, _extra_bonus);
        let base_amount = self._base_amount(_staked_amount, _share_price);
        let constant: U256 = U256::from(10)
            .checked_mul(
                U256::from(10)
                    .checked_pow(U256::from(9))
                    .ok_or(ApiError::User(error_codes::ErrorCodes::Overflow as u16))
                    .unwrap_or_revert(),
            )
            .ok_or(ApiError::User(error_codes::ErrorCodes::Overflow as u16))
            .unwrap_or_revert();

        base_amount
            .checked_mul(bonus)
            .unwrap_or_revert()
            .checked_div(constant)
            .unwrap_or_revert()
    }

    fn _get_bonus(&self, _lock_days: U256, _extra_bonus: U256) -> U256 {
        let constants_struct: Vec<u8> = Declaration::get_declaration_constants(self);
        let constants_struct: declaration::parameters::ConstantParameters =
            declaration::parameters::ConstantParameters::from_bytes(&constants_struct)
                .unwrap()
                .0;

        self._regular_bonus(
            _lock_days,
            constants_struct.daily_bonus_a,
            U256::from(constants_struct.max_bonus_days_a),
        ) + if _lock_days > U256::from(constants_struct.max_bonus_days_a) {
            self._regular_bonus(
                _lock_days - constants_struct.max_bonus_days_a,
                U256::from(0),
                U256::from(0),
            )
        } else {
            self._regular_bonus(
                U256::from(0),
                constants_struct.daily_bonus_b,
                U256::from(constants_struct.max_bonus_days_b),
            )
        } + _extra_bonus
    }

    fn _regular_bonus(&self, _lock_days: U256, _daily: U256, _max_days: U256) -> U256 {
        let ret: U256 = if _lock_days > _max_days {
            // _max_days * _daily
            _max_days
                .checked_mul(_daily)
                .ok_or(ApiError::User(error_codes::ErrorCodes::Overflow as u16))
                .unwrap_or_revert()
        } else {
            _lock_days
                .checked_mul(_daily)
                .ok_or(ApiError::User(error_codes::ErrorCodes::Overflow as u16))
                .unwrap_or_revert()
        };
        ret.checked_div(U256::from(100).checked_pow(9.into()).unwrap())
            .ok_or(ApiError::User(error_codes::ErrorCodes::Overflow as u16))
            .unwrap_or_revert()
    }

    fn _base_amount(&self, _staked_amount: U256, _share_price: U256) -> U256 {
        let constants_struct: Vec<u8> = Declaration::get_declaration_constants(self);
        let constants_struct: declaration::parameters::ConstantParameters =
            declaration::parameters::ConstantParameters::from_bytes(&constants_struct)
                .unwrap()
                .0;

        _staked_amount
            .checked_mul(U256::from(constants_struct.precision_rate))
            .unwrap()
            .checked_div(_share_price)
            .unwrap()
    }

    fn _referrer_shares(&self, _staked_amount: U256, _lock_days: U256, _referrer: Key) -> U256 {
        let constants_struct: Vec<u8> = Declaration::get_declaration_constants(self);
        let constants_struct: declaration::parameters::ConstantParameters =
            declaration::parameters::ConstantParameters::from_bytes(&constants_struct)
                .unwrap()
                .0;
        let share_price: U256 = Globals::get_globals(self, String::from(GLOBALS_SHARE_PRICE));
        let critical_mass_referrer: bool = Helper::not_critical_mass_referrer(self, _referrer);
        let constant: U256 = U256::from(100)
            .checked_pow(U256::from(9))
            .unwrap_or_revert(); //10E9 == 10*10**9

        if critical_mass_referrer || _lock_days < constants_struct.min_referral_days.into() {
            U256::from(0)
        } else {
            self._shares_amount(_staked_amount, _lock_days, share_price, constant)
        }
    }

    fn _check_reward_amount(&self, _stake: &declaration::structs::Stake) -> U256 {
        if _stake.is_active {
            self._detect_reward(&_stake)
        } else {
            _stake.reward_amount
        }
    }

    fn _detect_reward(&self, _stake: &declaration::structs::Stake) -> U256 {
        let stake_status: bool =
            Helper::stake_not_started(self, _stake.clone().into_bytes().unwrap());
        if stake_status {
            U256::from(0)
        } else {
            self._calculate_reward_amount(_stake)
        }
    }

    fn _store_penalty(&self, _store_day: u64, _penalty: U256) {
        if _penalty > 0.into() {
            let mut total_penalty: U256 =
                Declaration::get_total_penalties(self, U256::from(_store_day));
            // total_penalty = total_penalty + _penalty;
            total_penalty = total_penalty
                .checked_add(_penalty)
                .ok_or(ApiError::User(error_codes::ErrorCodes::Overflow as u16))
                .unwrap_or_revert();
            Declaration::set_total_penalties(self, U256::from(_store_day), total_penalty);
        }
    }

    fn _calculate_penalty_amount(&self, _stake: &declaration::structs::Stake) -> U256 {
        let stake_string: Vec<u8> = _stake.clone().into_bytes().unwrap();
        let stake_status: bool = Helper::stake_not_started(self, stake_string.clone());
        let stake_maturity: bool = Helper::is_mature_stake(self, stake_string.clone());

        if stake_status || stake_maturity {
            U256::from(0)
        } else {
            self._get_penalties(&_stake)
        }
    }

    fn _get_penalties(&self, _stake: &declaration::structs::Stake) -> U256 {
        let stake_string: Vec<u8> = _stake.clone().into_bytes().unwrap();

        let days_left: U256 = Helper::days_left(self, stake_string.clone());
        let locked_days: U256 = Helper::get_lock_days(self, stake_string.clone());

        _stake.staked_amount
            * (U256::from(100) + (U256::from(800) * (days_left - U256::from(1)) / locked_days))
            / U256::from(1000)
    }

    fn _calculate_reward_amount(&self, _stake: &declaration::structs::Stake) -> U256 {
        let starting_day: U256 = Helper::starting_day(self, _stake.clone().into_bytes().unwrap());
        let calculation_day: U256 =
            Helper::calculation_day(self, _stake.clone().into_bytes().unwrap());

        self._loop_reward_amount(_stake.stakes_shares, starting_day, calculation_day)
    }

    fn _loop_reward_amount(&self, _stake_shares: U256, _start_day: U256, _final_day: U256) -> U256 {
        let constants_struct: Vec<u8> = Declaration::get_declaration_constants(self);
        let constants_struct: declaration::parameters::ConstantParameters =
            declaration::parameters::ConstantParameters::from_bytes(&constants_struct)
                .unwrap()
                .0;
        let mut reward_amount: U256 = U256::from(0);
        for _day in _start_day.as_u64().._final_day.as_u64() {
            // get snapshot struct and convert to struct type
            let snapshot_str: Vec<u8> = Snapshot::get_struct_from_key(
                self,
                &U256::from(_day),
                String::from(SNAPSHOT_SNAPSHOTS_DICT),
            );
            let snapshot_struct: snapshot::structs::Snapshot =
                snapshot::structs::Snapshot::from_bytes(&snapshot_str)
                    .unwrap()
                    .0;

            // calc stuff
            let res: U256 = (_stake_shares.checked_mul(constants_struct.precision_rate))
                .unwrap_or_revert()
                .checked_div(snapshot_struct.inflation_amount)
                .unwrap_or_revert();
            // add to reward_amount
            reward_amount = reward_amount + res;
        }
        reward_amount
    }

    fn _is_contract(address: Key) -> bool {
        // account hashes starts with 'account-' where as contract hashes starts with 'hash-'
        if address.to_formatted_string().starts_with("hash") {
            true
        } else {
            false
        }
    }
}
