use crate::constants::*;
use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_types::{
    bytesrepr::ToBytes, runtime_args, ContractHash, ContractPackageHash, Key, RuntimeArgs, URef,
    U256, U512,
};
use test_env::{Sender, TestContract, TestEnv};
use stakeable_token_utils::commons::key_names;

pub struct TransferHelperInstance(TestContract);
impl TransferHelperInstance {
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: Sender,
        transfer_invoker: Key,
    ) -> TestContract {
        TestContract::new(
            env,
            CONTRACT_WASM_NAME,
            contract_name,
            sender,
            runtime_args! {
                TRANSFER_INVOKER_RUNTIME_ARG_NAME => transfer_invoker
            },
        )
    }

    pub fn instance(contract: TestContract) -> TransferHelperInstance {
        TransferHelperInstance(contract)
    }

    pub fn proxy(env: &TestEnv, sender: Sender) -> TestContract {
        TestContract::new(
            env,
            PROXY_CONTRACT_WASM_NAME,
            PROXY_CONTRACT_NAME,
            sender,
            runtime_args! {
                // TRANSFER_HELPER_HASH_RUNTIME_ARG_NAME => transfer_helper
            },
        )
    }

    pub fn set_key_by_name(&self, sender: Sender, name: String, key: Key) {
        self.0.call_contract(
            sender,
            "set_transfer_helper",
            runtime_args! {
                NAME_RUNTIME_ARG_NAME=>name,
                KEY_RUNTIME_ARG_NAME => key
            },
        );
    }

    pub fn get_transfer_invoker_address(&self, sender: Sender) {
        self.0.call_contract(
            sender,
            GET_TRANSFER_INVOKER_ADDRESS_ENTRYPOINT_NAME,
            runtime_args! {},
        );
    }

    pub fn forward_funds(&self, sender: Sender, token_address: Key, forward_amount: U256){
        self.0.call_contract(sender, FORWARD_FUNDS_ENTRYPOINT_NAME, runtime_args!{
            TOKEN_ADDRESS_RUNTIME_ARG_NAME=>token_address,
            FORWARD_AMOUNT_RUNTIME_ARG_NAME=>forward_amount
        });
    }

    // ================== ERC20 Methods ==================== //
    pub fn transfer<T: Into<Key>>(&self, sender: Sender, recipient: T, amount: U256) {
        self.0.call_contract(
            sender,
            "transfer",
            runtime_args! {
                "recipient" => recipient.into(),
                "amount" => amount
            },
        );
    }
    pub fn mint<T: Into<Key>>(&self, sender: Sender, to: T, amount: U256) {
        self.0.call_contract(
            sender,
            "mint",
            runtime_args! {
                "to" => to.into(),
                "amount" => amount
            },
        );
    }
    pub fn balance_of<T: Into<Key>>(&self, account: T) -> U256 {
        self.0
            .query_dictionary("balances", key_to_str(&account.into()))
            .unwrap_or_default()
    }

    // ======================================================= //
    pub fn package_hash_result(&self) -> ContractPackageHash {
        self.0.query_named_key(key_names::SELF_PACKAGE_HASH.to_string())
    }

    pub fn contract_hash_result(&self) -> Key {
        self.0.query_named_key(CONTRACT_HASH_KEY_NAME.to_string())
    }

    pub fn self_hash_result(&self) -> Key {
        self.0.query_named_key(key_names::SELF_CONTRACT_HASH.to_string())
    }

    pub fn self_contract_hash_result(&self) -> Key {
        self.0.query_named_key(SELF_CONTRACT_HASH_KEY_NAME.to_string())
    }

    pub fn get_transfer_invoker_address_result(&self) -> Key {
        self.0
            .query_named_key(GET_TRANSFER_INVOKER_ADDRESS_RESULT.to_string())
    }

    pub fn query_transfer_invoker_address(&self) -> Key {
        self.0
            .query_named_key(key_names::TRANSFER_HELPER_TRANSFER_INVOKER.to_string())
    }

    // pub fn get_transfer_helper_address_result(&self) -> Key {
    //     self.0
    //         .query_named_key(TRANSFER_HELPER_ADDRESS_RESULT.to_string())
    // }

    pub fn get_transfer_helper_address(&self) -> Key {
        self.0
            .query_named_key(TRANSFER_HELPER_HASH_KEY_NAME.to_string())
    }

    // pub fn get_erc20_name()->String{

    // }
    // pub fn constructor(&self, sender: Sender, name: &str, symbol: &str) {
    //     self.0.call_contract(
    //         sender,
    //         "constructor",
    //         runtime_args! {
    //             "name" => name,
    //             "symbol" => symbol,
    //         },
    //     );
    // }
}

pub fn key_to_str(key: &Key) -> String {
    match key {
        Key::Account(account) => account.to_string(),
        Key::Hash(package) => hex::encode(package),
        _ => panic!("Unexpected key type"),
    }
}

pub fn keys_to_str(key_a: &Key, key_b: &Key) -> String {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(key_a.to_bytes().unwrap());
    hasher.update(key_b.to_bytes().unwrap());
    let mut ret = [0u8; 32];
    hasher.finalize_variable(|hash| ret.clone_from_slice(hash));
    hex::encode(ret)
}
