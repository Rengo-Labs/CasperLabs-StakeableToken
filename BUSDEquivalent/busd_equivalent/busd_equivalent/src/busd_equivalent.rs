use crate::data::{self};
use casper_contract::contract_api::runtime;
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},
    runtime_args, Key, RuntimeArgs, U256,
    bytesrepr::{FromBytes, ToBytes}
};
use contract_utils::{ContractContext, ContractStorage};
extern crate alloc;
use crate::config::*;
use alloc::{string::String, vec::Vec};

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
        router: Key,
        declaration: Key,
    ) {
        data::set_package_hash(package_hash);
        data::set_self_hash(contract_hash);
        data::set_busd_hash(busd);
        data::set_sbnb_hash(sbnb);
        data::set_wbnb_hash(wbnb);
        data::set_wise_hash(wise);
        data::set_router_hash(router);
        data::set_decimals(U256::from(18));
        data::set_path(wise, sbnb, wbnb, busd);
        data::set_declaration_hash(declaration);
    }

    fn update_busd_equivalent(&self) {
        let latest_busd_equivalent: U256 = self._get_busd_equivalent();
        data::set_latest_busd_equivalent(latest_busd_equivalent);
    }

    fn get_busd_equivalent(&self) -> U256 {
        self._get_busd_equivalent()
    }
    fn _get_busd_equivalent(&self) -> U256 {
        let declaration_constants_bytes: Vec<u8> = runtime::call_contract(
            Self::_create_hash_from_key(data::declaration_hash()),
            "get_declaration_constants",
            runtime_args! {},
        );
        let declaration_constants: parameters::ConstantParameters =
        parameters::ConstantParameters::from_bytes(&declaration_constants_bytes).unwrap().0;
        let path: Vec<Key> = data::get_path();
        let results: Vec<U256> = runtime::call_contract(
            Self::_create_hash_from_key(data::router_hash()),
            "get_amounts_out",
            runtime_args! {
                "amount_in"=>U256::from(declaration_constants.yodas_per_wise),
                "path"=>path
            },
        );
        if results.len() > 0 {
            return results[3]
        } else {
            return data::latest_busd_equivalent()
        }
    }
    // ============== Helper functions ==============================//

    fn _create_hash_from_key(key: Key) -> ContractHash {
        ContractHash::from(key.into_hash().unwrap_or_default())
    }
}
