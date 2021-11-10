use crate::data::{self};
use casper_contract::contract_api::runtime;
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},
    runtime_args, Key, RuntimeArgs, U256,
};
use contract_utils::{ContractContext, ContractStorage};

pub trait BUSDEquivalent<Storage: ContractStorage>: ContractContext<Storage> {
    // Will be called by constructor
    fn init(
        &mut self,
        contract_hash: Key,
        package_hash: ContractPackageHash,
        wise: Key,
        sbnb: Key,
        wbnb: Key,
        busd: Key,
        router: Key
    ) {
        data::set_package_hash(package_hash);
        data::set_self_hash(contract_hash);
        data::set_busd_hash(busd);
        data::set_sbnb_hash(sbnb);
        data::set_wbnb_hash(wbnb);
        data::set_wise_hash(wise);
        data::set_router_hash(router);
        data::set_decimals(U256::from(18));
    }

    fn update_busd_equivalent(&self) {
        let latest_busd_equivalent: U256 = self._get_busd_equivalent();
        data::set_latest_busd_equivalent(latest_busd_equivalent);
    }

    fn get_busd_equivalent(&self) -> U256 {
        self._get_busd_equivalent()
    }
    fn _get_busd_equivalent(&self) -> U256 {
        // remove this later
        U256::from(0)
    }
    // ============== Helper functions ==============================//

    fn _create_hash_from_key(key: Key) -> ContractHash {
        ContractHash::from(key.into_hash().unwrap_or_default())
    }
}
