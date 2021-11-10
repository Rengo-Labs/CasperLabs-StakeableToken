use alloc::string::ToString;
use alloc::{string::String, vec::Vec};

use crate::data::{self};

use crate::config::*;
use casper_contract::contract_api::runtime;
use casper_types::{runtime_args, ApiError, ContractHash, Key, RuntimeArgs, U256};
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
    // fn create_stake(
    //     &mut self,
    //     staked_amount: U256,
    //     lock_days: u64,
    //     referrer: Key,
    // ) -> (Vec<u32>, U256, Vec<u32>) {

    //     if self.get_caller() == referrer || Self::_is_contract(referrer) == true {
    //         runtime::revert(ApiError::UnexpectedKeyVariant);
    //     }

    //     let declaration_contract_hash = self.convert_to_contract_hash(data::get_declaration_hash());
    //     let constants: String = runtime::call_contract(
    //         declaration_contract_hash,
    //         "get_declaration_constants",
    //         runtime_args! {},
    //     );
    //     let constants: parameters::ConstantParameters = serde_json::from_str(&constants).unwrap();

    //     if lock_days < constants.min_lock_days.into() || lock_days > constants.max_lock_days.into() {
    //         runtime::revert(ApiError::UnexpectedKeyVariant);
    //     }

    //     if (staked_amount < constants.min_stake_amount) {
    //         runtime::revert(ApiError::UnexpectedKeyVariant);
    //     } 
        
    //     let helper_contract_hash = self.convert_to_contract_hash(data::get_helper_hash());
    //     let (new_stake, stake_id, start_day) = self._create_stake(self.get_caller(), staked_amount, lock_days, referrer);
    //     if new_stake.referrer_shares > 0.into() {
            
    //         let mut referrer_link = structs::ReferrerLink {
    //             staker: self.get_caller(), 
    //             stake_id: stake_id, 
    //             reward_amount: 0.into(), 
    //             processed_days: 0.into(), 
    //             is_active: true
    //         };

    //         let referral_id: Vec<u32> = runtime::call_contract(
    //             helper_contract_hash,
    //             "generate_referral_id",
    //             runtime_args! {"referrer" => referrer},
    //         );
    //         let struct_key: String = Self::_generate_key_for_dictionary(&referrer, &referral_id); 
    //         //generate key and pass it to Declaration contract which will return the struct stored against this key in that contract
    //         let () = runtime::call_contract(
    //             declaration_contract_hash,
    //             "set_struct_from_key",
    //             runtime_args! {"key" => struct_key,"value"=>serde_json::to_string(&referrer_link).unwrap(), "struct_name" => structs::REFERRER_LINK}, // convert structure to json string and save
    //         );

    //         let () = runtime::call_contract(
    //             helper_contract_hash,
    //             "increase_referral_count",
    //             runtime_args! {"referrer" => referrer},
    //         );
    //         let referral_token_contract_hash = self.convert_to_contract_hash(data::get_referral_token_hash());
    //         let () = runtime::call_contract(
    //             referral_token_contract_hash,
    //             "add_referrer_shares_to_end",
    //             runtime_args! {"final_day" => new_stake.final_day,"shares" => new_stake.referrer_shares},
    //         );
    //     }

    //     let struct_key0: String = Self::_generate_key_for_dictionary(&self.get_caller(), &stake_id); // generate key and pass it to Declaration contract which will return the struct stored against this key in that contract
    //     let () = runtime::call_contract(
    //         declaration_contract_hash,
    //         "set_struct_from_key",
    //         runtime_args! {"key" => struct_key0,"value"=>serde_json::to_string(&new_stake).unwrap(), "struct_name" => structs::STAKES}, // convert structure to json string and save
    //     );
        
    //     let () = runtime::call_contract(
    //         helper_contract_hash,
    //         "increase_stake_count",
    //         runtime_args! {"staker" => self.get_caller()},
    //     );

    //     let helper_contract_hash = self.convert_to_contract_hash(data::get_helper_hash());
    //     let referral_id: Vec<u32> = runtime::call_contract(
    //         helper_contract_hash,
    //         "generate_referral_id",
    //         runtime_args! {"referrer" => referrer},
    //     );

    //     let global_contract_hash = self.convert_to_contract_hash(data::get_globals_hash());
    //     let () = runtime::call_contract(
    //         global_contract_hash,
    //         "increase_globals",
    //         runtime_args! {"staked" => new_stake.staked_amount,"shared" => new_stake.stakes_shares,"r_shareds" => new_stake.referrer_shares},
    //     );
    //     //self.add_scheduled_shared(new_stake.final_day, new_stake.stakes_shares)

    //     (stake_id, U256::from(start_day), referral_id)
    // }

    // fn _create_stake(
    //     &mut self,
    //     staker: Key,
    //     staked_amount: U256,
    //     lock_days: u64,
    //     referrer: Key,
    // ) -> (structs::Stake, Vec<u32>, u64) {
    //     let bep20_contract_hash = self.convert_to_contract_hash(data::get_bep20_hash());
    //     let () = runtime::call_contract(
    //         bep20_contract_hash,
    //         "_burn",
    //         runtime_args! {"account" => staker,"amount"=>staked_amount},
    //     );
    //     let timing_contract_hash = self.convert_to_contract_hash(data::get_timing_hash());
    //     let _start_day: u64 =
    //         runtime::call_contract(timing_contract_hash, "_next_wise_day", runtime_args! {});
    //     let helper_contract_hash = self.convert_to_contract_hash(data::get_helper_hash());

    //     let stake_id: Vec<u32> = runtime::call_contract(
    //         helper_contract_hash,
    //         "generate_stake_id",
    //         runtime_args! {"staker" => staker},
    //     );
    //     let mut new_stake = structs::STAKES;
    //     // new_stake.lock_days = lock_days;
    //     // new_stake.start_day = _start_day;
    //     // new_stake.final_day = _start_day + lock_days;
    //     // new_stake.is_active = true;
    //     // new_stake.staked_amount = staked_amount;
    //     // new_stake.stakes_shares =
    //     //     stake_shares(staked_amount, lock_days, referrer, globals.share_proce)
    //     let referral_token_contract_hash =
    //         self.convert_to_contract_hash(data::get_referral_token_hash());

    //     let busd_equivalent: U256 = runtime::call_contract(
    //         referral_token_contract_hash,
    //         "get_busd_equivalent",
    //         runtime_args! {},
    //     );
    //     let declaration_contract_hash = self.convert_to_contract_hash(data::get_declaration_hash());
    //     let constants: String = runtime::call_contract(
    //         declaration_contract_hash,
    //         "get_declaration_constants",
    //         runtime_args! {},
    //     );
    //     let constants: parameters::ConstantParameters = serde_json::from_str(&constants).unwrap();

    //     // new_stake.dai_equivalent =
    //     //     (busd_equivalent * new_stake.staked_amount) / constants.yodas_per_wise;
    //     if (self._non_zero_address(referrer)) {
    //         // new_stake.referrer = referrer;
    //         // let () = runtime::call_contract(
    //         //     referral_token_contract_hash,
    //         //     "add_critical_mass",
    //         //     runtime_args! {"referrer"=>new_stake.referrer,"dai_equivalend"=>new_stake._dai_equivalent},
    //         // );
    //         // new_stake.referrer_shares = referrer_shares(staked_amount, lock_days, referrer)
    //     }
        
    //     return (new_stake, stake_id, _start_day);
    // }

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
    fn convert_to_contract_hash(&mut self, contract_hash: Key) -> ContractHash {
        let contract_hash_add_array = match contract_hash {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        return ContractHash::new(contract_hash_add_array);
    }
    fn _non_zero_address(&mut self, key: Key) -> bool {
        let zero_addr: Key = Key::Hash([0u8; 32]);
        return key != zero_addr;
    }
    fn _is_contract(address: Key) -> bool{                  // need to work on it
        true
    }
}