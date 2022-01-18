extern crate alloc;
use alloc::{vec, vec::Vec, string::String, string::ToString};
use casper_contract::{ contract_api::{runtime, system}, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},Key, ApiError, U256, U512, runtime_args, RuntimeArgs, URef, bytesrepr::{FromBytes}};
use contract_utils::{ContractContext, ContractStorage};

use crate::config::*;
use crate::data::{self};
use declaration_crate::Declaration;
use staking_token_crate::StakingToken;
use globals_crate::Globals;
use timing_crate::Timing;
use helper_crate::Helper;
use liquidity_token_crate::LiquidityToken;
use referral_token_crate::ReferralToken;
use snapshot_crate::Snapshot;
use bep20_crate::BEP20;
use wise_token_utils::error_codes::ErrorCodes;

pub trait WiseToken<Storage: ContractStorage>: 
    ContractContext<Storage>
    + Declaration<Storage> 
    + StakingToken<Storage>
    + Globals<Storage>
    + Timing<Storage>
    + Helper<Storage>
    + LiquidityToken<Storage>
    + ReferralToken<Storage>
    + Snapshot<Storage>
    + BEP20<Storage>
{
    // Will be called by constructor
    fn init(&mut self, 
        contract_hash: Key, 
        package_hash: ContractPackageHash, 
        synthetic_cspr_address: Key, 
        router_address: Key, 
        launch_time: U256,
        factory_address: Key,
        pair_address: Key,
        liquidity_guard: Key,
        wcspr: Key
    ) 
    {
        data::set_package_hash(package_hash);
        data::set_contract_hash(contract_hash);
        data::set_router_hash(router_address);

        // init all the crates
        Declaration::init(self, launch_time, router_address, factory_address, pair_address, liquidity_guard, synthetic_cspr_address, wcspr);
        StakingToken::init(self);
        Globals::init(self);
        Timing::init(self);
        Helper::init(self);
        ReferralToken::init(self);
        BEP20::init(self, "Wise Token".to_string(), "WISB".to_string());
        LiquidityToken::init(self, synthetic_cspr_address, pair_address, liquidity_guard);
        Snapshot::init(self, synthetic_cspr_address, pair_address, liquidity_guard);

        data::set_transformer_gate_keeper(self.get_caller());
    }
    
    fn set_liquidity_transfomer(&self, immutable_transformer: Key, transformer_purse: URef)
    {
        if !self.only_keeper(Key::from(self.get_caller()))
        {
            runtime::revert(ApiError::User(ErrorCodes::NotKeeper as u16));
        }

        data::set_liquidity_transformer(immutable_transformer);
        data::set_liquidity_transformer_purse(transformer_purse);                       // This purse will be used in extend_lt_auction method
    }

    fn set_busd(&self, equalizer_address: Key)
    {
        if !self.only_keeper(Key::from(self.get_caller()))
        {
            runtime::revert(ApiError::User(ErrorCodes::NotKeeper as u16));
        }

        // set busd_eq in the declaration contract
        Declaration::set_busd_eq(self, equalizer_address);
    }

    fn renounce_keeper(&self)
    {
        if !self.only_keeper(Key::from(self.get_caller()))
        {
            runtime::revert(ApiError::User(ErrorCodes::NotKeeper as u16));
        }

        let zero_addr: Key = Key::Hash([0u8;32]);
        data::set_transformer_gate_keeper(zero_addr);
    }

    // only keeper can call this method
    fn change_keeper(&self, new_keeper: Key)
    {
        if !self.only_keeper(Key::from(self.get_caller()))
        {
            runtime::revert(ApiError::User(ErrorCodes::NotKeeper as u16));
        }
        data::set_transformer_gate_keeper(new_keeper);
    }

    fn mint_supply(&self, investor_address: Key, amount: U256)
    {
        let liquidity_transformer: Key = data::liquidity_transformer();

        if self.get_caller() == liquidity_transformer
        {
            let _ : () = BEP20::_mint(self, investor_address, amount);
        }
        else {
            runtime::revert(ApiError::User(ErrorCodes::NotLiquidityTransformer as u16));
        }
    }

    fn create_stake_with_cspr(&mut self, lock_days: u64, referrer: Key, amount: U256, caller_purse: URef) -> (Vec<u32>, U256 ,Vec<u32>)
    {
        let router_contract: Key = data::router_hash();

        let wcspr: Key = Declaration::get_wcspr(self);
        let scspr: Key = Declaration::get_scspr(self);
        let self_hash: Key = data::contract_hash();

        let path : Vec<Key> = vec![wcspr, scspr, self_hash];

        // get the consts struct from declaration
        let constant_struct: Vec<u8> = Declaration::get_declaration_constants(self);
        let constant_struct: DeclarationConstantParameters = DeclarationConstantParameters::from_bytes(&constant_struct).unwrap().0;

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
        let (stake_id, start_day, referrer_id):(Vec<u32>, U256 ,Vec<u32>) = StakingToken::create_stake(self, amounts[2], lock_days, referrer);
        
        (stake_id, start_day, referrer_id)
    }

    fn create_stake_with_token(&mut self, token_address: Key, token_amount: U256, lock_days: u64, referrer: Key) -> (Vec<u32>, U256 ,Vec<u32>)
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

        let router_address: Key = data::router_hash();
        let wcspr: Key = Declaration::get_wcspr(self);
        let scspr: Key = Declaration::get_scspr(self);
        let path : Vec<Key> = vec![token_address, wcspr, scspr, data::contract_hash()];

        // get the consts struct from declaration
        let constant_struct: Vec<u8> = Declaration::get_declaration_constants(self);
        let constant_struct: DeclarationConstantParameters = DeclarationConstantParameters::from_bytes(&constant_struct).unwrap().0;
        
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

        let (stake_id, start_day, referrer_id):(Vec<u32>, U256 ,Vec<u32>) = StakingToken::create_stake(self, amounts[3], lock_days, referrer);
        (stake_id, start_day, referrer_id)
    }

    fn get_pair_address(&self) -> Key
    {
        Declaration::get_uniswap_pair(self)
    }

    fn get_total_staked(&self) -> U256
    {
        Globals::get_globals(self, String::from("total_staked"))
    }
    
    fn get_liquidity_transformer(&self) -> Key
    {
        data::liquidity_transformer()
    }

    fn get_synthetic_token_address(&self) -> Key
    {
        Declaration::get_scspr(self)
    }
    
    fn extend_lt_auction(&self)
    {
        let sixteen_days_milliseconds: u64 = 1382400000;
        let ten_minutes_milliseconds: u64 = 600000;
        let blocktime_milliseconds: u64 = runtime::get_blocktime().into();
        
        let current_wise_day: u64 = Timing::_current_wise_day(self);
        let mut launch_time_milliseconds: U256 = Declaration::get_launchtime(self);

        if current_wise_day == 15 {
            if launch_time_milliseconds.as_u64() + sixteen_days_milliseconds - blocktime_milliseconds <= ten_minutes_milliseconds {
                let liquidity_transformer_purse: URef = data::liquidity_transformer_purse();
                let new_balance: U512 = system::get_purse_balance(liquidity_transformer_purse).unwrap_or_revert();
                
                let new_balance: U256 = U256::from(new_balance.as_u128());               // convert U512 to U256
                let lt_balance: U256 = Declaration::get_lt_balance(self);
                let ten_cspr: i32 = 10;
                let amount_motes: U256 = U256::from(10 * (ten_cspr.pow(9)));             // 1 cspr = 10^9 motes, therefore 10 cspr = 10 * (10^9)
                
                if new_balance.checked_sub(lt_balance).unwrap_or_revert() >= amount_motes {
                    launch_time_milliseconds = launch_time_milliseconds + 600000;        // add 10 minutes to launch time
                    Declaration::set_lt_balance(self, new_balance);
                    Declaration::set_launchtime(self, launch_time_milliseconds);
                }
            }
        }
        if current_wise_day > 15 {
            Declaration::set_launchtime(self, U256::from(LAUNCH_TIME));
        }
        
    }

    // ************************** Modifiers *************************

    fn only_keeper(&self, sender: Key) -> bool 
    {
        let transformer_gate_keeper = data::transformer_gate_keeper();
        transformer_gate_keeper == sender
    }
    
}