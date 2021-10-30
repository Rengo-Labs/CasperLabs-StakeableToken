extern crate alloc;

use casper_contract::{ contract_api::{runtime}};
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},Key,  U256, BlockTime, CLType::U64, runtime_args, RuntimeArgs};
use contract_utils::{ContractContext, ContractStorage};

use crate::data::{self};
use crate::config::parameters::*;

pub trait Timing<Storage: ContractStorage>: ContractContext<Storage> 
{
    // Will be called by constructor
    fn init(&mut self, contract_hash: Key, package_hash: ContractPackageHash, declaration_contract_hash: Key) 
    {
        data::set_package_hash(package_hash);
        data::set_self_hash(contract_hash);
        data::set_declaration_contract_hash(declaration_contract_hash);

        // get launch_time from declaration contract and set it into the key
        let launch_time: U256 = runtime::call_contract(ContractHash::from(declaration_contract_hash.into_hash().unwrap_or_default()), "launch_time", runtime_args!{});
        data::set_launch_time(launch_time);
    }

    fn current_wise_day(&mut self) -> u64
    {
        let launch_time = data::launch_time();
        if Self::_get_now() >= launch_time { Self::_current_wise_day() } else {0}
    }

    // Helper methods

    fn _current_wise_day() -> u64
    {
        Self::_wise_day_from_stamp(Self::_get_now())
    }

    fn _next_wise_day() -> u64
    {
        Self::_current_wise_day() + 1
    }

    fn _previous_wise_day() -> u64
    {
        Self::_current_wise_day() - 1
    }

    fn _wise_day_from_stamp(_timestamp: U256) -> u64 
    {
        let launch_time = data::launch_time();
        let seconds_in_day = U256::from(SECONDS_IN_DAY);

        ((_timestamp - launch_time) / seconds_in_day).as_u64()
    }

    fn _get_now() -> U256 
    {
        let block_time: u64 = runtime::get_blocktime().into();                  // returns milliseconds since the Unix epoch
        U256::from(block_time) / U256::from(1000)                               // convert milliseconds to seconds
    }
}