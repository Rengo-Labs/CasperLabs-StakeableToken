extern crate alloc;
use alloc::{vec, vec::Vec, string::String}; 
use casper_contract::{ contract_api::{runtime, system}, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},Key, ApiError, U256, U512, BlockTime, CLType::U64, runtime_args, RuntimeArgs, URef};
use contract_utils::{ContractContext, ContractStorage};

use crate::config::*;
use crate::data::{self};

pub trait WiseToken<Storage: ContractStorage>: ContractContext<Storage> 
{
    // Will be called by constructor
    fn init(&mut self, contract_hash: Key, package_hash: ContractPackageHash, declaration_contract: Key, globals_contract: Key, synthetic_bnb_address: Key, bep20_address: Key, router_address: Key, staking_token_address: Key, timing_contract: Key) 
    {
        data::set_package_hash(package_hash);
        data::set_self_hash(contract_hash);
        data::set_declaration_hash(declaration_contract);
        data::set_bep20_hash(bep20_address);
        data::set_router_hash(router_address);
        data::set_staking_token_hash(staking_token_address);
        data::set_globals_hash(globals_contract);
        data::set_timing_hash(timing_contract);

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

    fn create_stake_with_bnb(&self, lock_days: u64, referrer: Key, amount: U256, caller_purse: URef) -> (Vec<u32>, U256 ,Vec<u32>)
    {
        let declaration_contract: Key = data::declaration_hash();
        let router_contract: Key = data::router_hash();
        let staking_token_contract: Key = data::staking_token_hash();

        let wbnb: Key = runtime::call_contract(ContractHash::from(declaration_contract.into_hash().unwrap_or_revert()), "get_wbnb", runtime_args![]);
        let sbnb: Key = runtime::call_contract(ContractHash::from(declaration_contract.into_hash().unwrap_or_revert()), "get_sbnb", runtime_args![]);
        let path : Vec<Key> = vec![wbnb, sbnb, self.get_caller()];

        // get the consts struct from declaration
        let constant_struct_json: String = runtime::call_contract(ContractHash::from(declaration_contract.into_hash().unwrap_or_revert()), "get_declaration_constants", runtime_args![]);
        let constant_struct: DeclarationConstantParameters = serde_json::from_str(&constant_struct_json).unwrap();

        let blocktime:u64 = runtime::get_blocktime().into();                    // current blocktime in epoch (milliseconds)
        let two_hours_milliseconds: u64 = 2*((1000*60)*60);
        let deadline: U256 = U256::from(blocktime + two_hours_milliseconds);


        let self_purse = system::create_purse();                    // create new temporary purse and transfer cspr from caller purse to this
        let _:() = system::transfer_from_purse_to_purse(caller_purse, self_purse,  U512::from(amount.as_u128()), None).unwrap_or_revert();


        // call swap method from router
        let args: RuntimeArgs = runtime_args! {
            "amount_out_min" => constant_struct.yodas_per_wise,
            "amount_in" => amount,
            "path" => path.clone(),
            "to" => Key::from(self.get_caller()),
            "deadline" => deadline,
            "purse" => self_purse
        };
        let amounts: Vec<U256> = runtime::call_contract(ContractHash::from(router_contract.into_hash().unwrap_or_revert()), "swap_exact_cspr_for_tokens", args);
        

        // call create_stake
        let args: RuntimeArgs = runtime_args! {
            "staked_amount" => amounts[2],
            "lock_days" => lock_days,
            "referrer" => referrer
        };
        let (stake_id, start_day, referrer_id):(Vec<u32>, U256 ,Vec<u32>) = 
            runtime::call_contract(ContractHash::from(staking_token_contract.into_hash().unwrap_or_revert()), "create_stake", args);
        
        (stake_id, start_day, referrer_id)
    }

    fn create_stake_with_token(&self, token_address: Key, token_amount: U256, lock_days: u64, referrer: Key) -> (Vec<u32>, U256 ,Vec<u32>)
    {
        let args: RuntimeArgs = runtime_args! {
            "sender" => self.get_caller(),
            "recipient" =>  Key::from(data::package_hash()),
            "amount" => token_amount
        };
        let _:() = runtime::call_contract(ContractHash::from(token_address.into_hash().unwrap_or_revert()), "transfer_from", args);

        // Approve the router. First need router's package hash
        let router_contract_hash: Key = data::router_hash();
        let router_package_hash: Key = runtime::call_contract(ContractHash::from(router_contract_hash.into_hash().unwrap_or_revert()), "package_hash", runtime_args![]);    // need to create this method in router. It currently doesnot exist
        
        // Now approve the router packageHash
        let args: RuntimeArgs = runtime_args! {
            "spender"=> router_package_hash, 
            "amount"=> token_amount
        };
        let _:() = runtime::call_contract(ContractHash::from(token_address.into_hash().unwrap_or_revert()), "approve", args);

        let declaration_contract: Key = data::declaration_hash();
        let router_address: Key = data::router_hash();
        let staking_token_contract: Key = data::staking_token_hash();
        let wbnb: Key = runtime::call_contract(ContractHash::from(declaration_contract.into_hash().unwrap_or_revert()), "get_wbnb", runtime_args![]);
        let sbnb: Key = runtime::call_contract(ContractHash::from(declaration_contract.into_hash().unwrap_or_revert()), "get_sbnb", runtime_args![]);
        let path : Vec<Key> = vec![token_address, wbnb, sbnb, self.get_caller()];

        // get the consts struct from declaration
        let constant_struct_json: String = runtime::call_contract(ContractHash::from(declaration_contract.into_hash().unwrap_or_revert()), "get_declaration_constants", runtime_args![]);
        let constant_struct: DeclarationConstantParameters = serde_json::from_str(&constant_struct_json).unwrap();
        
        let blocktime:u64 = runtime::get_blocktime().into();                    // current blocktime in epoch (milliseconds)
        let two_hours_milliseconds: u64 = 2*((1000*60)*60);
        let deadline: U256 = U256::from(blocktime + two_hours_milliseconds);


        let args: RuntimeArgs = runtime_args! {
            "amount_in" => token_amount,
            "amount_out_min" => constant_struct.yodas_per_wise,
            "path" => path,
            "to" => self.get_caller(),
            "deadline" => deadline
        };
    
        let amounts: Vec<U256> =
            runtime::call_contract(ContractHash::from(router_address.into_hash().unwrap_or_revert()), "swap_exact_tokens_for_tokens", args);

        // call create_stake
        let args: RuntimeArgs = runtime_args! {
            "staked_amount" => amounts[3],
            "lock_days" => lock_days,
            "referrer" => referrer
        };
        let (stake_id, start_day, referrer_id):(Vec<u32>, U256 ,Vec<u32>) = 
            runtime::call_contract(ContractHash::from(staking_token_contract.into_hash().unwrap_or_revert()), "create_stake", args);
        
        (stake_id, start_day, referrer_id)
    }

    fn get_pair_address(&self) -> Key
    {
        let declaration_contract: Key = data::declaration_hash();
        let pair: Key = runtime::call_contract(ContractHash::from(declaration_contract.into_hash().unwrap_or_revert()), "get_pancake_pair", runtime_args!{});

        pair
    }

    fn get_total_staked(&self) -> U256
    {
        let globals_contract: Key = data::globals_hash();
        let total_staked: U256 = runtime::call_contract(ContractHash::from(globals_contract.into_hash().unwrap_or_revert()),"get_globals", runtime_args!{"field" => String::from("total_staked")});

        total_staked
    }

    fn get_liquidity_transformer(&self) -> Key
    {
        data::liquidity_transformer()
    }

    fn get_synthetic_token_address(&self) -> Key
    {
        let declaration_contract: Key = data::declaration_hash();
        let sbnb: Key = runtime::call_contract(ContractHash::from(declaration_contract.into_hash().unwrap_or_revert()), "get_sbnb", runtime_args![]);

        sbnb
    }

    fn extend_lt_auction(&self)
    {
        let declaration_contract: Key = data::declaration_hash();
        let timing_contract: Key = data::timing_hash();

        let sixteen_days_milliseconds: u64 = 1382400000;
        let ten_minutes_milliseconds: u64 = 600000;
        let blocktime_milliseconds: u64 = runtime::get_blocktime().into();           
        let current_wise_day: u64 = runtime::call_contract(ContractHash::from(timing_contract.into_hash().unwrap_or_revert()), "_current_wise_day", runtime_args![]);
        let launch_time_milliseconds: U256 = runtime::call_contract(ContractHash::from(declaration_contract.into_hash().unwrap_or_revert()), "get_launchtime", runtime_args![]);

        if current_wise_day == 15 {
            if launch_time_milliseconds.as_u64() + sixteen_days_milliseconds - blocktime_milliseconds <= ten_minutes_milliseconds {

            }
        }
        // if current_wise_day > 15 {
        //     runtime::
        // }
        
    }

    // ************************** Helper Methods *************************

    fn only_keeper(&self, sender: Key) -> bool 
    {
        let transformer_gate_keeper = data::transformer_gate_keeper();
        transformer_gate_keeper == sender
    }
}