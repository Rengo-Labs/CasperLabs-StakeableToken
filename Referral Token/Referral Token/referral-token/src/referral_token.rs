use alloc::string::ToString;
use alloc::{string::String, vec::Vec};

use crate::data::{self};

use crate::config::*;
// use crate::config::parameters::*;
use casper_contract::contract_api::runtime;
use casper_types::{runtime_args, ApiError, ContractHash, Key, RuntimeArgs, U256};
use contract_utils::{ContractContext, ContractStorage};

pub trait REFERRALTOKEN<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(
        &mut self,
        declaration_hash: Key,
        timing_hash: Key,
        helper_hash: Key,
        bep20_hash: Key,
        snapshot_hash: Key,
        contract_hash: Key,
    ) {
        data::set_hash(contract_hash);
        data::set_declaration_hash(declaration_hash);
        data::set_timing_hash(timing_hash);
        data::set_helper_hash(helper_hash);
        data::set_bep20_hash(bep20_hash);
        data::set_snapshot_hash(snapshot_hash);
    }

    fn referral_token(&mut self) {
        data::set_owner(self.get_caller());
    }

    fn _add_referrer_shares_to_end(&mut self, _final_day: U256, _shares: U256) {
        let declaration_contract_hash = self.convert_to_contract_hash(data::get_declaration_hash());
        let final_day: U256 = runtime::call_contract(
            declaration_contract_hash,
            "get_referral_shares_to_end",
            runtime_args! {"key" => _final_day},
        );
        let () = runtime::call_contract(
            declaration_contract_hash,
            "set_referral_shares_to_end",
            runtime_args! {"key" => _final_day, "value"=>final_day+_shares},
        );
    }
    fn _remove_referrer_shares_to_end(&mut self, _final_day: U256, _shares: U256) {
        let helper_contract_hash = self.convert_to_contract_hash(data::get_helper_hash());
        let is_final_day: bool = runtime::call_contract(
            helper_contract_hash,
            "not_past",
            runtime_args! {"day" => _final_day},
        );
        if is_final_day {
            let declaration_contract_hash =
                self.convert_to_contract_hash(data::get_declaration_hash());
            let final_day: U256 = runtime::call_contract(
                declaration_contract_hash,
                "get_referral_shares_to_end",
                runtime_args! {"key" => _final_day},
            );
            if final_day > _shares {
                let () = runtime::call_contract(
                    declaration_contract_hash,
                    "set_referral_shares_to_end",
                    runtime_args! {"key" => _final_day,"value"=>final_day-_shares},
                );
            } else {
                let () = runtime::call_contract(
                    declaration_contract_hash,
                    "set_referral_shares_to_end",
                    runtime_args! {"key" => _final_day,"value"=>0},
                );
            }
        } else {
            let timing_contract_hash = self.convert_to_contract_hash(data::get_timing_hash());
            let _day: u64 = runtime::call_contract(
                timing_contract_hash,
                "_previous_wise_day",
                runtime_args! {},
            );
            let snapshot_contract_hash = self.convert_to_contract_hash(data::get_snapshot_hash());
            let struct_key: U256 = U256::from(_day);
            let snapshots: String = runtime::call_contract(
                snapshot_contract_hash,
                "get_struct_from_key",
                runtime_args! {"key" => struct_key.clone(), "struct_name" => structs::RSNAPSHOT},
            );
            let mut snapshots: structs::RSnapshot = serde_json::from_str(&snapshots).unwrap();

            if snapshots.scheduled_to_end > _shares {
                snapshots.scheduled_to_end = snapshots.scheduled_to_end - _shares;
            } else {
                snapshots.scheduled_to_end = 0.into();
            }
            let () = runtime::call_contract(
                snapshot_contract_hash,
                "set_struct_from_key",
                runtime_args! {"key" => struct_key.clone(),"value"=>serde_json::to_string(&snapshots).unwrap(),"struct_name" => structs::RSNAPSHOT}, // convert structure to json string and save
            );
        }
    }
    fn _below_threshhold_level(&mut self, _referrer: Key) -> bool {
        let declaration_contract_hash = self.convert_to_contract_hash(data::get_declaration_hash());
        let struct_key: String = _referrer.to_formatted_string();
        let critical_mass: String = runtime::call_contract(
            declaration_contract_hash,
            "get_struct_from_key",
            runtime_args! {"key" => struct_key, "struct_name" => structs::CRITICAL_MASS},
        );
        let critical_mass: structs::CriticalMass = serde_json::from_str(&critical_mass).unwrap();

        let constants: String = runtime::call_contract(
            declaration_contract_hash,
            "get_declaration_constants",
            runtime_args! {},
        );
        let constants: parameters::ConstantParameters = serde_json::from_str(&constants).unwrap();
        if critical_mass.total_amount > constants.threshold_limit {
            return true;
        } else {
            return false;
        }
    }
    fn _add_critical_mass(&mut self, _referrer: Key, _dai_equivalent: U256) {
        let declaration_contract_hash = self.convert_to_contract_hash(data::get_declaration_hash());
        let struct_key: String = _referrer.to_formatted_string();
        let critical_mass: String = runtime::call_contract(
            ContractHash::from(declaration_contract_hash),
            "get_struct_from_key",
            runtime_args! {"key" => struct_key.clone(), "struct_name" => structs::CRITICAL_MASS},
        );
        let mut critical_mass: structs::CriticalMass =
            serde_json::from_str(&critical_mass).unwrap();
        critical_mass.total_amount = critical_mass.total_amount + _dai_equivalent;
        critical_mass.activation_day = self._determine_activation_day(_referrer);
        let () = runtime::call_contract(
            declaration_contract_hash,
            "set_struct_from_key",
            runtime_args! {"key" => struct_key.clone(),"value"=>serde_json::to_string(&critical_mass).unwrap(),"struct_name" => structs::CRITICAL_MASS}, // convert structure to json string and save
        );
    }
    fn _remove_critical_mass(&mut self, _referrer: Key, _dai_equivalent: U256, _start_day: U256) {
        let helper_contract_hash = self.convert_to_contract_hash(data::get_helper_hash());
        let is_not_future: bool = runtime::call_contract(
            helper_contract_hash,
            "not_future",
            runtime_args! {"day" => _start_day},
        );
        let is_non_zero_address: bool = self._non_zero_address(_referrer);
        if is_not_future == false && is_non_zero_address {
            let declaration_contract_hash =
                self.convert_to_contract_hash(data::get_declaration_hash());
            let struct_key: String = _referrer.to_formatted_string();
            let critical_mass: String = runtime::call_contract(
                ContractHash::from(declaration_contract_hash),
                "get_struct_from_key",
                runtime_args! {"key" => struct_key.clone(), "struct_name" => structs::CRITICAL_MASS},
            );
            let mut critical_mass: structs::CriticalMass =
                serde_json::from_str(&critical_mass).unwrap();

            if critical_mass.total_amount > _dai_equivalent {
                critical_mass.total_amount = critical_mass.total_amount - _dai_equivalent;
            } else {
                critical_mass.total_amount = 0.into();
            }
            critical_mass.activation_day = self._determine_activation_day(_referrer);
            let () = runtime::call_contract(
                declaration_contract_hash,
                "set_struct_from_key",
                runtime_args! {"key" => struct_key,"value"=>serde_json::to_string(&critical_mass).unwrap(), "struct_name" => structs::CRITICAL_MASS}, // convert structure to json string and save
            );
        }
    }
    fn _determine_activation_day(&mut self, _referrer: Key) -> U256 {
        if self._below_threshhold_level(_referrer) {
            return 0.into();
        } else {
            return self._activation_day(_referrer);
        }
    }
    fn _activation_day(&mut self, _referrer: Key) -> U256 {
        let declaration_contract_hash = self.convert_to_contract_hash(data::get_declaration_hash());
        let struct_key: String = _referrer.to_formatted_string();
        let critical_mass: String = runtime::call_contract(
            declaration_contract_hash,
            "get_struct_from_key",
            runtime_args! {"key" => struct_key.clone(), "struct_name" => structs::CRITICAL_MASS},
        );
        let critical_mass: structs::CriticalMass = serde_json::from_str(&critical_mass).unwrap();

        if critical_mass.activation_day > 0.into() {
            return critical_mass.activation_day;
        } else {
            let timing_contract_hash = self.convert_to_contract_hash(data::get_timing_hash());
            let _current_wise_day: U256 =
                runtime::call_contract(timing_contract_hash, "_current_wise_day", runtime_args! {});
            return _current_wise_day;
        }
    }
    fn get_busd_equivalent(&mut self) -> U256 {
        self._get_busd_equivalent()
    }
    fn _get_busd_equivalent(&mut self) -> U256 {
        let declaration_contract_hash = self.convert_to_contract_hash(data::get_declaration_hash());
        let busd_equivalent: U256 = runtime::call_contract(
            declaration_contract_hash,
            "get_declaration_constants",
            runtime_args! {},
        );
        busd_equivalent
    }
    fn referrer_interest(&mut self, _referral_id: u32, _scrape_days: U256) {
        self._referrer_interest(self.get_caller(), _referral_id, _scrape_days);
    }
    fn referrer_interest_bulk(&mut self, _referral_ids: Vec<u32>, _scrape_days: Vec<U256>) {
        for i in 0.._referral_ids.len() {
            self._referrer_interest(self.get_caller(), _referral_ids[i], _scrape_days[i]);
        }
    }
    fn _referrer_interest(&mut self, _referrer: Key, _referral_id: u32, _process_days: U256) {
        let declaration_contract_hash = self.convert_to_contract_hash(data::get_declaration_hash());
        let struct_key0: String = Self::_generate_key_for_dictionary(&_referrer, &_referral_id); // generate key and pass it to Declaration contract which will return the struct stored against this key in that contract
        let referrer_link_struct: String = runtime::call_contract(
            declaration_contract_hash,
            "get_struct_from_key",
            runtime_args! {"key" => struct_key0.clone(), "struct_name" => structs::REFERRER_LINK},
        );
        let mut referral_link: structs::ReferrerLink =
            serde_json::from_str(&referrer_link_struct).unwrap();
        if referral_link.is_active == true {
            let _staker: Key = referral_link.staker;
            let _stake_id: u32 = referral_link.stake_id;
            let struct_key1: String = Self::_generate_key_for_dictionary(&_staker, &_stake_id); // generate key and pass it to Declaration contract which will return the struct stored against this key in that contract
            let stakes_struct: String = runtime::call_contract(
                declaration_contract_hash,
                "get_struct_from_key",
                runtime_args! {"key" => struct_key1, "struct_name" => structs::STAKES},
            );
            let stake: structs::Stake = serde_json::from_str(&stakes_struct).unwrap(); // convert json string received, back to Stake Structure

            let start_day: U256 = self._determine_start_day(&stake, &referral_link);
            let mut final_day: U256 = self._determine_final_day(&stake);
            let helper_contract_hash = self.convert_to_contract_hash(data::get_helper_hash());
            let is_stake_ended: bool = runtime::call_contract(
                helper_contract_hash,
                "_stake_ended",
                runtime_args! {"_stake" => serde_json::to_string(&stake).unwrap()}, // convert structure to json string and save
            );

            if is_stake_ended {
                let days_diff: U256 = runtime::call_contract(
                    helper_contract_hash,
                    "_days_diff",
                    runtime_args! {"_start_day" => start_day,"_end_date"=>final_day},
                );
                if _process_days > 0.into() && _process_days < days_diff {
                    referral_link.processed_days = referral_link.processed_days + _process_days;
                    final_day = start_day + _process_days.as_u64();
                } else {
                    referral_link.is_active = false;
                }
            } else {
                let timing_contract_hash = self.convert_to_contract_hash(data::get_timing_hash());
                let _current_wise_day: U256 = runtime::call_contract(
                    timing_contract_hash,
                    "_current_wise_day",
                    runtime_args! {},
                );
                let process_days: U256 = runtime::call_contract(
                    helper_contract_hash,
                    "_days_diff",
                    runtime_args! {"_start_day" => start_day,"_end_date"=>_current_wise_day},
                );
                referral_link.processed_days = referral_link.processed_days + process_days;
                final_day = start_day + process_days.as_u64();
            }
            let referral_interest = self._check_referral_interest(&stake, start_day, final_day);
            referral_link.reward_amount = referral_link.reward_amount + referral_interest;
            let () = runtime::call_contract(
                declaration_contract_hash,
                "set_struct_from_key",
                runtime_args! {"key" => struct_key0.clone(),"value"=>serde_json::to_string(&referral_link).unwrap(), "struct_name" => structs::REFERRER_LINK}, // convert structure to json string and save
            );
            let bep20_contract_hash = self.convert_to_contract_hash(data::get_bep20_hash());
            let () = runtime::call_contract(
                bep20_contract_hash,
                "_mint",
                runtime_args! {"account" => _referrer,"amount"=>referral_interest},
            );
        }
    }
    fn check_referrals_by_id(
        &mut self,
        _referrer: Key,
        _referral_id: u32,
    ) -> (Key, u32, U256, U256, bool, bool, bool, bool) {
        let declaration_contract_hash = self.convert_to_contract_hash(data::get_declaration_hash());
        let struct_key: String = Self::_generate_key_for_dictionary(&_referrer, &_referral_id); // generate key and pass it to Declaration contract which will return the struct stored against this key in that contract
        let referrer_link_struct: String = runtime::call_contract(
            declaration_contract_hash,
            "get_struct_from_key",
            runtime_args! {"key" => struct_key, "struct_name" => structs::REFERRER_LINK},
        );
        let referral_link: structs::ReferrerLink =
            serde_json::from_str(&referrer_link_struct).unwrap();
        let _staker: Key = referral_link.staker;
        let _stake_id: u32 = referral_link.stake_id;
        let is_active_referral: bool = referral_link.is_active;
        let struct_key: String = Self::_generate_key_for_dictionary(&_staker, &_stake_id); // generate key and pass it to Declaration contract which will return the struct stored against this key in that contract
        let stakes_struct: String = runtime::call_contract(
            declaration_contract_hash,
            "get_struct_from_key",
            runtime_args! {"key" => struct_key, "struct_name" => structs::STAKES},
        );
        let stake: structs::Stake = serde_json::from_str(&stakes_struct).unwrap(); // convert json string received, back to Stake Structure

        let referrer_shares = stake.referrer_shares;
        let start_day: U256 = self._determine_start_day(&stake, &referral_link);
        let final_day: U256 = self._determine_final_day(&stake);
        let referral_interest = self._check_referral_interest(&stake, start_day, final_day);
        let is_active_stake = stake.is_active;

        let helper_contract_hash = self.convert_to_contract_hash(data::get_helper_hash());
        let is_stake_ended: bool = runtime::call_contract(
            helper_contract_hash,
            "stake_ended",
            runtime_args! {"stake" => serde_json::to_string(&stake).unwrap()}, // convert structure to json string and save
        );
        let is_ended_stake = is_stake_ended;
        let is_stake_mature: bool = runtime::call_contract(
            helper_contract_hash,
            "is_mature_stake",
            runtime_args! {"stake" => serde_json::to_string(&stake).unwrap()}, // convert structure to json string and save
        );
        let is_mature_stake = is_stake_mature;
        return (
            _staker,
            _stake_id,
            referrer_shares,
            referral_interest,
            is_active_referral,
            is_active_stake,
            is_mature_stake,
            is_ended_stake,
        );
    }
    fn _check_referral_interest(
        &mut self,
        _stake: &structs::Stake,
        _start_day: U256,
        _final_day: U256,
    ) -> U256 {
        let helper_contract_hash = self.convert_to_contract_hash(data::get_helper_hash());
        let is_not_critical_mass_referrer: bool = runtime::call_contract(
            helper_contract_hash,
            "not_critical_mass_referrer",
            runtime_args! {"referrer" => _stake.referrer},
        );
        if is_not_critical_mass_referrer {
            return 0.into();
        } else {
            self._get_referral_interest(_stake, _start_day, _final_day)
        }
    }
    fn _get_referral_interest(
        &mut self,
        _stake: &structs::Stake,
        _start_day: U256,
        _final_day: U256,
    ) -> U256 {
        let mut _referral_interest: U256 = 0.into();

        let snapshot_contract_hash = self.convert_to_contract_hash(data::get_snapshot_hash());

        for _day in _start_day.as_u64().._final_day.as_u64() {
            let struct_key: U256 = U256::from(_day);
            let snapshots: String = runtime::call_contract(
                snapshot_contract_hash,
                "get_struct_from_key",
                runtime_args! {"key" => struct_key, "struct_name" => structs::RSNAPSHOT},
            );
            let snapshots: structs::RSnapshot = serde_json::from_str(&snapshots).unwrap();

            let declaration_contract_hash =
                self.convert_to_contract_hash(data::get_declaration_hash());
            let constants: String = runtime::call_contract(
                declaration_contract_hash,
                "get_declaration_constants",
                runtime_args! {},
            );
            let constants: parameters::ConstantParameters =
                serde_json::from_str(&constants).unwrap();
            _referral_interest = _referral_interest
                + _stake.stakes_shares * constants.precision_rate / snapshots.inflation_amount;
        }
        return _referral_interest;
    }
    fn _determine_start_day(
        &mut self,
        _stake: &structs::Stake,
        _link: &structs::ReferrerLink,
    ) -> U256 {
        let declaration_contract_hash = self.convert_to_contract_hash(data::get_declaration_hash());
        let struct_key: String = _stake.referrer.to_formatted_string();
        let critical_mass: String = runtime::call_contract(
            declaration_contract_hash,
            "get_struct_from_key",
            runtime_args! {"key" => struct_key, "struct_name" => structs::CRITICAL_MASS},
        );
        let critical_mass: structs::CriticalMass = serde_json::from_str(&critical_mass).unwrap();

        if critical_mass.activation_day > U256::from(_stake.start_day) {
            return critical_mass.activation_day;
        } else {
            let sum: U256 = U256::from(_stake.start_day) + _link.processed_days;
            return sum;
        }
    }
    fn _determine_final_day(&mut self, _stake: &structs::Stake) -> U256 {
        if _stake.close_day > 0 {
            return U256::from(_stake.close_day);
        } else {
            let helper_contract_hash = self.convert_to_contract_hash(data::get_helper_hash());
            let calculation_day: U256 = runtime::call_contract(
                helper_contract_hash,
                "calculation_day",
                runtime_args! {"stake" => serde_json::to_string(&_stake).unwrap()}, // convert structure to json string and save
            );
            return calculation_day;
        }
    }
    fn convert_to_contract_hash(&mut self, contract_hash: Key) -> ContractHash {
        let contract_hash_add_array = match contract_hash {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        return ContractHash::new(contract_hash_add_array);
    }
    fn _generate_key_for_dictionary(key: &Key, id: &u32) -> String {
        let mut result: String = String::from("");
        result.push_str(&key.to_formatted_string());
        result.push_str("-");
        result.push_str(&id.to_string());

        result
    }
    fn _non_zero_address(&mut self, key: Key) -> bool {
        let zero_addr: Key = Key::Hash([0u8; 32]);
        return key != zero_addr;
    }
}
