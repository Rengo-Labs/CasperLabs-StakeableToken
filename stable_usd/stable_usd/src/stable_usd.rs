use crate::data::{self};
use casper_contract::contract_api::runtime;
use casper_types::{
    contracts::ContractHash, runtime_args, ContractPackageHash, Key, RuntimeArgs, U256,
};
use contract_utils::{ContractContext, ContractStorage};
extern crate alloc;
use alloc::vec::Vec;
pub trait StableUSD<Storage: ContractStorage>: ContractContext<Storage> {
    // Will be called by constructor
    fn init(
        &mut self,
        contract_hash: Key,
        package_hash: ContractPackageHash,
        wise: Key,
        scspr: Key,
        wcspr: Key,
        busd: Key,
        router: Key,
    ) {
        data::set_busd_hash(busd);
        data::set_self_hash(contract_hash);
        data::set_package_hash(package_hash);
        data::set_scspr_hash(scspr);
        data::set_wcspr_hash(wcspr);
        data::set_wise_hash(wise);
        data::set_router_hash(router);
        data::set_decimals(U256::from(9)); // also sets yodas_per_wise
        data::set_path(wise, scspr, wcspr, busd);
        data::set_latest_stable_usd(0.into());
    }

    fn update_stable_usd(&self) {
        let latest_stable_usd: U256 = self._get_stable_usd();
        data::set_latest_stable_usd(latest_stable_usd);
    }

    fn get_stable_usd(&self) -> U256 {
        self._get_stable_usd()
    }
    fn _get_stable_usd(&self) -> U256 {
        let yodas_per_wise: U256 = data::yodas_per_wise();
        let path: Vec<Key> = data::get_path();
        let results: Vec<U256> = runtime::call_contract(
            Self::_create_hash_from_key(data::router_hash()),
            "get_amounts_out",
            runtime_args! {
                "amount_in"=>yodas_per_wise,
                "path"=>path
            },
        );
        if results.len() > 0 {
            return results[3];
        } else {
            return data::latest_stable_usd();
        }
    }
    // ============== Helper functions ==============================//

    fn _create_hash_from_key(key: Key) -> ContractHash {
        ContractHash::from(key.into_hash().unwrap_or_default())
    }
}
