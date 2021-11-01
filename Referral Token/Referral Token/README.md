# CasperLabs ORACLIZE

Implementation of the ORACLIZE Contract for the Casper platform.

## Usage
### Install
Make sure `wasm32-unknown-unknown` is installed.
```
make prepare
```

It's also recommended to have [wasm-strip](https://github.com/WebAssembly/wabt)
available in your PATH to reduce the size of compiled Wasm.

### Build Smart Contract
```
make build-contract
```

### Test
Test logic and smart contract.
```
make test
```

## Repository overview

### ORACLIZE

The `oraclize` crate contains the implementation of the ORACLIZE Contract and ERC20 standard.

#### ORACLIZE as library
It can be used as a library to create oraclizes of two erc20 tokens and it can be used to build custom tokens. The code structure allows for easy entry points extensions and overrides.


##### Entry Point override example
The following code shows how to override the `transfer` method to alwasy mint
one additional token for a sender. 

```rust
#[derive(Default)]
struct Oraclize(ContractStorage);

impl ContractContext for Oraclize {
    fn storage(&self) -> &ContractStorage {
        &self.0
    }
}

impl ORACLIZE for Oraclize {}

impl Oraclize {
    fn constructor(&mut self, name: String, symbol: String, decimals: u8, initial_supply: U256, nonce:U256, domain_separator: String, permit_type_hash: String, contract_hash: ContractHash) {
        ORACLIZE::init(self, name, symbol, decimals, domain_separator, permit_type_hash, Key::from(contract_hash));
        ORACLIZE::mint(self, self.get_caller(), initial_supply);
        ORACLIZE::set_nonce(self, self.get_caller(), nonce);
    }
}
```

#### ORACLIZE Vanilla Contract
The library comes with a vanilla implementation of the ORACLIZE contract and ERC20 contract that is
ready to use. It is implemented in `oraclize/bin/oraclize_token.rs` and after 
compilation the `oraclize-main.wasm` file is produced.

### ORACLIZE Tests
The `oraclize-tests` crate implements multiple integration test scenarios that
check the compatibility with the ERC20 standard.

Tests provide the `ERC20Instance` struct that can be reused in larger smart
contract projects with multiple ERC20 tokens and other smart contracts
to interact with the instance of an ERC20 token.

Tests are implemented in `oraclize-tests/src/oraclize_tests.rs`.

### Utils

The repository contains 2 utility crates:

* `utils/test-env`
* `utils/contract-utils`

The utility code after review and adoption should be moved to a separate repo
and eventually be added to `casper-contract` and `casper-engine-test-support`.

#### Test Env Crate
`utils/test-env` is a small library written on top of 
`casper-engine-test-support`. It provides two structs:

* `TestEnv` wraps `TestContext` and provides user accounts with initial
  CSPR balances. It is implemented using `Arc<Mutex<...>>` so it can
  be copied, especial between `TestContract` instances.
* `TestContract` wraps an instance of `TestEnv` and simplifies calling
  contracts and reading named keys and dictionaries.

##### Test Example
```rust
struct Token(TestContract);

impl Token {
    pub fn new(env: &TestEnv, sender: Sender) -> Token {
        Token(TestContract::new(env, "token.wasm", "token_name", sender, runtime_args! {
            "initial_supply" => U256::from(1000)
        }))
    }

    pub fn transfer(&self, sender: Sender, recipient: AccountHash, amount: u64) {
        self.0.call_contract(
            sender,
            "transfer",
            runtime_args! {
                "recipient" => recipient,
                "amount" => amount
            },
        );
    }

    pub fn balance_of(&self, account: AccountHash) -> u64 {
        self.0
            .query_dictionary("balances", account.to_string())
            .unwrap_or_default()
    }
}

#[test]
fn test_multiple_tokens() {
    // Prepare the env and users.
    let env = TestEnv::new();
    let user1 = env.next_user();
    let user2 = env.next_user();
    
    // Deploy multiple instances of the same contract
    // agains a single virtual machine.
    let token1 = Token::new(&env, Sender(user1));
    let token2 = Token::new(&env, Sender(user2));

    // Transfer tokens.
    let amount = 10;
    token1.transfer(Sender(user1), user2, amount);
    assert_eq!(token1.balance_of(user1), amount);
}
```

#### Contract Utils Crate
`utils/contract-utils` contains common building blocks for writing smart
contracts:
* `contract_context.rs` provides the `ContractContext` trait that has 
  `get_caller` and `self_addr` methods.
* `data.rs` provides helper methods to work with dictionaries and named
  keys.
* `admin_control.rs` provides the `AdminControl` trait to support admin
  list functionality.

