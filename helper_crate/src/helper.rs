use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use casper_contract::contract_api::runtime;
use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    contracts::ContractHash,
    runtime_args, Key, RuntimeArgs, U256,
};
use contract_utils::{ContractContext, ContractStorage};
use renvm_sig::keccak256;

use declaration_crate::Declaration;
use globals_crate::Globals;
use timing_crate::Timing;
use wise_token_utils::{commons::key_names::*, declaration::structs::*};

pub trait Helper<Storage: ContractStorage>:
    ContractContext<Storage> + Globals<Storage> + Declaration<Storage> + Timing<Storage>
{
    // Will be called by constructor
    fn init(&mut self) {}

    fn to_bytes16(&self, x: U256) -> Vec<u16> {
        let x: Vec<u8> = x.to_bytes().unwrap_or_default();
        let result: Vec<u16> = x
            .chunks_exact(2)
            .into_iter()
            .map(|a| u16::from_ne_bytes([a[0], a[1]]))
            .collect(); // Create a native endian integer value

        result
    }

    fn generate_id(&self, x: Key, y: U256, z: u8) -> Vec<u32> {
        let encoded: String = format!("{}{}{}", x, y, z);
        let hash: [u8; 32] = keccak256(encoded.as_bytes());

        let id_u16: Vec<u16> = hash
            .chunks_exact(2)
            .into_iter()
            .map(|a| u16::from_ne_bytes([a[0], a[1]]))
            .collect(); // Create a native endian integer value

        let mut id_u32: Vec<u32> = Vec::new();
        for n in id_u16 {
            id_u32.push(u32::from(n));
        }

        id_u32 // Casper doesnot support u16 therefore returning u32
    }

    fn generate_stake_id(&self, _staker: Key) -> Vec<u32> {
        let stake_count: U256 = Declaration::get_stake_count(self, _staker);
        Self::generate_id(self, _staker, stake_count, 0x01)
    }

    fn generate_referral_id(&self, _referrer: Key) -> Vec<u32> {
        let referral_count: U256 = Declaration::get_referral_count(self, _referrer);
        Self::generate_id(self, _referrer, referral_count, 0x02)
    }

    fn generate_liquidity_stake_id(&self, _staker: Key) -> Vec<u32> {
        let liquidity_stake_count: U256 = Declaration::get_liquidity_stake_count(self, _staker);
        Self::generate_id(self, _staker, liquidity_stake_count, 0x03)
    }

    fn stakes_pagination(&self, _staker: Key, _offset: U256, _length: U256) -> Vec<Vec<u32>> {
        let stake_count: U256 = Declaration::get_stake_count(self, _staker);
        let start: U256 = if _offset > 0.into() && stake_count > _offset {
            stake_count - _offset
        } else {
            stake_count
        };
        let finish: U256 = if _length > 0.into() && start > _length {
            start - _length
        } else {
            0.into()
        };
        let mut i: U256 = 0.into();
        let mut _stakes: Vec<Vec<u32>> = Vec::with_capacity((start - finish).as_usize()); // bytes16[] - vector of vector<u16>

        for _stake_index in (finish.as_usize()..start.as_usize()).rev() {
            let _stake_id: Vec<u32> =
                Self::generate_id(self, _staker, U256::from(_stake_index), 0x01); // no need to do _staker_index - 1 because start in this loop already is exclusive and finish is inclusive
            let struct_key: String =
                Declaration::_generate_key_for_dictionary(self, &_staker, &_stake_id); // generate key and pass it to Declaration contract which will return the struct stored against this key in that contract
            let stakes_struct: Vec<u8> = Declaration::get_struct_from_key(
                self,
                struct_key,
                DECLARATION_STAKES_DICT.to_string(),
            );
            let stake: Stake = Stake::from_bytes(&stakes_struct).unwrap().0;
            if stake.staked_amount > 0.into() {
                _stakes[i.as_usize()] = _stake_id;
                i += 1.into();
            }
        }

        _stakes
    }

    fn referrals_pagination(&self, _referrer: Key, _offset: U256, _length: U256) -> Vec<Vec<u32>> {
        let referrer_count: U256 = Declaration::get_referral_count(self, _referrer);
        let start: U256 = if _offset > 0.into() && referrer_count > _offset {
            referrer_count - _offset
        } else {
            referrer_count
        };
        let finish: U256 = if _length > 0.into() && start > _length {
            start - _length
        } else {
            0.into()
        };
        let mut i: U256 = 0.into();
        let mut _referrers: Vec<Vec<u32>> = Vec::with_capacity((start - finish).as_usize());

        for _referrer_index in (finish.as_usize()..start.as_usize()).rev() {
            let _referrer_id: Vec<u32> =
                Self::generate_id(self, _referrer, U256::from(_referrer_index), 0x01); // no need to do _staker_index - 1 because start in this loop already is exclusive and finish is inclusive
            let struct_key: String =
                Declaration::_generate_key_for_dictionary(self, &_referrer, &_referrer_id); // generate key and pass it to Declaration contract which will return the struct stored against this key in that contract
            let referrer_link_struct: Vec<u8> = Declaration::get_struct_from_key(
                self,
                struct_key,
                DECLARATION_REFERRER_LINK_DICT.to_string(),
            );
            let referral_link: ReferrerLink =
                ReferrerLink::from_bytes(&referrer_link_struct).unwrap().0; // convert json string received, back to Stake Structure
            if Self::_non_zero_address(referral_link.staker) {
                _referrers[i.as_usize()] = _referrer_id;
                i += 1.into();
            }
        }

        _referrers
    }

    fn latest_stake_id(&self, _staker: Key) -> Vec<u32> {
        let stake_count: U256 = Declaration::get_stake_count(self, _staker);
        let result: Vec<u32> = if stake_count == 0.into() {
            Vec::new()
        } else {
            Self::generate_id(
                self,
                _staker,
                stake_count.checked_sub(1.into()).unwrap_or_default(),
                0x01,
            )
        };
        result
    }

    fn latest_referrer_id(&self, _referrer: Key) -> Vec<u32> {
        let referrer_count: U256 = Declaration::get_referral_count(self, _referrer);
        let result: Vec<u32> = if referrer_count == 0.into() {
            Vec::new()
        } else {
            Self::generate_id(
                self,
                _referrer,
                referrer_count.checked_sub(1.into()).unwrap_or_default(),
                0x02,
            )
        };
        result
    }

    fn latest_liquidity_stake_id(&self, _staker: Key) -> Vec<u32> {
        let stake_count: U256 = Declaration::get_liquidity_stake_count(self, _staker);
        let result: Vec<u32> = if stake_count == 0.into() {
            Vec::new()
        } else {
            Self::generate_id(
                self,
                _staker,
                stake_count.checked_sub(1.into()).unwrap_or_default(),
                0x03,
            )
        };
        result
    }

    // Following function will be used for safe_transfer_from
    fn transfer_from(&self, token: Key, from: Key, to: Key, value: U256) -> Result<(), u32> {
        // Token must be approved for helper to spend.
        let args: RuntimeArgs = runtime_args! {
            "owner" => from,
            "recipient" => to,
            "amount" => value
        };

        let ret: Result<(), u32> = runtime::call_contract(
            ContractHash::from(token.into_hash().unwrap_or_default()),
            "transfer_from",
            args,
        );
        ret
    }

    // Following function will be used for safe_transfer
    fn transfer(&self, token: Key, to: Key, value: U256) -> Result<(), u32> {
        // Token must be approved for helper to spend.
        let args: RuntimeArgs = runtime_args! {
            "recipient" => to,
            "amount" => value
        };

        let ret: Result<(), u32> = runtime::call_contract(
            ContractHash::from(token.into_hash().unwrap_or_default()),
            "transfer",
            args,
        );
        ret
    }
    fn get_lock_days(&self, _stake: Vec<u8>) -> U256 {
        let stake: Stake = Stake::from_bytes(&_stake).unwrap().0;
        Self::_get_lock_days(stake)
    }

    fn stake_ended(&self, stake: Vec<u8>) -> bool // stake struct
    {
        let stake: Stake = Stake::from_bytes(&stake).unwrap().0; // get struct from json string
        Self::_stake_ended(self, stake)
    }

    fn days_diff(&self, start_date: U256, end_date: U256) -> U256 {
        Self::_days_diff(start_date, end_date)
    }

    fn days_left(&self, stake: Vec<u8>) -> U256 {
        let stake: Stake = Stake::from_bytes(&stake).unwrap().0; // get struct from json string
        Self::_days_left(self, stake)
    }

    fn is_mature_stake(&self, stake: Vec<u8>) -> bool // stake struct
    {
        let stake: Stake = Stake::from_bytes(&stake).unwrap().0; // get struct from json string
        Self::_is_mature_stake(self, stake)
    }

    fn not_critical_mass_referrer(&self, referrer: Key) -> bool {
        Self::_not_critical_mass_referrer(self, referrer)
    }

    fn calculation_day(&self, stake: Vec<u8>) -> U256 {
        let stake: Stake = Stake::from_bytes(&stake).unwrap().0; // get struct from json string
        Self::_calculation_day(self, stake)
    }

    fn not_past(&self, day: U256) -> bool {
        Self::_not_past(self, day)
    }

    fn not_future(&self, day: U256) -> bool {
        Self::_not_future(self, day)
    }

    fn starting_day(&self, stake: Vec<u8>) -> U256 {
        let stake: Stake = Stake::from_bytes(&stake).unwrap().0; // get struct from json string
        Self::_starting_day(stake)
    }

    fn increase_liquidity_stake_count(&self, staker: Key) {
        Self::_increase_liquidity_stake_count(self, staker);
    }

    fn stake_not_started(&self, stake: Vec<u8>) -> bool {
        let stake: Stake = Stake::from_bytes(&stake).unwrap().0; // get struct from json string
        Self::_stake_not_started(self, stake)
    }

    // ************************ HELPER METHODS ***************************

    fn _increase_stake_count(&self, _staker: Key) {
        let mut stake_count: U256 = Declaration::get_stake_count(self, _staker);
        stake_count = stake_count + U256::from(1);
        Declaration::set_stake_count(self, _staker, stake_count);
    }

    fn _increase_referral_count(&self, _referrer: Key) {
        let mut referrer_count: U256 = Declaration::get_referral_count(self, _referrer);
        referrer_count = referrer_count + U256::from(1);
        Declaration::set_referral_count(self, _referrer, referrer_count);
    }

    fn _increase_liquidity_stake_count(&self, _staker: Key) {
        let mut stake_count: U256 = Declaration::get_liquidity_stake_count(self, _staker);
        stake_count = stake_count + U256::from(1);
        Declaration::set_liquidity_stake_count(self, _staker, stake_count);
    }

    fn _is_mature_stake(&self, _stake: Stake) -> bool {
        if _stake.close_day > 0 {
            return _stake.final_day <= _stake.close_day;
        } else {
            let _current_wise_day: u64 = Timing::_current_wise_day(self);

            return _stake.final_day <= _current_wise_day;
        }
    }

    fn _not_critical_mass_referrer(&self, _referrer: Key) -> bool {
        let struct_key: String = _referrer.to_formatted_string();

        let critical_mass: Vec<u8> = Declaration::get_struct_from_key(
            self,
            struct_key,
            DECLARATION_CRITICAL_MASS_DICT.to_string(),
        );
        let critical_mass: CriticalMass = CriticalMass::from_bytes(&critical_mass).unwrap().0;
        return critical_mass.activation_day == 0.into();
    }

    fn _stake_not_started(&self, _stake: Stake) -> bool {
        if _stake.close_day > 0 {
            return _stake.start_day > _stake.close_day;
        } else {
            let _current_wise_day: u64 = Timing::_current_wise_day(self);

            return _stake.start_day > _current_wise_day;
        }
    }

    fn _stake_ended(&self, _stake: Stake) -> bool {
        return _stake.is_active == false || Self::_is_mature_stake(self, _stake);
    }

    fn _days_left(&self, _stake: Stake) -> U256 {
        if _stake.is_active == false {
            return Self::_days_diff(U256::from(_stake.close_day), U256::from(_stake.final_day));
        } else {
            let _current_wise_day: u64 = Timing::_current_wise_day(self);
            return Self::_days_diff(U256::from(_current_wise_day), U256::from(_stake.final_day));
        }
    }

    fn _days_diff(_start_date: U256, _end_date: U256) -> U256 {
        return if _start_date > _end_date {
            0.into()
        } else {
            _end_date.checked_sub(_start_date).unwrap_or_default()
        };
    }

    fn _calculation_day(&self, _stake: Stake) -> U256 {
        let current_wise_day: U256 =
            Globals::get_globals(self, GLOBALS_CURRENT_WISE_DAY.to_string());

        return if _stake.final_day > current_wise_day.as_u64() {
            current_wise_day
        } else {
            U256::from(_stake.final_day)
        };
    }

    fn _starting_day(_stake: Stake) -> U256 {
        return if _stake.scrape_day == 0.into() {
            U256::from(_stake.start_day)
        } else {
            U256::from(_stake.scrape_day)
        };
    }

    fn _not_future(&self, _day: U256) -> bool {
        let _current_wise_day: u64 = Timing::_current_wise_day(self);

        return _day <= U256::from(_current_wise_day);
    }

    fn _not_past(&self, _day: U256) -> bool {
        let _current_wise_day: u64 = Timing::_current_wise_day(self);
        return _day >= U256::from(_current_wise_day);
    }

    fn _non_zero_address(key: Key) -> bool {
        let zero_addr: Key = Key::Hash([0u8; 32]);
        return key != zero_addr;
    }

    fn _get_lock_days(_stake: Stake) -> U256 {
        return if _stake.lock_days > 1 {
            U256::from(_stake.lock_days - 1)
        } else {
            1.into()
        };
    }

    fn _prepare_path(_token_address: Key, _synthetic_address: Key, _wise_address: Key) -> Vec<Key> {
        let mut _path: Vec<Key> = Vec::with_capacity(3);
        _path.push(_token_address);
        _path.push(_synthetic_address);
        _path.push(_wise_address);

        _path
    }
}
