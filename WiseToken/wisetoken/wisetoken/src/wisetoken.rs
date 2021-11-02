extern crate alloc;

use casper_contract::{ contract_api::{runtime}};
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},Key,  U256, BlockTime, CLType::U64, runtime_args, RuntimeArgs};
use contract_utils::{ContractContext, ContractStorage};

use crate::data::{self};

pub trait WiseToken<Storage: ContractStorage>: ContractContext<Storage> 
{
    // Will be called by constructor
    fn init(&mut self, contract_hash: Key, package_hash: ContractPackageHash) 
    {
        data::set_package_hash(package_hash);
        data::set_self_hash(contract_hash);
    }
}