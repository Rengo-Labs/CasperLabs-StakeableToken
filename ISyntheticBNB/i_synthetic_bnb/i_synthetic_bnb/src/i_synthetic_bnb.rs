use crate::data::{self};
use casper_contract::contract_api::runtime;
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},
    runtime_args, Key, RuntimeArgs, U256,
};
use contract_utils::{ContractContext, ContractStorage};

pub trait ISyntheticBNB<Storage: ContractStorage>: ContractContext<Storage> {
    // Will be called by constructor
    fn init(
        &mut self,
        contract_hash: Key,
        package_hash: ContractPackageHash,
        sbnb_contract_hash: Key,
    ) {
        data::set_package_hash(package_hash);
        data::set_self_hash(contract_hash);
        data::set_sbnb_hash(sbnb_contract_hash);
    }

    fn _deposit(&self) {
        let sbnb_hash = data::sbnb_hash();

        let () = runtime::call_contract(
            Self::_create_hash_from_key(sbnb_hash),
            "deposit",
            runtime_args! {},
        );
    }

    fn _approve(&self, _spender: Key, _value: U256) -> bool {
        let sbnb_hash = data::sbnb_hash();
        runtime::call_contract(
            Self::_create_hash_from_key(sbnb_hash),
            "approve",
            runtime_args! {"spender"=>_spender, "value"=>_value},
        )
    }

    fn _transfer_from(&self, _from: Key, _to: Key, _value: U256) -> bool {
        let sbnb_hash = data::sbnb_hash();
        runtime::call_contract(
            Self::_create_hash_from_key(sbnb_hash),
            "transfer_from",
            runtime_args! {"from"=>_from,"to"=>_to,  "value"=>_value},
        )
    }
    // ============== Helper functions ==============================//

    fn _create_hash_from_key(key: Key) -> ContractHash {
        ContractHash::from(key.into_hash().unwrap_or_default())
    }
}
