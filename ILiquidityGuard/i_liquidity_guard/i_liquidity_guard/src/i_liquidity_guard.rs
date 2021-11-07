use crate::data::{self};
use casper_contract::contract_api::runtime;
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},
    runtime_args,
    Key, RuntimeArgs, U256,
};
use contract_utils::{ContractContext, ContractStorage};

pub trait ILiquidityGuard<Storage: ContractStorage>: ContractContext<Storage> {
    // Will be called by constructor
    fn init(
        &mut self,
        contract_hash: Key,
        package_hash: ContractPackageHash,
        liquidity_guard_contract_hash: Key,
    ) {
        data::set_package_hash(package_hash);
        data::set_self_hash(contract_hash);
        data::set_liquidity_guard_hash(liquidity_guard_contract_hash);
    }

    fn _get_inflation(_amount: U256) -> U256 {
        let liquidity_guard_hash = data::liquidity_guard_hash();

        let ret: U256 = runtime::call_contract(
            Self::_create_hash_from_key(liquidity_guard_hash),
            "get_inflation",
            runtime_args! {
                "amount"=>_amount
            },
        );
        ret
    }

    // ============== Helper functions ==============================//

    fn _create_hash_from_key(key: Key) -> ContractHash {
        ContractHash::from(key.into_hash().unwrap_or_default())
    }
}
