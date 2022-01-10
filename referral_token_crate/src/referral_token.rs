use alloc::string::ToString;
use alloc::{string::String, vec::Vec};
use casper_contract::unwrap_or_revert::UnwrapOrRevert;

use bep20_crate::BEP20;
use declaration_crate::Declaration;
use globals_crate::Globals;
use helper_crate::Helper;
use snapshot_crate::Snapshot;
use timing_crate::Timing;
use wise_token_utils::{commons::key_names::*, declaration, events::*, snapshot};

use casper_contract::contract_api::runtime;
use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    runtime_args, ApiError, ContractHash, Key, RuntimeArgs, U256,
};
use contract_utils::{ContractContext, ContractStorage};

pub trait ReferralToken<Storage: ContractStorage>:
    ContractContext<Storage>
    + Declaration<Storage>
    + Globals<Storage>
    + Timing<Storage>
    + Helper<Storage>
    + Snapshot<Storage>
    + BEP20<Storage>
{
    fn init(&mut self) {}

    fn _add_referrer_shares_to_end(&self, _final_day: U256, _shares: U256) {
        let shares: U256 = Declaration::get_referral_shares_to_end(self, _final_day);
        Declaration::set_referral_shares_to_end(self, _final_day, shares + _shares);
    }

    fn _remove_referrer_shares_to_end(&self, _final_day: U256, _shares: U256) {
        let is_final_day: bool = Helper::not_past(self, _final_day);
        if is_final_day {
            let final_day: U256 = Declaration::get_referral_shares_to_end(self, _final_day);
            if final_day > _shares {
                Declaration::set_referral_shares_to_end(self, _final_day, final_day - _shares);
            } else {
                Declaration::set_referral_shares_to_end(self, _final_day, U256::from(0));
            }
        } else {
            let _day: u64 = Timing::_previous_wise_day(self);
            let struct_key: U256 = U256::from(_day);
            let snapshots: Vec<u8> = Snapshot::get_struct_from_key(
                self,
                &struct_key,
                SNAPSHOT_RSNAPSHOTS_DICT.to_string(),
            );
            let mut snapshots: snapshot::structs::RSnapshot =
                snapshot::structs::RSnapshot::from_bytes(&snapshots)
                    .unwrap()
                    .0;

            if snapshots.scheduled_to_end > _shares {
                snapshots.scheduled_to_end = snapshots.scheduled_to_end - _shares;
            } else {
                snapshots.scheduled_to_end = 0.into();
            }

            Snapshot::set_struct_from_key(
                self,
                &struct_key,
                snapshots.into_bytes().unwrap(),
                SNAPSHOT_RSNAPSHOTS_DICT.to_string(),
            );
        }
    }
    fn _below_threshhold_level(&self, _referrer: Key) -> bool {
        let struct_key: String = _referrer.to_formatted_string();
        let critical_mass: Vec<u8> = Declaration::get_struct_from_key(
            self,
            struct_key,
            DECLARATION_CRITICAL_MASS_DICT.to_string(),
        );
        let critical_mass: declaration::structs::CriticalMass =
            declaration::structs::CriticalMass::from_bytes(&critical_mass)
                .unwrap()
                .0;

        let constants: Vec<u8> = Declaration::get_declaration_constants(self);
        let constants: declaration::parameters::ConstantParameters =
            declaration::parameters::ConstantParameters::from_bytes(&constants)
                .unwrap()
                .0;
        if critical_mass.total_amount > constants.threshold_limit {
            return true;
        } else {
            return false;
        }
    }
    fn _add_critical_mass(&mut self, _referrer: Key, _dai_equivalent: U256) {
        let struct_key: String = _referrer.to_formatted_string();
        let critical_mass: Vec<u8> = Declaration::get_struct_from_key(
            self,
            struct_key.clone(),
            DECLARATION_CRITICAL_MASS_DICT.to_string(),
        );
        let mut critical_mass: declaration::structs::CriticalMass =
            declaration::structs::CriticalMass::from_bytes(&critical_mass)
                .unwrap_or_revert()
                .0;
        critical_mass.total_amount = critical_mass.total_amount + _dai_equivalent;
        critical_mass.activation_day = Self::_determine_activation_day(self, _referrer);
        Declaration::set_struct_from_key(
            self,
            struct_key,
            critical_mass.into_bytes().unwrap(),
            DECLARATION_CRITICAL_MASS_DICT.to_string(),
        );
    }
    fn _remove_critical_mass(&self, _referrer: Key, _dai_equivalent: U256, _start_day: U256) {
        let is_not_future: bool = Helper::not_future(self, _start_day);
        let is_non_zero_address: bool = Self::_non_zero_address(_referrer);
        if is_not_future == false && is_non_zero_address {
            let struct_key: String = _referrer.to_formatted_string();
            let critical_mass: Vec<u8> = Declaration::get_struct_from_key(
                self,
                struct_key.clone(),
                DECLARATION_CRITICAL_MASS_DICT.to_string(),
            );
            let mut critical_mass: declaration::structs::CriticalMass =
                declaration::structs::CriticalMass::from_bytes(&critical_mass)
                    .unwrap()
                    .0;

            if critical_mass.total_amount > _dai_equivalent {
                critical_mass.total_amount = critical_mass.total_amount - _dai_equivalent;
            } else {
                critical_mass.total_amount = U256::from(0);
            }
            critical_mass.activation_day = Self::_determine_activation_day(self, _referrer);
            Declaration::set_struct_from_key(
                self,
                struct_key,
                critical_mass.into_bytes().unwrap(),
                DECLARATION_CRITICAL_MASS_DICT.to_string(),
            );
        }
    }
    fn _determine_activation_day(&self, _referrer: Key) -> U256 {
        if Self::_below_threshhold_level(self, _referrer) {
            return U256::from(0);
        } else {
            return Self::_activation_day(self, _referrer);
        }
    }
    fn _activation_day(&self, _referrer: Key) -> U256 {
        let struct_key: String = _referrer.to_formatted_string();
        let critical_mass: Vec<u8> = Declaration::get_struct_from_key(
            self,
            struct_key.clone(),
            DECLARATION_CRITICAL_MASS_DICT.to_string(),
        );
        let critical_mass: declaration::structs::CriticalMass =
            declaration::structs::CriticalMass::from_bytes(&critical_mass)
                .unwrap()
                .0;

        if critical_mass.activation_day > 0.into() {
            return critical_mass.activation_day;
        } else {
            let _current_wise_day: u64 = Timing::_current_wise_day(self);
            return U256::from(_current_wise_day);
        }
    }
    fn get_busd_equivalent(&self) -> U256 {
        Self::_get_busd_equivalent(self)
    }
    fn _get_busd_equivalent(&self) -> U256 {
        let busd_eq = Declaration::get_busd_eq(self);

        let busd_equivalent: U256 = runtime::call_contract(
            Self::convert_to_contract_hash(busd_eq),
            "get_busd_equivalent",
            runtime_args! {},
        );
        busd_equivalent
    }
    fn referrer_interest(&self, _referral_id: Vec<u32>, _scrape_days: U256) {
        Self::_referrer_interest(self, self.get_caller(), _referral_id, _scrape_days);
    }
    fn referrer_interest_bulk(&self, _referral_ids: Vec<Vec<u32>>, _scrape_days: Vec<U256>) {
        for i in 0.._referral_ids.len() {
            Self::_referrer_interest(
                self,
                self.get_caller(),
                _referral_ids[i].clone(),
                _scrape_days[i],
            );
        }
    }
    fn _referrer_interest(&self, _referrer: Key, _referral_id: Vec<u32>, _process_days: U256) {
        let struct_key0: String =
            Declaration::_generate_key_for_dictionary(self, &_referrer, &_referral_id); // generate key and pass it to Declaration contract which will return the struct stored against this key in that contract
        let referrer_link_struct: Vec<u8> = Declaration::get_struct_from_key(
            self,
            struct_key0.clone(),
            DECLARATION_REFERRER_LINK_DICT.to_string(),
        );
        let mut referral_link: declaration::structs::ReferrerLink =
            declaration::structs::ReferrerLink::from_bytes(&referrer_link_struct)
                .unwrap()
                .0;
        if referral_link.is_active == true {
            let _staker: Key = referral_link.staker;
            let _stake_id: Vec<u32> = referral_link.stake_id.clone();
            let struct_key1: String =
                Declaration::_generate_key_for_dictionary(self, &_staker, &_stake_id); // generate key and pass it to Declaration contract which will return the struct stored against this key in that contract
            let stakes_struct: Vec<u8> = Declaration::get_struct_from_key(
                self,
                struct_key1,
                DECLARATION_STAKES_DICT.to_string(),
            );
            let stake: declaration::structs::Stake =
                declaration::structs::Stake::from_bytes(&stakes_struct)
                    .unwrap()
                    .0;

            let start_day: U256 = Self::_determine_start_day(self, &stake, &referral_link);
            let mut final_day: U256 = Self::_determine_final_day(self, &stake);
            let is_stake_ended: bool =
                Helper::stake_ended(self, stake.clone().into_bytes().unwrap());

            if is_stake_ended {
                let days_diff: U256 = Helper::days_diff(self, start_day, final_day);
                if _process_days > 0.into() && _process_days < days_diff {
                    referral_link.processed_days = referral_link.processed_days + _process_days;
                    final_day = start_day + _process_days.as_u64();
                } else {
                    referral_link.is_active = false;
                }
            } else {
                let _current_wise_day: u64 = Timing::_current_wise_day(self);
                let _current_wise_day: U256 = U256::from(_current_wise_day);
                let process_days: U256 = Helper::days_diff(self, start_day, _current_wise_day);
                referral_link.processed_days = referral_link.processed_days + process_days;
                final_day = start_day + process_days.as_u64();
            }
            let referral_interest =
                Self::_check_referral_interest(self, &stake, start_day, final_day);
            referral_link.reward_amount = referral_link.reward_amount + referral_interest;
            Declaration::set_struct_from_key(
                self,
                struct_key0.clone(),
                referral_link.clone().into_bytes().unwrap(),
                DECLARATION_REFERRER_LINK_DICT.to_string(),
            );

            let _: () = BEP20::_mint(self, _referrer, referral_interest);

            emit(&WiseEvents::ReferralCollected {
                staker: referral_link.staker,
                stake_id: referral_link.stake_id,
                referrer: _referrer,
                referrer_id: _referral_id,
                reward_amount: referral_interest,
            });
        }
    }
    fn check_referrals_by_id(
        &self,
        _referrer: Key,
        _referral_id: Vec<u32>,
    ) -> (Key, Vec<u32>, U256, U256, bool, bool, bool, bool) {
        let struct_key: String =
            Declaration::_generate_key_for_dictionary(self, &_referrer, &_referral_id); // generate key and pass it to Declaration contract which will return the struct stored against this key in that contract
        let referrer_link_struct: Vec<u8> = Declaration::get_struct_from_key(
            self,
            struct_key,
            DECLARATION_REFERRER_LINK_DICT.to_string(),
        );
        let referral_link: declaration::structs::ReferrerLink =
            declaration::structs::ReferrerLink::from_bytes(&referrer_link_struct)
                .unwrap()
                .0;
        let _staker: Key = referral_link.staker;
        let _stake_id: Vec<u32> = referral_link.stake_id.clone();
        let is_active_referral: bool = referral_link.is_active;
        let struct_key: String =
            Declaration::_generate_key_for_dictionary(self, &_staker, &_stake_id); // generate key and pass it to Declaration contract which will return the struct stored against this key in that contract
        let stakes_struct: Vec<u8> =
            Declaration::get_struct_from_key(self, struct_key, DECLARATION_STAKES_DICT.to_string());
        let stake: declaration::structs::Stake =
            declaration::structs::Stake::from_bytes(&stakes_struct)
                .unwrap()
                .0; // convert json string received, back to Stake Structure

        let referrer_shares = stake.referrer_shares;
        let start_day: U256 = Self::_determine_start_day(self, &stake, &referral_link);
        let final_day: U256 = Self::_determine_final_day(self, &stake);
        let referral_interest = Self::_check_referral_interest(self, &stake, start_day, final_day);
        let is_active_stake = stake.is_active;

        let is_stake_ended: bool = Helper::stake_ended(self, stake.clone().into_bytes().unwrap());
        let is_ended_stake = is_stake_ended;
        let is_stake_mature: bool =
            Helper::is_mature_stake(self, stake.clone().into_bytes().unwrap());
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
        &self,
        _stake: &declaration::structs::Stake,
        _start_day: U256,
        _final_day: U256,
    ) -> U256 {
        let is_not_critical_mass_referrer: bool =
            Helper::not_critical_mass_referrer(self, _stake.referrer);
        if is_not_critical_mass_referrer {
            return 0.into();
        } else {
            Self::_get_referral_interest(self, _stake, _start_day, _final_day)
        }
    }
    fn _get_referral_interest(
        &self,
        _stake: &declaration::structs::Stake,
        _start_day: U256,
        _final_day: U256,
    ) -> U256 {
        let mut _referral_interest: U256 = 0.into();

        for _day in _start_day.as_u64().._final_day.as_u64() {
            let struct_key: U256 = U256::from(_day);
            let snapshots: Vec<u8> = Snapshot::get_struct_from_key(
                self,
                &struct_key,
                SNAPSHOT_RSNAPSHOTS_DICT.to_string(),
            );
            let snapshots: snapshot::structs::RSnapshot =
                snapshot::structs::RSnapshot::from_bytes(&snapshots)
                    .unwrap()
                    .0;
            let constants: Vec<u8> = Declaration::get_declaration_constants(self);
            let constants: declaration::parameters::ConstantParameters =
                declaration::parameters::ConstantParameters::from_bytes(&constants)
                    .unwrap()
                    .0;
            _referral_interest = _referral_interest
                + _stake.stakes_shares * constants.precision_rate / snapshots.inflation_amount;
        }
        return _referral_interest;
    }
    fn _determine_start_day(
        &self,
        _stake: &declaration::structs::Stake,
        _link: &declaration::structs::ReferrerLink,
    ) -> U256 {
        let struct_key: String = _stake.referrer.to_formatted_string();
        let critical_mass: Vec<u8> = Declaration::get_struct_from_key(
            self,
            struct_key,
            DECLARATION_CRITICAL_MASS_DICT.to_string(),
        );
        let critical_mass: declaration::structs::CriticalMass =
            declaration::structs::CriticalMass::from_bytes(&critical_mass)
                .unwrap()
                .0;

        if critical_mass.activation_day > U256::from(_stake.start_day) {
            return critical_mass.activation_day;
        } else {
            let sum: U256 = U256::from(_stake.start_day) + _link.processed_days;
            return sum;
        }
    }
    fn _determine_final_day(&self, _stake: &declaration::structs::Stake) -> U256 {
        if _stake.close_day > 0 {
            return U256::from(_stake.close_day);
        } else {
            let calculation_day: U256 =
                Helper::calculation_day(self, _stake.clone().into_bytes().unwrap());
            return calculation_day;
        }
    }
    fn convert_to_contract_hash(contract_hash: Key) -> ContractHash {
        let contract_hash_add_array = match contract_hash {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        return ContractHash::new(contract_hash_add_array);
    }
}
