extern crate alloc;

use casper_contract::{ contract_api::{runtime}, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},Key, ApiError, U256, BlockTime, CLType::U64, runtime_args, RuntimeArgs};
use contract_utils::{ContractContext, ContractStorage};

use crate::config::*;
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

    fn set_liquidity_transfomer(&self, immutable_transformer: Key)
    {
        if !self.only_keeper(Key::from(self.get_caller()))
        {
            runtime::revert(ApiError::User(ErrorCodes::TimedOut as u16));
        }

        data::set_liquidity_transformer(immutable_transformer);
    }

    fn set_busd(&self, equalizer_address: Key)
    {
        if !self.only_keeper(Key::from(self.get_caller()))
        {
            runtime::revert(ApiError::User(ErrorCodes::TimedOut as u16));
        }

        // set busd_eq in the declaration contract
        let declaration_contract: Key = data::declaration_hash();
        let _: () = runtime::call_contract(ContractHash::from(declaration_contract.into_hash().unwrap_or_revert()), "set_busd_eq", runtime_args!{"busd_eq" => equalizer_address});
    }

    fn renounce_keeper(&self)
    {
        if !self.only_keeper(Key::from(self.get_caller()))
        {
            runtime::revert(ApiError::User(ErrorCodes::TimedOut as u16));
        }

        let zero_addr: Key = Key::Hash([0u8;32]);
        data::set_transformer_gate_keeper(zero_addr);
    }

    fn mint_supply(&self, investor_address: Key, amount: U256)
    {
        let liquidity_transformer: Key = data::liquidity_transformer();
        let bep20: Key = data::bep20_hash();

        if self.get_caller() == liquidity_transformer
        {
            // call _mint from BEP20
            let _: () = runtime::call_contract(ContractHash::from(bep20.into_hash().unwrap_or_revert()), "_mint", runtime_args!{/* need to add parameters here*/});
        }
    }




    // ************************** Helper Methods *************************

    fn only_keeper(&self, sender: Key) -> bool 
    {
        let transformer_gate_keeper = data::transformer_gate_keeper();
        transformer_gate_keeper == sender
    }
}