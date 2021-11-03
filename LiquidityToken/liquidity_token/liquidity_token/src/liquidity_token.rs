use crate::config::*;
use crate::data::{self};
use alloc::{format, string::String, vec, vec::Vec};
use casper_contract::contract_api::runtime;
use casper_types::{
    bytesrepr::ToBytes,
    contracts::{ContractHash, ContractPackageHash},
    runtime_args, ApiError, Key, RuntimeArgs, U128, U256,
};
use contract_utils::{ContractContext, ContractStorage};

pub trait LiquidityToken<Storage: ContractStorage>: ContractContext<Storage> {
    // Will be called by constructor
    fn init(
        &mut self,
        contract_hash: Key,
        package_hash: ContractPackageHash,
        timing_contract_hash: Key,
        declaration_contract_hash: Key,
        globals_contract_hash: Key,
        helper_contract_hash: Key,
        sbnb_contract_hash: Key,
        pair_contract_hash: Key,
        bep20_contract_hash: Key,
        guard_contract_hash: Key,
        snapshot_contract_hash: Key,
    ) {
        data::set_package_hash(package_hash);
        data::set_self_hash(contract_hash);
        data::set_helper_hash(helper_contract_hash);
        data::set_timing_hash(timing_contract_hash);
        data::set_declaration_hash(declaration_contract_hash);
        data::set_globals_hash(globals_contract_hash);
        data::set_sbnb_hash(sbnb_contract_hash);
        data::set_pair_hash(pair_contract_hash);
        data::set_bep20_hash(bep20_contract_hash);
        data::set_guard_hash(guard_contract_hash);
        data::set_snapshot_hash(snapshot_contract_hash);
    }

    fn to_bytes16(&self, x: U256) -> Vec<u16> {
        let x: Vec<u8> = x.to_bytes().unwrap_or_default();
        let result: Vec<u16> = x
            .chunks_exact(2)
            .into_iter()
            .map(|a| u16::from_ne_bytes([a[0], a[1]]))
            .collect(); // Create a native endian integer value
        result
    }

    fn _create_liquidity_stake(&self, _liquidity_tokens: U256) -> Vec<u16> {
        let snapshot_hash: Key = data::snapshot_hash();
        let helper_hash: Key = data::helper_hash();
        let pair_hash: Key = data::pair_hash();
        let timing_hash: Key = data::timing_hash();
        let globals_hash: Key = data::globals_hash();

        // calling hooks
        let () = runtime::call_contract(
            Self::_create_hash_from_key(snapshot_hash),
            "snapshot_trigger",
            runtime_args! {},
        );
        //

        let declaration_hash: Key = data::declaration_hash();
        let liquidity_guard_status: bool = runtime::call_contract(
            Self::_create_hash_from_key(declaration_hash),
            "get_liquidity_guard_status",
            runtime_args! {},
        );

        // TODO better error handling
        if liquidity_guard_status == true {
            runtime::revert(ApiError::None);
        }

        // VERIFY order of params and their context
        let () = runtime::call_contract(
            Self::_create_hash_from_key(helper_hash),
            "transfer_from",
            runtime_args! {
                "token"=>pair_hash,
                "from"=>self.get_caller(),
                "to"=>data::self_hash(),
                "value"=>_liquidity_tokens

            },
        );

        let liquidity_stake_id: Vec<u8> = runtime::call_contract(
            Self::_create_hash_from_key(helper_hash),
            "generate_liquidity_stake_id",
            runtime_args! {
                "staker"=>self.get_caller()
            },
        );

        let next_wise_day: u64 = runtime::call_contract(
            Self::_create_hash_from_key(timing_hash),
            "next_wise_day",
            runtime_args! {},
        );

        let mut new_liquidity_stake: Structs::LiquidityStake = Structs::LiquidityStake::new();
        new_liquidity_stake.start_day = next_wise_day;
        new_liquidity_stake.staked_amount = _liquidity_tokens;
        new_liquidity_stake.is_active = true;

        let mut liquidity_shares: U256 = runtime::call_contract(
            Self::_create_hash_from_key(globals_hash),
            "get_globals",
            runtime_args! {
                "field"=>data::LIQUIDITY_SHARES
            },
        );
        liquidity_shares = liquidity_shares + _liquidity_tokens;

        let () = runtime::call_contract(
            Self::_create_hash_from_key(globals_hash),
            "set_globals",
            runtime_args! {
                "field"=>data::LIQUIDITY_SHARES,
                "value"=>liquidity_shares
            },
        );

        // VERIFY args name with muzahir's implementation
        let new_liquidity_stake_string: String =
            serde_json::to_string(&new_liquidity_stake).unwrap();
        let () = runtime::call_contract(
            Self::_create_hash_from_key(declaration_hash),
            "set_liquidity_stakes",
            runtime_args! {
                "key"=>self.get_caller(),
                "liquidity_stake_id"=>Vec::clone(&liquidity_stake_id),
                "value"=>new_liquidity_stake_string
            },
        );

        let () = runtime::call_contract(
            Self::_create_hash_from_key(helper_hash),
            "increase_liquidity_stake_count",
            runtime_args! {
                "key"=>self.get_caller()
            },
        );
        Self::_to_bytes16(&Vec::clone(&liquidity_stake_id))
    }
    // ============== Helper functions ==============================//

    // VERIFY this works?
    fn _to_bytes16(value: &Vec<u8>) -> Vec<u16> {
        let temp: Vec<u8> = Vec::clone(value);
        let result: Vec<u16> = temp
            .chunks_exact(2)
            .into_iter()
            .map(|a| u16::from_ne_bytes([a[0], a[1]]))
            .collect(); // Create a native endian integer value
        result
    }
    fn _set_constant_in_contract(contract_hash: Key, constant_name: &str, constant_value: U256) {
        let args: RuntimeArgs = runtime_args! {
            constant_name => constant_value
        };
        let () = runtime::call_contract(
            Self::_create_hash_from_key(contract_hash),
            &format!("set_{}", constant_name),
            args,
        );
    }

    fn _set_liquidity_guard_status(status: bool) {
        let declaration_hash: Key = data::declaration_hash();
        let args: RuntimeArgs = runtime_args! {
            "status"=>status
        };
        let () = runtime::call_contract(
            Self::_create_hash_from_key(declaration_hash),
            "set_liquidity_guard_status",
            args,
        );
    }

    fn _create_hash_from_key(key: Key) -> ContractHash {
        ContractHash::from(key.into_hash().unwrap_or_default())
    }
}
