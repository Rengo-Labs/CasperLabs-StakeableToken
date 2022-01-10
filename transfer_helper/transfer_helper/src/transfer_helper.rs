use crate::data::{self};
use casper_contract::contract_api::runtime;
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},
    runtime_args, ApiError, Key, RuntimeArgs, U256,
};
use contract_utils::{ContractContext, ContractStorage};

pub trait TransferHelper<Storage: ContractStorage>: ContractContext<Storage> {
    // Will be called by constructor
    fn init(
        &mut self,
        contract_hash: Key,
        package_hash: ContractPackageHash,
        transfer_invoker: Key,
    ) {
        data::set_package_hash(package_hash);
        data::set_self_hash(contract_hash);
        data::set_transfer_invoker(transfer_invoker);
    }

    fn forward_funds(&self, _token_address: Key, _forward_amount: U256) -> bool {
        // self._only_transfer_invoker();
        let caller: Key = self.get_caller();
        let transfer_invoker: Key = data::transfer_invoker();

        if !caller.eq(&transfer_invoker) {
            runtime::revert(ApiError::NoAccessRights);
        }
        let ret: bool = runtime::call_contract(
            Self::_create_hash_from_key(_token_address),
            "transfer",
            runtime_args! {
                "recipient"=>transfer_invoker,
                "amount"=>_forward_amount
            },
        );

        ret
    }

    fn get_transfer_invoker_address(&self) -> Key {
        let addr: Key = data::transfer_invoker();
        addr
    }
    // ============== Helper functions ==============================//
    fn _only_transfer_invoker(&self) {
        let transfer_invoker = data::transfer_invoker();
        let caller = Key::from(self.get_caller());

        if caller != transfer_invoker {
            runtime::revert(ApiError::NoAccessRights);
        }
    }
    fn _create_hash_from_key(key: Key) -> ContractHash {
        ContractHash::from(key.into_hash().unwrap_or_default())
    }
}
