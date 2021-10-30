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
    fn init(&mut self, contract_hash: Key, package_hash: ContractPackageHash, timing_contract_hash: Key, declaration_contract_hash: Key) 
    {
        data::set_package_hash(package_hash);
        data::set_self_hash(contract_hash);
        data::set_timing_hash(timing_contract_hash);
        data::set_declaration_hash(declaration_contract_hash);
    }

    fn to_bytes16(&self, x: U256) -> Vec<u16>
    {
        let x: Vec<u8> = x.to_bytes().unwrap_or_default();
        let result: Vec<u16> = x.chunks_exact(2).
        into_iter().
        map(|a| u16::from_ne_bytes([a[0], a[1]])).collect();            // Create a native endian integer value
        
        result
    }

    fn generate_id(&self, x: Key, y: U256, z: u8) -> Vec<u16>
    {
        let encoded: String = format!("{}{}{}", x, y, z);
        let hash: [u8; 32] = keccak256(encoded.as_bytes());

        let result: Vec<u16> = hash.chunks_exact(2).
        into_iter().
        map(|a| u16::from_ne_bytes([a[0], a[1]])).collect();            // Create a native endian integer value
        
        result
    }

    fn generate_stake_id(&self, _staker: Key) -> Vec<u16>
    {
        let declaration_hash: Key = data::declaration_hash();
        let args: RuntimeArgs = runtime_args!{
            "staker" => _staker
        };

        let stake_count: U256 = runtime::call_contract(ContractHash::from(declaration_hash.into_hash().unwrap_or_default()), "get_stake_count", args);
        Self::generate_id(self, _staker, stake_count, 0x01)
    }


    fn generate_referral_id(&self, _referrer: Key) -> Vec<u16>
    {
        let declaration_hash: Key = data::declaration_hash();
        let args: RuntimeArgs = runtime_args!{
            "referrer" => _referrer
        };

        let referral_count: U256 = runtime::call_contract(ContractHash::from(declaration_hash.into_hash().unwrap_or_default()), "get_referral_count", args);
        Self::generate_id(self, _referrer, referral_count, 0x02)
    }


    fn generate_liquidity_stake_id(&self, _staker: Key) -> Vec<u16>
    {
        let declaration_hash: Key = data::declaration_hash();
        let args: RuntimeArgs = runtime_args!{
            "staker" => _staker
        };

        let liquidity_stake_count: U256 = runtime::call_contract(ContractHash::from(declaration_hash.into_hash().unwrap_or_default()), "get_liquidity_stake_count", args);
        Self::generate_id(self, _staker, liquidity_stake_count, 0x03)
    }

    fn stakes_pagination(&self, _staker: Key, _offset: U256, _length: U256) -> Vec<Vec<u16>>
    {
        let declaration_hash: Key = data::declaration_hash();
        let args: RuntimeArgs = runtime_args!{
            "staker" => _staker
        };

        let stake_count: U256 = runtime::call_contract(ContractHash::from(declaration_hash.into_hash().unwrap_or_default()), "get_stake_count", args);
        let start: U256 = if _offset > 0.into() && stake_count > _offset { stake_count - _offset} else { stake_count };
        let finish: U256 = if _length > 0.into() && start > _length {  start - _length } else {0.into()};
        let mut i: U256 = 0.into();
        let mut _stakes: Vec<Vec<u16>> = Vec::with_capacity((start - finish).as_usize());                               // bytes16[] - vector of vector<u16>

        for _stake_index in (finish.as_usize()..start.as_usize()).rev()
        {
            let _stake_id: Vec<u16> = Self::generate_id(self, _staker, U256::from(_stake_index), 0x01);             // no need to do _staker_index - 1 because start in this loop already is exclusive and finish is inclusive
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


    fn referrals_pagination(&self, _referrer: Key, _offset: U256, _length: U256) -> Vec<Vec<u16>>
    {
        let declaration_hash: Key = data::declaration_hash();
        let args: RuntimeArgs = runtime_args!{
            "referrer" => _referrer
        };

        let referrer_count: U256 = runtime::call_contract(ContractHash::from(declaration_hash.into_hash().unwrap_or_default()), "get_referral_count", args);
        let start: U256 = if _offset > 0.into() && referrer_count > _offset { referrer_count - _offset} else { referrer_count };
        let finish: U256 = if _length > 0.into() && start > _length {  start - _length } else {0.into()};
        let mut i: U256 = 0.into();
        let mut _referrers: Vec<Vec<u16>> = Vec::with_capacity((start - finish).as_usize());

        for _referrer_index in (finish.as_usize()..start.as_usize()).rev()
        {
            let _referrer_id: Vec<u16> = Self::generate_id(self, _referrer, U256::from(_referrer_index), 0x01);                 // no need to do _staker_index - 1 because start in this loop already is exclusive and finish is inclusive
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
    
    fn latest_stake_id(&self, _staker: Key) -> Vec<u16>
    {
        let declaration_hash: Key = data::declaration_hash();
        let stake_count: U256 = runtime::call_contract(ContractHash::from(declaration_hash.into_hash().unwrap_or_default()), "get_stake_count", runtime_args!{ "staker" => _staker});
        let result: Vec<u16> = if stake_count == 0.into() {Vec::new()} else {Self::generate_id(self, _staker, stake_count.checked_sub(1.into()).unwrap_or_default(), 0x01)};
        result
    }

    fn latest_referrer_id(&self, _referrer: Key) -> Vec<u16>
    {
        let declaration_hash: Key = data::declaration_hash();
        let referrer_count: U256 = runtime::call_contract(ContractHash::from(declaration_hash.into_hash().unwrap_or_default()), "get_referral_count", runtime_args!{ "referrer" => _referrer});
        let result: Vec<u16> = if referrer_count == 0.into() {Vec::new()} else {Self::generate_id(self, _referrer, referrer_count.checked_sub(1.into()).unwrap_or_default(), 0x02)};
        result
    }


    fn latest_liquidity_stake_id(&self, _staker: Key) -> Vec<u16>
    {
        let declaration_hash: Key = data::declaration_hash();
        let stake_count: U256 = runtime::call_contract(ContractHash::from(declaration_hash.into_hash().unwrap_or_default()), "get_liquidity_stake_count", runtime_args!{ "staker" => _staker});
        let result: Vec<u16> = if stake_count == 0.into() {Vec::new()} else {Self::generate_id(self, _staker, stake_count.checked_sub(1.into()).unwrap_or_default(), 0x03)};
        result
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

    fn _non_zero_address(key: Key) -> bool
    {
        let zero_addr: Key = Key::Hash([0u8;32]);
        return key != zero_addr
    }

    fn _generate_key_for_dictionary(key: &Key, id: &Vec<u16>) -> String
    {
        let mut result: String = String::from("");
        result.push_str(&key.to_formatted_string());
        result.push_str("-");
        result.push_str(&Self::_convert_vec_to_string(id));

        result
    }

    fn _convert_vec_to_string(data: &Vec<u16>) -> String
    {
        let mut result: String = String::from("");
        for value in data {
            result.push_str(&value.to_string());
        }
        result
    }
}