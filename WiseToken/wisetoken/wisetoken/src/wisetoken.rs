extern crate alloc;

use casper_contract::{ contract_api::{runtime}, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},Key,  U256, BlockTime, CLType::U64, runtime_args, RuntimeArgs};
use contract_utils::{ContractContext, ContractStorage};

use crate::data::{self};

pub trait WiseToken<Storage: ContractStorage>: ContractContext<Storage> 
{
    // Will be called by constructor
    fn init(&mut self, contract_hash: Key, package_hash: ContractPackageHash, declaration_contract: Key, synthetic_bnb_address: Key, bep20_address: Key) 
    {
        data::set_package_hash(package_hash);
        data::set_self_hash(contract_hash);
        data::set_declaration_hash(declaration_contract);
        data::set_bep20_hash(bep20_address);

        let _: () = runtime::call_contract(ContractHash::from(declaration_contract.into_hash().unwrap_or_revert()), "set_sbnb", runtime_args!{"sbnb" => synthetic_bnb_address});
        data::set_transformer_gate_keeper(self.get_caller());
    }









    // ************************** Helper Methods *************************

    fn only_keeper(sender: Key) -> bool 
    {
        let transformer_gate_keeper = data::transformer_gate_keeper();
        transformer_gate_keeper == sender
    }
}