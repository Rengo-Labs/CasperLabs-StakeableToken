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

    // VERIFY returning Vec<u32> assuming vec<u32> is sent as argument to contract calls and returned from them as standard
    fn _create_liquidity_stake(&self, _liquidity_tokens: U256) -> Vec<u32> {
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
        if liquidity_guard_status == false {
            runtime::revert(ApiError::None);
        }

        // VERIFY args name with entrypoint implementation
        // VERIFY this transfer_from is from bep20 or helper(helper then calls this endpoint from pair)
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

        let liquidity_stake_id: Vec<u32> = runtime::call_contract(
            Self::_create_hash_from_key(helper_hash),
            "generate_liquidity_stake_id",
            runtime_args! {
                "staker"=>self.get_caller()
            },
        );

        // VERIFY args name with entrypoint implementation
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

        let new_liquidity_stake_string: String =
            serde_json::to_string(&new_liquidity_stake).unwrap();

        // VERIFY  args name with entrypoint implementation
        let () = runtime::call_contract(
            Self::_create_hash_from_key(declaration_hash),
            "set_liquidity_stake",
            runtime_args! {
                "staker"=>self.get_caller(),
                "id"=>Vec::clone(&liquidity_stake_id),
                "value"=>new_liquidity_stake_string
            },
        );

        let () = runtime::call_contract(
            Self::_create_hash_from_key(helper_hash),
            "increase_liquidity_stake_count",
            runtime_args! {
                "staker"=>self.get_caller()
            },
        );
        liquidity_stake_id
    }

    fn _check_liquidity_stake_by_id(
        &self,
        _staker: Key,
        _liquidity_stake_id: Vec<u32>,
    ) -> String {
        let declaration_hash = data::declaration_hash();
        let liquidity_stake_string: String = runtime::call_contract(
            Self::_create_hash_from_key(declaration_hash),
            "get_liquidity_stake",
            runtime_args! {
                "staker"=>_staker,
                "id"=>_liquidity_stake_id
            },
        );

        liquidity_stake_string
    }

    fn _end_liquidity_stake(&self, _liquidity_stake_id: Vec<u32>) -> U256 {
        let timing_hash = data::timing_hash();
        let declaration_hash = data::declaration_hash();
        let bep20_hash = data::bep20_hash();
        let pair_hash = data::pair_hash();
        let globals_hash = data::globals_hash();
        let helper_hash = data::helper_hash();

        let mut liquidity_stake_string: String = runtime::call_contract(
            Self::_create_hash_from_key(declaration_hash),
            "get_liquidity_stake",
            runtime_args! {
                "staker"=>self.get_caller(),
                "id"=>Vec::clone(&_liquidity_stake_id)
            },
        );
        let mut liquidity_stake_struct: Structs::LiquidityStake =
            serde_json::from_str(&liquidity_stake_string).unwrap();

        if !liquidity_stake_struct.is_active {
            runtime::revert(ApiError::InvalidDictionaryItemKey)
        }

        let current_wise_day: u64 = runtime::call_contract(
            Self::_create_hash_from_key(timing_hash),
            "current_wise_day",
            runtime_args! {},
        );

        liquidity_stake_struct.is_active = false;
        liquidity_stake_struct.close_day = current_wise_day;
        liquidity_stake_struct.reward_amount =
            self._calculate_reward_amount(&liquidity_stake_struct);

        // VERIFY this end point exists and argument names
        // verify which contract mint is from
        let () = runtime::call_contract(
            Self::_create_hash_from_key(bep20_hash),
            "mint",
            runtime_args! {
                "to"=>self.get_caller(),
                "amount"=>liquidity_stake_struct.reward_amount
            },
        );

        // VERIFY endpoint with implementation
        let () = runtime::call_contract(
            Self::_create_hash_from_key(helper_hash),
            "transfer_from",
            runtime_args! {
                "token"=>pair_hash,
                "to"=>self.get_caller(),
                "from"=>data::self_hash(),
                "value"=>liquidity_stake_struct.reward_amount
            },
        );

        let mut liquidity_shares: U256 = runtime::call_contract(
            Self::_create_hash_from_key(globals_hash),
            "get_globals",
            runtime_args! {
                "field"=>data::LIQUIDITY_SHARES
            },
        );

        liquidity_shares = liquidity_shares - liquidity_stake_struct.staked_amount;

        let () = runtime::call_contract(
            Self::_create_hash_from_key(globals_hash),
            "set_globals",
            runtime_args! {
                "field"=>data::LIQUIDITY_SHARES,
                "valie"=>liquidity_shares
            },
        );

        liquidity_stake_string = serde_json::to_string(&liquidity_stake_struct).unwrap();
        let () = runtime::call_contract(
            Self::_create_hash_from_key(declaration_hash),
            "set_liquidity_stake",
            runtime_args! {
                "staker"=>self.get_caller(),
                "id"=>Vec::clone(&_liquidity_stake_id),
                "value"=>liquidity_stake_string
            },
        );

        liquidity_stake_struct.reward_amount
    }
    fn _calculate_reward_amount(&self, _liquidity_stake: &Structs::LiquidityStake) -> U256 {
        let declaration_hash = data::declaration_hash();
        let snapshot_hash = data::snapshot_hash();
        let globals_hash = data::globals_hash();

        let constant_parameters_string: String = runtime::call_contract(
            Self::_create_hash_from_key(declaration_hash),
            "get_declaration_constants",
            runtime_args! {},
        );
        let constant_parameters_struct: Structs::ConstantParameters =
            serde_json::from_str(&constant_parameters_string).unwrap();

        let max_calculation_day: U256 = U256::from(_liquidity_stake.start_day)
            + U256::from(constant_parameters_struct.min_referral_days);

        let current_wise_day_globals: U256 = runtime::call_contract(
            Self::_create_hash_from_key(globals_hash),
            "get_globals",
            runtime_args! {"field"=>data::CURRENT_WISE_DAY},
        );

        let calculation_day: U256 = if current_wise_day_globals < max_calculation_day {
            current_wise_day_globals
        } else {
            max_calculation_day
        };

        let mut reward_amount: U256 = U256::from(0);
        let mut day: U256 = U256::from(_liquidity_stake.start_day);

        let l_snapshot_string: String = runtime::call_contract(
            Self::_create_hash_from_key(snapshot_hash),
            "get_struct_from_key",
            runtime_args! {
                "struct_name"=>data::LSNAPSHOTS_DICT,
                "key"=>day
            },
        );
        let l_snapshot_struct: Structs::LSnapShot =
            serde_json::from_str(&l_snapshot_string).unwrap();

        while day < calculation_day {
            reward_amount = reward_amount
                + ((U256::from(_liquidity_stake.staked_amount)
                    * constant_parameters_struct.precision_rate)
                    / l_snapshot_struct.inflation_amount);
            day = day + U256::from(1);
        }

        reward_amount
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

    fn _create_hash_from_key(key: Key) -> ContractHash {
        ContractHash::from(key.into_hash().unwrap_or_default())
    }
}
