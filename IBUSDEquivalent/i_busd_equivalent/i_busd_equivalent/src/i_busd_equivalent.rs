use crate::data::{self};
use casper_contract::contract_api::runtime;
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},
    runtime_args, Key, RuntimeArgs, U256,
};
use contract_utils::{ContractContext, ContractStorage};

pub trait IBUSDEquivalent<Storage: ContractStorage>: ContractContext<Storage> {
    // Will be called by constructor
    fn init(&mut self, contract_hash: Key, package_hash: ContractPackageHash, busd_hash: Key) {
        data::set_package_hash(package_hash);
        data::set_self_hash(contract_hash);
        data::set_busd_hash(busd_hash);
    }

    fn _get_busd_equivalent(&self) -> U256 {
        let busd_hash = data::busd_hash();

        let ret: U256 = runtime::call_contract(
            Self::_create_hash_from_key(busd_hash),
            "get_busd_equivalent",
            runtime_args! {},
        );

        ret
    }
    // ============== Helper functions ==============================//

    fn _create_hash_from_key(key: Key) -> ContractHash {
        ContractHash::from(key.into_hash().unwrap_or_default())
    }
}
