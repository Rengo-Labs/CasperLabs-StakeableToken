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
        stakeable: Key,
        scspr: Key,
        wcspr: Key,
        stable_usd: Key,
        router: Key,
    ) {
        data::set_stable_usd_hash(stable_usd);
        data::set_self_hash(contract_hash);
        data::set_package_hash(package_hash);
        data::set_scspr_hash(scspr);
        data::set_wcspr_hash(wcspr);
        data::set_stakeable_hash(stakeable);
        data::set_router_hash(router);
        data::set_decimals(U256::from(9)); // also sets yodas_per_stakeable
        data::set_path(stakeable, scspr, wcspr, stable_usd);
        data::set_latest_stable_usd_equivalent(0.into());
    }

    fn update_stable_usd_equivalent(&self) {
        let latest_stable_usd_equivalent: U256 = self._get_stable_usd_equivalent();
        data::set_latest_stable_usd_equivalent(latest_stable_usd_equivalent);
    }

    fn get_stable_usd_equivalent(&self) -> U256 {
        self._get_stable_usd_equivalent()
    }
    fn _get_stable_usd_equivalent(&self) -> U256 {
        let yodas_per_stakeable: U256 = data::yodas_per_stakeable();
        let path: Vec<Key> = data::get_path();
        let results: Vec<U256> = runtime::call_contract(
            Self::_create_hash_from_key(data::router_hash()),
            "get_amounts_out",
            runtime_args! {
                "amount_in"=>yodas_per_stakeable,
                "path"=>path
            },
        );
        if results.len() > 0 {
            return results[3];
        } else {
            return data::latest_stable_usd_equivalent();
        }
    }
    // ============== Helper functions ==============================//

    fn _create_hash_from_key(key: Key) -> ContractHash {
        ContractHash::from(key.into_hash().unwrap_or_default())
    }
}
