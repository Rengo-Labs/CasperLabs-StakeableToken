use casper_contract::{ contract_api::{runtime}};
use alloc::{collections::BTreeSet, format, vec::Vec, vec, string::String, string::ToString};
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},Key,  U256, BlockTime, CLType::U64, runtime_args, RuntimeArgs, bytesrepr::ToBytes};
use contract_utils::{ContractContext, ContractStorage};

use renvm_sig::keccak256;


use crate::data::{self};
use crate::config::*;

pub trait Helper<Storage: ContractStorage>: ContractContext<Storage> 
{
    // Will be called by constructor
    fn init(&mut self, contract_hash: Key, package_hash: ContractPackageHash, timing_contract_hash: Key, declaration_contract_hash: Key, globals_contract_hash: Key) 
    {
        data::set_package_hash(package_hash);
        data::set_self_hash(contract_hash);
        data::set_timing_hash(timing_contract_hash);
        data::set_declaration_hash(declaration_contract_hash);
        data::set_globals_hash(globals_contract_hash);
    }

    fn to_bytes16(&self, x: U256) -> Vec<u16>
    {
        let x: Vec<u8> = x.to_bytes().unwrap_or_default();
        let result: Vec<u16> = x.chunks_exact(2).
        into_iter().
        map(|a| u16::from_ne_bytes([a[0], a[1]])).collect();            // Create a native endian integer value
        
        result
    }

    fn generate_id(&self, x: Key, y: U256, z: u8) -> Vec<u32>
    {
        let encoded: String = format!("{}{}{}", x, y, z);
        let hash: [u8; 32] = keccak256(encoded.as_bytes());

        let id_u16: Vec<u16> = hash.chunks_exact(2).
        into_iter().
        map(|a| u16::from_ne_bytes([a[0], a[1]])).collect();            // Create a native endian integer value
        
        let mut id_u32: Vec<u32> = Vec::new();
        for n in id_u16
        {
            id_u32.push(u32::from(n));
        }

        id_u32                                                          // Casper doesnot support u16 therefore returning u32
    }

    fn generate_stake_id(&self, _staker: Key) -> Vec<u32>
    {
        let declaration_hash: Key = data::declaration_hash();
        let args: RuntimeArgs = runtime_args!{
            "staker" => _staker
        };

        let stake_count: U256 = runtime::call_contract(ContractHash::from(declaration_hash.into_hash().unwrap_or_default()), "get_stake_count", args);
        Self::generate_id(self, _staker, stake_count, 0x01)
    }


    fn generate_referral_id(&self, _referrer: Key) -> Vec<u32>
    {
        let declaration_hash: Key = data::declaration_hash();
        let args: RuntimeArgs = runtime_args!{
            "referrer" => _referrer
        };

        let referral_count: U256 = runtime::call_contract(ContractHash::from(declaration_hash.into_hash().unwrap_or_default()), "get_referral_count", args);
        Self::generate_id(self, _referrer, referral_count, 0x02)
    }


    fn generate_liquidity_stake_id(&self, _staker: Key) -> Vec<u32>
    {
        let declaration_hash: Key = data::declaration_hash();
        let args: RuntimeArgs = runtime_args!{
            "staker" => _staker
        };

        let liquidity_stake_count: U256 = runtime::call_contract(ContractHash::from(declaration_hash.into_hash().unwrap_or_default()), "get_liquidity_stake_count", args);
        Self::generate_id(self, _staker, liquidity_stake_count, 0x03)
    }

    fn stakes_pagination(&self, _staker: Key, _offset: U256, _length: U256) -> Vec<Vec<u32>>
    {
        let declaration_hash: Key = data::declaration_hash();
        let args: RuntimeArgs = runtime_args!{
            "staker" => _staker
        };

        let stake_count: U256 = runtime::call_contract(ContractHash::from(declaration_hash.into_hash().unwrap_or_default()), "get_stake_count", args);
        let start: U256 = if _offset > 0.into() && stake_count > _offset { stake_count - _offset} else { stake_count };
        let finish: U256 = if _length > 0.into() && start > _length {  start - _length } else {0.into()};
        let mut i: U256 = 0.into();
        let mut _stakes: Vec<Vec<u32>> = Vec::with_capacity((start - finish).as_usize());                               // bytes16[] - vector of vector<u16>

        for _stake_index in (finish.as_usize()..start.as_usize()).rev()
        {
            let _stake_id: Vec<u32> = Self::generate_id(self, _staker, U256::from(_stake_index), 0x01);             // no need to do _staker_index - 1 because start in this loop already is exclusive and finish is inclusive
            let struct_key: String = Self::_generate_key_for_dictionary(&_staker, &_stake_id);                      // generate key and pass it to Declaration contract which will return the struct stored against this key in that contract
            let stakes_struct: String = runtime::call_contract(ContractHash::from(declaration_hash.into_hash().unwrap_or_default()), "get_struct_from_key", runtime_args!{"key" => struct_key, "struct_name" => Structs::STAKES});
            let stake: Structs::Stake = serde_json::from_str(&stakes_struct).unwrap();                              // convert json string received, back to Stake Structure

            if stake.staked_amount > 0.into()
            {
                _stakes[i.as_usize()] = _stake_id;
                i += 1.into();
            }
        }

        _stakes
    }


    fn referrals_pagination(&self, _referrer: Key, _offset: U256, _length: U256) -> Vec<Vec<u32>>
    {
        let declaration_hash: Key = data::declaration_hash();
        let args: RuntimeArgs = runtime_args!{
            "referrer" => _referrer
        };

        let referrer_count: U256 = runtime::call_contract(ContractHash::from(declaration_hash.into_hash().unwrap_or_default()), "get_referral_count", args);
        let start: U256 = if _offset > 0.into() && referrer_count > _offset { referrer_count - _offset} else { referrer_count };
        let finish: U256 = if _length > 0.into() && start > _length {  start - _length } else {0.into()};
        let mut i: U256 = 0.into();
        let mut _referrers: Vec<Vec<u32>> = Vec::with_capacity((start - finish).as_usize());

        for _referrer_index in (finish.as_usize()..start.as_usize()).rev()
        {
            let _referrer_id: Vec<u32> = Self::generate_id(self, _referrer, U256::from(_referrer_index), 0x01);                 // no need to do _staker_index - 1 because start in this loop already is exclusive and finish is inclusive
            let struct_key: String = Self::_generate_key_for_dictionary(&_referrer, &_referrer_id);                             // generate key and pass it to Declaration contract which will return the struct stored against this key in that contract
            let referrer_link_struct: String = runtime::call_contract(ContractHash::from(declaration_hash.into_hash().unwrap_or_default()), "get_struct_from_key", runtime_args!{"key" => struct_key, "struct_name" => Structs::REFERRER_LINK});
            let referral_link: Structs::ReferrerLink = serde_json::from_str(&referrer_link_struct).unwrap();                                  // convert json string received, back to Stake Structure

            if Self::_non_zero_address(referral_link.staker)
            {
                _referrers[i.as_usize()] = _referrer_id;
                i += 1.into();
            }
        }

        _referrers
    }
    
    fn latest_stake_id(&self, _staker: Key) -> Vec<u32>
    {
        let declaration_hash: Key = data::declaration_hash();
        let stake_count: U256 = runtime::call_contract(ContractHash::from(declaration_hash.into_hash().unwrap_or_default()), "get_stake_count", runtime_args!{ "staker" => _staker});
        let result: Vec<u32> = if stake_count == 0.into() {Vec::new()} else {Self::generate_id(self, _staker, stake_count.checked_sub(1.into()).unwrap_or_default(), 0x01)};
        result
    }

    fn latest_referrer_id(&self, _referrer: Key) -> Vec<u32>
    {
        let declaration_hash: Key = data::declaration_hash();
        let referrer_count: U256 = runtime::call_contract(ContractHash::from(declaration_hash.into_hash().unwrap_or_default()), "get_referral_count", runtime_args!{ "referrer" => _referrer});
        let result: Vec<u32> = if referrer_count == 0.into() {Vec::new()} else {Self::generate_id(self, _referrer, referrer_count.checked_sub(1.into()).unwrap_or_default(), 0x02)};
        result
    }


    fn latest_liquidity_stake_id(&self, _staker: Key) -> Vec<u32>
    {
        let declaration_hash: Key = data::declaration_hash();
        let stake_count: U256 = runtime::call_contract(ContractHash::from(declaration_hash.into_hash().unwrap_or_default()), "get_liquidity_stake_count", runtime_args!{ "staker" => _staker});
        let result: Vec<u32> = if stake_count == 0.into() {Vec::new()} else {Self::generate_id(self, _staker, stake_count.checked_sub(1.into()).unwrap_or_default(), 0x03)};
        result
    }

    // Following function will be used for both safe_transfer as well as safe_transfer_from
    // Incase of safe_transfer, 'from' will be the caller.
    fn transfer_from(&self, token: Key, from: Key, to: Key, value: U256)
    {
        // Token must be approved for router to spend.
        let args: RuntimeArgs = runtime_args!{
            "owner" => from,
            "recipient" => to,
            "amount" => value
        };

        let _:() = runtime::call_contract(ContractHash::from(token.into_hash().unwrap_or_default()), "transfer_from", args);
    }


    fn stake_ended(&self, stake: String) -> bool           // stake struct
    {
        let stake: Structs::Stake = serde_json::from_str(&stake).unwrap();  // get struct from json string
        Self::_stake_ended(stake)
    }

    fn days_diff(&self, start_date: U256, end_date: U256) -> U256 
    {
        Self::_days_diff(start_date, end_date)
    }

    fn is_mature_stake(&self, stake: String) -> bool      // stake struct
    {
        let stake: Structs::Stake = serde_json::from_str(&stake).unwrap();  // get struct from json string
        Self::_is_mature_stake(stake)
    }

    fn not_critical_mass_referrer(&self, referrer: Key) -> bool
    {
        Self::_not_critical_mass_referrer(referrer)
    }

    fn calculation_day(&self, stake: String) -> U256
    {
        let stake: Structs::Stake = serde_json::from_str(&stake).unwrap();  // get struct from json string
        Self::_calculation_day(stake)
    } 

    fn not_past(&self, day: U256) -> bool 
    {
        Self::_not_past(day)
    }

    fn not_future(&self, day: U256) -> bool 
    {
        Self::_not_future(day)
    }


    // ************************ HELPER METHODS ***************************

    fn _increase_stake_count(&self, _staker: Key)
    {
        let declaration_hash: Key = data::declaration_hash();
        let mut stake_count: U256 = runtime::call_contract(ContractHash::from(declaration_hash.into_hash().unwrap_or_default()), "get_stake_count", runtime_args!{ "staker" => _staker});
        stake_count = stake_count + U256::from(1);

        let _:() = runtime::call_contract(ContractHash::from(declaration_hash.into_hash().unwrap_or_default()), "set_stake_count", runtime_args!{ "staker" => _staker, "value" => stake_count});
    }
    
    fn _increase_referral_count(&self, _referrer: Key)
    {
        let declaration_hash: Key = data::declaration_hash();
        let mut referrer_count: U256 = runtime::call_contract(ContractHash::from(declaration_hash.into_hash().unwrap_or_default()), "get_referral_count", runtime_args!{ "referrer" => _referrer});
        referrer_count = referrer_count + U256::from(1);

        let _:() = runtime::call_contract(ContractHash::from(declaration_hash.into_hash().unwrap_or_default()), "set_referral_count", runtime_args!{ "referrer" => _referrer, "value" => referrer_count});
    }
    
    fn _increase_liquidity_stake_count(&self, _staker: Key)
    {
        let declaration_hash: Key = data::declaration_hash();
        let mut stake_count: U256 = runtime::call_contract(ContractHash::from(declaration_hash.into_hash().unwrap_or_default()), "get_liquidity_stake_count", runtime_args!{ "staker" => _staker});
        stake_count = stake_count + U256::from(1);

        let _:() = runtime::call_contract(ContractHash::from(declaration_hash.into_hash().unwrap_or_default()), "set_liquidity_stake_count", runtime_args!{ "staker" => _staker, "value" => stake_count});
    }

    fn _is_mature_stake(_stake: Structs::Stake) -> bool
    {
        if _stake.close_day > 0 {
            return _stake.final_day <= _stake.close_day;
        }
        else {
            let timing_hash: Key = data::timing_hash();
            let _current_wise_day: u64 = runtime::call_contract(ContractHash::from(timing_hash.into_hash().unwrap_or_default()), "_current_wise_day", runtime_args!{});
        
            return _stake.final_day <= _current_wise_day;
        }
    }

    fn _not_critical_mass_referrer(_referrer: Key) -> bool
    {
        let declaration_hash: Key = data::declaration_hash();
        let struct_key: String = _referrer.to_formatted_string();

        let critical_mass: String = runtime::call_contract(ContractHash::from(declaration_hash.into_hash().unwrap_or_default()), "get_struct_from_key", runtime_args!{"key" => struct_key, "struct_name" => Structs::CRITICAL_MASS});
        let critical_mass: Structs::CriticalMass = serde_json::from_str(&critical_mass).unwrap();
        return critical_mass.activation_day == 0.into();
    }

    fn _stake_not_started(_stake: Structs::Stake) -> bool
    {
        if _stake.close_day > 0 {
            return _stake.start_day > _stake.close_day;
        }
        else {
            let timing_hash: Key = data::timing_hash();
            let _current_wise_day: u64 = runtime::call_contract(ContractHash::from(timing_hash.into_hash().unwrap_or_default()), "_current_wise_day", runtime_args!{});
        
            return _stake.start_day > _current_wise_day;
        }
    }

    fn _stake_ended(_stake: Structs::Stake) -> bool
    {
        return _stake.is_active == false || Self::_is_mature_stake(_stake);
    }

    fn _days_left(_stake: Structs::Stake) -> U256
    {
        if _stake.is_active == false {
            return Self::_days_diff(U256::from(_stake.close_day), U256::from(_stake.final_day));
        }
        else {
            let timing_hash: Key = data::timing_hash();
            let _current_wise_day: u64 = runtime::call_contract(ContractHash::from(timing_hash.into_hash().unwrap_or_default()), "_current_wise_day", runtime_args!{});
            return Self::_days_diff(U256::from(_current_wise_day), U256::from(_stake.final_day));
        }
    }

    fn _days_diff(_start_date: U256, _end_date: U256) -> U256 
    {
        return if  _start_date > _end_date {0.into()} else {_end_date.checked_sub(_start_date).unwrap_or_default()};
    }

    fn _calculation_day(_stake: Structs::Stake) -> U256
    {
        let globals_hash: Key = data::globals_hash();
        let current_wise_day: U256 = runtime::call_contract(ContractHash::from(globals_hash.into_hash().unwrap_or_default()), "get_globals", runtime_args!{"field" => Structs::CURRENT_WISE_DAY});

        return if _stake.final_day > current_wise_day.as_u64() {current_wise_day} else {U256::from(_stake.final_day)};
    }

    fn _starting_day(_stake: Structs::Stake) -> U256 
    {
        return if _stake.scrape_day == 0.into() { U256::from(_stake.start_day) } else { U256::from(_stake.scrape_day) } 
    }

    fn _not_future(_day: U256) -> bool 
    {
        let timing_hash: Key = data::timing_hash();
        let _current_wise_day: u64 = runtime::call_contract(ContractHash::from(timing_hash.into_hash().unwrap_or_default()), "_current_wise_day", runtime_args!{});
        
        return _day <= U256::from(_current_wise_day);
    }

    fn _not_past(_day: U256) -> bool 
    {
        let timing_hash: Key = data::timing_hash();
        let _current_wise_day: u64 = runtime::call_contract(ContractHash::from(timing_hash.into_hash().unwrap_or_default()), "_current_wise_day", runtime_args!{});
        
        return _day >= U256::from(_current_wise_day);
    }

    fn _non_zero_address(key: Key) -> bool
    {
        let zero_addr: Key = Key::Hash([0u8;32]);
        return key != zero_addr
    }

    fn _get_lock_days(_stake: Structs::Stake) -> U256
    {
        return if _stake.lock_days > 1 {U256::from(_stake.lock_days - 1)} else {1.into()};
    }

    fn _prepare_path(_token_address: Key, _synthetic_address: Key, _wise_address: Key) -> Vec<Key>
    {
        let mut _path: Vec<Key> = Vec::with_capacity(3);
        _path.push(_token_address);
        _path.push(_synthetic_address);
        _path.push(_wise_address);
        
        _path
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
}