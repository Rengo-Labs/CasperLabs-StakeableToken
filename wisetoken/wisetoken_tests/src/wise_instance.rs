use casper_types::{runtime_args, ContractHash, ContractPackageHash, Key, RuntimeArgs, U256};
use test_env::{Sender, TestContract, TestEnv};

pub struct WiseTestInstance(pub TestContract);

impl WiseTestInstance {
    pub fn new(env: &TestEnv, wise_address: Key, bep20: Key, sender: Sender) -> WiseTestInstance {
        WiseTestInstance(TestContract::new(
            env,
            "contract.wasm",
            "WiseTest",
            sender,
            runtime_args! {
                "wise_address" => wise_address,
                "bep20_address" => bep20
                // contract_name is passed seperately, so we don't need to pass it here.
            },
        ))
    }

    pub fn balance_of<T: Into<Key>>(&self, token: &TestContract, account: T) -> U256 {
        token
            .query_dictionary("balances", key_to_str(&account.into()))
            .unwrap_or_default()
    }

    pub fn test_contract_package_hash(&self) -> Key {
        let package_hash: ContractPackageHash = self.0.query_named_key("package_hash".to_string());
        Key::from(package_hash)
    }

    pub fn test_contract_hash(&self) -> Key {
        let contract_hash: ContractHash = self.0.query_named_key("self_hash".to_string());
        Key::from(contract_hash)
    }

    pub fn set_liquidity_transfomer(&self, owner: Sender, immutable_transformer: Key)
    {
        self.0.call_contract(
            owner, 
            "set_liquidity_transfomer",
            runtime_args!{
                "immutable_transformer" => immutable_transformer,
            });
    }

    pub fn set_busd(&self, owner: Sender, equalizer_address: Key)
    {
        self.0.call_contract(
            owner, 
            "set_busd",
            runtime_args!{
                "equalizer_address" => equalizer_address
            });
    }

    pub fn renounce_keeper(&self, owner: Sender) 
    {
        self.0.call_contract(
            owner, 
            "renounce_keeper",
            runtime_args!{}
        );
    }

    pub fn mint_supply(&self, owner: Sender, investor_address: Key, amount: U256)
    {
        self.0.call_contract(
            owner, 
            "mint_supply",
            runtime_args!{
                "investor_address" => investor_address,
                "amount" => amount
            }
        );
    }

    pub fn set_balances(&self, owner: Sender, account_owner: Key)
    {
        self.0.call_contract(
            owner, 
            "get_bep20_balance",
            runtime_args!{
                "owner" => account_owner,
            }
        );
    }

    pub fn create_stake_with_bnb(&self, owner: Sender, test_contract_hash: Key, lock_days: u64, referrer: Key, amount: U256)
    {
        self.0.call_contract(
            owner, 
            "create_stake_with_bnb",
            runtime_args!{
                "test_contract_hash" => test_contract_hash,
                "lock_days" => lock_days,
                "referrer" => referrer,
                "amount" => amount,
            }
        );
    }

    pub fn add_liquidity_cspr_to_router(
        &self, 
        owner: Sender, 
        router_address: Key, 
        token: Key, 
        amount_token_desired: U256,
        amount_cspr_desired: U256,
        amount_token_min: U256,
        amount_cspr_min: U256,
        to: Key,
        deadline: U256,
        pair: Option<Key>,
        self_hash: Key
    )
    {
        self.0.call_contract(
            owner, 
            "add_liquidity_cspr_to_router",
            runtime_args!{
                "router_address" => router_address,
                "token" => token,
                "amount_token_desired" => amount_token_desired,
                "amount_cspr_desired" => amount_cspr_desired,
                "amount_token_min" => amount_token_min,
                "amount_cspr_min" => amount_cspr_min,
                "to" => to,
                "deadline" => deadline,
                "pair" => pair,
                "self_hash" => self_hash
            }
        );
    }

    pub fn extend_lt_auction(&self, owner: Sender) {
        self.0.call_contract(owner, "extend_lt_auction", runtime_args!{});
    }

    pub fn add_liquidity_to_router(
        &self, 
        owner: Sender, 
        router_address: Key, 
        token_a: Key,
        token_b: Key,
        amount_a_desired: U256,
        amount_b_desired: U256,
        amount_a_min: U256,
        amount_b_min: U256,
        to: Key,
        deadline: U256,
        pair: Option<Key>
    )
    {
        self.0.call_contract(
            owner, 
            "router_add_liquidity",
            runtime_args!{
                "router_address" => router_address,
                "token_a" => token_a,
                "token_b" => token_b,
                "amount_a_desired" => amount_a_desired,
                "amount_b_desired" => amount_b_desired,
                "amount_a_min" => amount_a_min,
                "amount_b_min" => amount_b_min,
                "to" => to,
                "deadline" => deadline,
                "pair" => pair
            }
        );
    }

}

pub fn key_to_str(key: &Key) -> String {
    match key {
        Key::Account(account) => account.to_string(),
        Key::Hash(package) => hex::encode(package),
        _ => panic!("Unexpected key type"),
    }
}
