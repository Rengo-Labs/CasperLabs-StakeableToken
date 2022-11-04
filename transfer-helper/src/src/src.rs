use crate::data::{set_transfer_invoker, transfer_invoker};
use casper_contract::contract_api::runtime;
use casper_types::{runtime_args, Key, RuntimeArgs, U256};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use common::{
    errors::Errors,
    functions::{key_to_hash, set_contract_hash, set_package_hash},
};

pub trait TRANSFERHELPER<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&self, contract_hash: Key, package_hash: Key, transfer_invoker: Key) {
        set_transfer_invoker(transfer_invoker);
        set_contract_hash(contract_hash);
        set_package_hash(package_hash);
    }

    fn only_transfer_invoker(&self) {
        if transfer_invoker() != self.get_caller() {
            runtime::revert(Errors::TransferHelperWrongSender);
        }
    }

    fn forward_funds(&self, token_address: Key, forward_amount: U256) -> bool {
        self.only_transfer_invoker();
        let ret: Result<(), u32> = runtime::call_versioned_contract(
            key_to_hash(token_address, Errors::InvalidHash1),
            None,
            "transfer",
            runtime_args! {
                "recipient" => transfer_invoker(),
                "amount" => forward_amount
            },
        );
        ret.is_ok()
    }

    fn get_transfer_invoker_address(&self) -> Key {
        transfer_invoker()
    }
}
