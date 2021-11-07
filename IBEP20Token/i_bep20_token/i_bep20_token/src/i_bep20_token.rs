use crate::data::{self};
use casper_contract::contract_api::runtime;
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},
    runtime_args, Key, RuntimeArgs, U256,
};
use contract_utils::{ContractContext, ContractStorage};

pub trait IBEP20Token<Storage: ContractStorage>: ContractContext<Storage> {
    // Will be called by constructor
    fn init(
        &mut self,
        contract_hash: Key,
        package_hash: ContractPackageHash,
        bep20_token_hash: Key,
    ) {
        data::set_package_hash(package_hash);
        data::set_self_hash(contract_hash);
        data::set_bep20_hash(bep20_token_hash);
    }

    fn _approve(&self, _spender: Key, _value: U256) -> bool {
        let bep20_hash = data::bep20_hash();
        runtime::call_contract(
            Self::_create_hash_from_key(bep20_hash),
            "approve",
            runtime_args! {"spender"=>_spender, "value"=>_value},
        )
    }

    fn _transfer_from(&self, _from: Key, _to: Key, _value: U256) -> bool {
        let bep20_hash = data::bep20_hash();
        runtime::call_contract(
            Self::_create_hash_from_key(bep20_hash),
            "transfer_from",
            runtime_args! {"from"=>_from,"to"=>_to,  "value"=>_value},
        )
    }
    // ============== Helper functions ==============================//

    fn _create_hash_from_key(key: Key) -> ContractHash {
        ContractHash::from(key.into_hash().unwrap_or_default())
    }
}
