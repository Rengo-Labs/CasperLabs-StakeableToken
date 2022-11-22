# CasperLabs-Stakeable-StakeableToken

Implementation of `Contract Crates`, `Liquidity Guard` and `Stakeable Token` for CasperLabs Blockchain.

### Interacting with the contract

You need to have `casper-client` and `jq` installed on your system to run the examples. The instructions have been tested on Ubuntu 20.04.0 LTS.

### Install the prerequisites

You can install the required software by issuing the following commands. If you are on an up-to-date Casper node, you probably already have all of the prerequisites installed so you can skip this step.

#### Note: If any command fails try again by restarting the terminal to reset the enviornment variable.

### Update package repositories

```
sudo apt update
```

### Install the command-line JSON processor

```
sudo apt install jq -y
```

### Install rust

Choose cutomize intallation to install nightly version
Install the nightly version (by default stable toolchain is installed)

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```
rustup install nightly-2022-08-29
```

### Check that nightly toolchain version is installed(this will list stable and nightly versions)

```
rustup toolchain list
```

### Set rust nightly as default

```
rustup default nightly-2022-08-29-x86_64-unknown-linux-gnu
```

### Install wasm32-unknown-unknown

```
rustup target add wasm32-unknown-unknown
```

### Rust Version

```
rustup --version
```

### Install Cmake

```
sudo apt-get -y install cmake
```

Note:https://cgold.readthedocs.io/en/latest/first-step/installation.html

### check if cmake is installed properly

```
cmake --version
```

### Install the Casper Crates

```
cargo install cargo-casper
```

### Add Casper repository

```
echo "deb https://repo.casperlabs.io/releases" bionic main | sudo tee -a /etc/apt/sources.list.d/casper.list
```

```
curl -O https://repo.casperlabs.io/casper-repo-pubkey.asc
```

```
sudo apt-key add casper-repo-pubkey.asc
```

```
sudo apt update
```

```
sudo apt install libssl-dev
```

```
sudo apt install pkg-config
```

### Install the Casper client software

```
cargo +nightly-2022-08-29-x86_64-unknown-linux-gnu install casper-client
```

### To check Casper Client Version

```
casper-client --version
```

# Additonal commands for help

```
casper-client --help
casper-client <command> --help
```

### Generate the keys

```
casper-client keygen keys
```

### Fund the key

The keys can be funded from casper live website [testnet faucet](https://testnet.cspr.live/tools/faucet). Requires chrome browser and the casper signer extension. You should import the keys that were generated in the previous step

## Deployment

A handy script is being created to deploy all required dependencies sequentially for stakeable token contract deployment.

Note:- Following are the prerequisites to be considered before executing the deploy command

- Make sure 'Node' is installed in your machine _(use node version 16.14.0 for avoiding unknown errors)_
- Latest wasms should be in the folder 'script/wasm' _(related wasms already placed for ease)_
- Your deployemnt keys should be in a new folder named 'keys' at 'script/keys'

### Deploy stakeable contract

Use the following command to execute the deployments

```
make deploy
```

After deployments, hashes can be found in the folder 'script/hashes'

## Install

Make sure `wasm32-unknown-unknown` is installed.

```
make prepare
```

It's also recommended to have [wasm-strip](https://github.com/WebAssembly/wabt)
available in your PATH to reduce the size of compiled Wasm.

### Build Dependencies of Contract

Run this command to build dependencies of contracts.

```
make build-dependencies
```

### Individual Run Contracts

Run this command to build & test specific test cases.

```
make run-stakeable-token
make run-liquidity-guard
```

### Run All Contracts

Run this command to build & test all contracts.

```
make run-all
```

### Build Individual Smart Contract

Run this command to build Smart Contracts individually.

```
make build-stakeable-token
make build-liquidity-guard
```

### Build All Smart Contracts

Run this command to build all Smart Contract.

```
make build-all
```

### Individual Test Cases

Run this command to run specific test cases.

```
make test-stakeable-token
make test-liquidity-guard
```

### All Test Cases

Run this command to run all contract's Test Cases.

```
make test-all
```

### Liquidity Guard <a id="liquidity-guard"></a>

### Entry Point methods <a id="liquidity-guard-entry-point-methods"></a>

Following are the Liquidity Guard's entry point methods.

- #### get_inflation <a id="liquidity-guard-get-inflation"></a>

  Returns the inflation calculated at a certain amount.

  | Parameter Name | Type |
  | -------------- | ---- |
  | amount         | u32  |

  This method **returns** U256.

- #### assign_inflation <a id="liquidity-guard-assign-inflation"></a>

  Assigns inflation as set in contract to inflations dictionary.
  <br> Contract reverts if inflation is assigned already.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

### Stakable Token <a id="stakeable-token"></a>

### Entry Point methods

<a id="stakeable-token-entry-point-methods"></a>

Following are the Stakeable Token's entry point methods.

- #### set_liquidity_transfomer

  <a id="stakeable-token-set-liquidity-transfomer"></a>
  Set Liquidity Transformer's hash and it's purse's uref to stakeable token contract global state.

  | Parameter Name        | Type |
  | --------------------- | ---- |
  | immutable_transformer | Key  |
  | transformer_purse     | URef |

  This method **returns** nothing.

- #### get_transformer_gate_keeper

  <a id="stakeable-token-get-transformer-gate-keeper"></a>
  Return the transformer gate keeper.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- #### mint_supply <a id="stakeable-token-mint-supply"></a>

  Mints tokens to an address. Contract reverts if `self.get_caller()` is not the Liquidity Transformer contract.

  | Parameter Name   | Type |
  | ---------------- | ---- |
  | investor_address | Key  |
  | amount           | U256 |

  This method **returns** nothing.

- #### create_stake_with_cspr <a id="stakeable-token-create-stake-with-cspr"></a>

  Creates a stake by withdrawing a cspr amount from a provided purse.
  | Parameter Name | Type |
  | ---------------- | ---- |
  | referrer | Key |
  | amount | U256 |
  | lock_days | u64 |
  | purse | URef |

  This method **returns** a tuple of order 3, described below.
  | Index | Item Name | Type |
  | ----- | ----------- | ---------- |
  | 0 | stake_id | `Vec<u32>` |
  | 1 | start_day | U256 |
  | 2 | referrer_id | `Vec<u32>` |

- #### get_liquidity_rate <a id="stakeable-token-get-liquidity-rate"></a>

  Return the liquidity rate.
  Parameter Name | Type
  |---|--- |

  This method **returns** `U256`

- #### get_scspr <a id="stakeable-token-get-scspr"></a>

  Return the scspr.
  Parameter Name | Type
  |---|--- |

  This method **returns** `Key`

- #### get_uniswap_pair <a id="stakeable-token-get-uniswap-pair"></a>

  Return the uniswap pair.
  Parameter Name | Type
  |---|--- |

  This method **returns** `Key`

- #### get_inflation_rate <a id="stakeable-token-get-inflation-rate"></a>

  Return the inflation rate.
  Parameter Name | Type
  |---|--- |

  This method **returns** `U256`

- #### create_pair <a id="stakeable-token-create-pair"></a>

  This function is used to create the pair.
  Parameter Name | Type
  |---|--- |

  This method **returns** nothing

- #### get_scheduled_to_end <a id="stakeable-token-get-scheduled-to-end"></a>

  Return the value of scheduled to end.
  Parameter Name | Type
  |---|--- |
  |key|U256 |

  This method **returns** `U256`

- #### get_total_penalties <a id="stakeable-token-get-total-penalties"></a>

  Return the total penalties.
  Parameter Name | Type
  |---|--- |
  |key|U256 |

  This method **returns** `U256`

- #### get_scrapes <a id="stakeable-token-get-scrapes"></a>

  Return the value of scrapes.
  Parameter Name | Type
  |---|--- |
  |key0|Key |
  |key1|Vec\<u32> |

  This method **returns** `U256`

- #### get_stake_count <a id="stakeable-token-get-stake-count"></a>

  Return the count of stake.
  Parameter Name | Type
  |---|--- |
  |staker|Key |

  This method **returns** `U256`

- #### get_referral_count <a id="stakeable-token-get-referral-count"></a>

  Return the count of referrals.
  Parameter Name | Type
  |---|--- |
  |referral|Key |

  This method **returns** `U256`

- #### get_liquidity_stake_count <a id="stakeable-token-get-liquidity-stake-count"></a>

  Return the count of liquidity stake.
  Parameter Name | Type
  |---|--- |
  |staker|Key |

  This method **returns** `U256`

- #### get_referral_shares_to_end <a id="stakeable-token-get-referral-shares-to-end"></a>

  Return the value of referral shares to end.
  Parameter Name | Type
  |---|--- |
  |key|U256 |

  This method **returns** `U256`

- #### snapshots <a id="stakeable-token-snapshots"></a>

  Return the snapshots.
  Parameter Name | Type
  |---|--- |
  |key|U256 |

  This method **returns** `Vec<Snapshot>`

- #### rsnapshots <a id="stakeable-token-rsnapshots"></a>

  Return the rsnapshots.
  Parameter Name | Type
  |---|--- |
  |key|U256 |

  This method **returns** `Vec<RSnapshot>`

- #### lsnapshots <a id="stakeable-token-lsnapshots"></a>

  Return the lsnapshots.
  Parameter Name | Type
  |---|--- |
  |key|U256 |

  This method **returns** `Vec<LSnapshot>`

- #### check_stake_by_id <a id="stakeable-token-check-stake-by-id"></a>

  Return Vec`<String>`.

  Following is the table of parameters.

  | Parameter Name | Type      |
  | -------------- | --------- |
  | stake_id       | Vec\<u32> |
  | staker         | Key       |

  This method **returns** `Vec<String>`

- #### generate_id <a id="stakeable-token-generate-id"></a>

  This function is used to generate the id.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | x              | Key  |
  | y              | U256 |
  | z              | U8   |

  This method **returns** `Vec<u32>`

- #### stakes_pagination <a id="stakeable-token-stakes-pagination"></a>

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | staker         | Key  |
  | length         | U256 |
  | offset         | U256 |

  This method **returns** `Vec<Vec<u32>>`

- #### referrals_pagination <a id="stakeable-token-referrals-pagination"></a>

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | referrer       | Key  |
  | length         | U256 |
  | offset         | U256 |

  This method **returns** `Vec<Vec<u32>>`

- #### latest_stake_id <a id="stakeable-token-latest-stake-id"></a>

  This function returns the latest stake id.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | staker         | Key  |

  This method **returns** `Vec<u32>`

- #### latest_referral_id <a id="stakeable-token-latest-referral-id"></a>

  This function returns the latest referral id.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | staker         | Key  |

  This method **returns** `Vec<u32>`

- #### latest_liquidity_stake_id <a id="stakeable-token-latest-liquidity-stake-id"></a>

  This function returns the latest referral id.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | staker         | Key  |

  This method **returns** `Vec<u32>`

- #### scrape_interest <a id="stakeable-token-scrape-interest"></a>

  Allows to scrape interest from active stake

  Following is the table of parameters.

  | Parameter Name | Type       |
  | -------------- | ---------- |
  | stake_id       | `Vec<u32>` |
  | scrape_days    | u64        |

  This method **returns** `Vec<String>`

- #### check_referrals_by_id <a id="stakeable-token-check-referrals-by-id"></a>

  Calculates rewards and shares for a referrer on a partical referral, and returns all information as a serialized struct.

  Following is the table of parameters.

  | Parameter Name | Type      |
  | -------------- | --------- |
  | referreral_ids | Vec\<u32> |
  | referrer       | Key       |

  This method **returns** `Vec<String>`.

- #### current_stakeable_day <a id="stakeable-token-current-stakeable-day"></a>

  Returns the day since launch of stakeable token.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** u64.

- #### liquidity_guard_trigger <a id="stakeable-token-liquidity-guard-trigger"></a>

  Enables the liquidity guard if it is disabled.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- #### manual_daily_snapshot <a id="stakeable-token-manual-daily-snapshot"></a>

  Call the function of manual daily snapshot.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** nothing.

- #### manual_daily_snapshot_point <a id="stakeable-token-manual-daily-snapshot-point"></a>

  Creates a snapshot from `update_day` till the current stakeable day.

  Following is the table of parameters.

  |Parameter Name | Type
  |update_day|u64

  This method **returns** nothing.

- #### get_stable_usd_equivalent <a id="stakeable-token-get-stable-usd-equivalent"></a>

  Returns the value of stable usd.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `U256`.

- #### referrer_interest <a id="stakeable-token-referrer-interest"></a>

  Returns the calculated interest on a particular referral for `scrape_days` duration and mints equivalend WISE tokens to `self.get_caller()`

  Following is the table of parameters.

  | Parameter Name | Type      |
  | -------------- | --------- |
  | referreral_id  | Vec\<u32> |
  | scrape_days    | U256      |

  This method **returns** nothing.

- #### referrer_interest_bulk <a id="stakeable-token-referrer-interest-bulk"></a>

  Returns the calculated interest on a several referrals, each for for several `scrape_days` duration and mints equivalend WISE tokens to `self.get_caller()` in each case.

  Following is the table of parameters.

  | Parameter Name | Type            |
  | -------------- | --------------- |
  | referreral_ids | Vec\<Vec\<u32>> |
  | scrape_days    | Vec\<U256>      |

  This method **returns** nothing.

- #### create_stake_bulk <a id="stakeable-token-create-stake-bulk"></a>

  Creates several stakes for `self.get_caller()` each with a referrer.

  Following is the table of parameters.

  | Parameter Name | Type         |
  | -------------- | ------------ |
  | staked_amount  | Vec\<U256>   |
  | lock_days      | Vec\<u64>    |
  | referrer       | Vec\<String> |

  This method **returns** nothing.

- #### create_stake <a id="stakeable-token-create-stake"></a>

  Creates a stake for `self.get_caller()` with a referrer.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | staked_amount  | U256 |
  | lock_days      | u64  |
  | referrer       | Key  |

  This method **returns** a tupe of order 3 described below.

  | Tuple Index | Item Name   | Type      |
  | ----------- | ----------- | --------- |
  | 0           | stake_id    | Vec\<u32> |
  | 1           | start_day   | U256      |
  | 2           | referrer_id | Vec\<u32> |

- #### end_stake <a id="stakeable-token-end-stake"></a>

  Ends a stakes of given `stake_id` having been created by `self.get_caller()`.

  Following is the table of parameters.

  | Parameter Name | Type      |
  | -------------- | --------- |
  | stake_id       | Vec\<u32> |

  This method **returns** `U256`.

- #### check_mature_stake <a id="stakeable-token-check-mature-stake"></a>

  Retrns true if a stake of `stake_id` created by a `staker` has matured.

  Following is the table of parameters.

  | Parameter Name | Type      |
  | -------------- | --------- |
  | stake_id       | Vec\<u32> |
  | staker         | Key       |

  This method **returns** Bool

- #### create_liquidity_stake <a id="stakeable-token-create-liquidity-stake"></a>

  Creates a liquidity stake for `self.get_caller()` staking `liquidity_token` of token amount.

  Following is the table of parameters.

  | Parameter Name   | Type |
  | ---------------- | ---- |
  | liquidity_tokens | U256 |

  This method **returns** Vec<u32> type of stake id.

- #### end_liquidity_stake <a id="stakeable-token-end-liquidity-stake"></a>

  End a liquidity stake for `self.get_caller()` having.

  Following is the table of parameters.

  | Parameter Name     | Type      |
  | ------------------ | --------- |
  | liquidity_stake_id | Vec\<U32> |

  This method **returns** `U256`.

- #### check_liquidity_stake_by_id <a id="stakeable-token-check-liquidity-stake-by-id"></a>

  End a liquidity stake for `self.get_caller()` having.

  Following is the table of parameters.

  | Parameter Name     | Type      |
  | ------------------ | --------- |
  | staker             | Key       |
  | liquidity_stake_id | Vec\<u32> |

  This method **returns** `Vec<String>`.

- #### get_pair_address <a id="stakeable-token-get-pair-address"></a>

  Returns the address of Uniswap V2 Pair contract on the CasperLabs Blockchain.
  Parameter Name | Type
  |---|--- |

  This method **returns** a Key type.

- #### get_total_staked <a id="stakeable-token-get-total-staked"></a>

  Returns the total amount of tokens staked for stakes in Stakeable Token.
  Parameter Name | Type
  |---|--- |

  This method **returns** a U256 type.

- #### get_liquidity_transformer <a id="stakeable-token-get-liquidity-transformer"></a>

  Returns the `liquidity_transformer` address.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** a Key type.

- #### get_synthetic_token_address <a id="stakeable-token-get-synthetic-token-address"></a>

  Returns the `synthetic_token` address.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** a Key type.

- #### transfer <a id="erc20-transfer"></a>

  Returns Result<(), u32> if amount transfered successfully return ok().

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | recipient      | Key  |
  | amount         | U256 |

  This method **returns** `Result<(), u32>`.

- #### transfer_from <a id="erc20-transfer-from"></a>

  Returns Result<(), u32> if amount transfered successfully return ok().

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |
  | recipient      | Key  |
  | amount         | U256 |

  This method **returns** `Result<(), u32>`.

  - #### approve <a id="erc20-approve"></a>
    Lets `self.get_caller()` set their allowance for a spender.
    <br>user needs to call this `approve` method before calling the `transfer_from` method.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | spender        | Key  |
  | amount         | U256 |

  This method **returns** nothing.

- #### balance_of <a id="erc20-balance-of"></a>

  This method will return the balance of owner in `ERC20 Contract`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** U256.

- #### nonce <a id="erc20-nonce"></a>

  Returns the current `nonce` for an address for use in `permit`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** U256.

- #### allowance <a id="erc20-allowance"></a>

  Returns the amount of liquidity tokens owned by an hash that a spender is allowed to transfer via `transfer_from`.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |
  | spender        | Key  |

  This method **returns** U256.

- #### total_supply <a id="erc20-total-supply"></a>

  Returns the total amount of pool tokens for a pair.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** U256.

- #### mint <a id="erc20-mint"></a>

  This method mints the number of tokens provided by user against the hash provided by user.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | to             | Key  |
  | amount         | U256 |

  This method **returns** nothing.

- #### burn <a id="erc20-burn"></a>

  This method burns the number of tokens provided by user against the hash provided by user.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | from           | Key  |
  | amount         | U256 |

  This method **returns** nothing.

- #### name <a id="erc20-name"></a>

  Returns the `name` of tokens for a pair.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** String.

- #### symbol <a id="erc20-symbol"></a>

  Returns the `symbol` of tokens for a pair.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** String.

- #### increase_allowance <a id="erc20-increase-allowance"></a>

  Use to increase the allowance of the user.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | spender        | Key  |
  | amount         | U256 |

  This method **returns** `Result<(), u32>`.

- #### decrease_allowance <a id="erc20-decrease-allowance"></a>

  Use to decrease the allowance of the user.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | spender        | Key  |
  | amount         | U256 |

  This method **returns** `Result<(), u32>`.

- #### package_hash <a id="stakeable-token-package-hash"></a>

  Return the package hash of the contract.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `ContractPackageHash`.
