# CasperLabs-Stakeable-StakeableToken

Implementation of `Liquidity Guard` and `Stakeable Token` contracts for CasperLabs Blockchain.

## Table of contents

- [Interacting with the contract](#interacting-with-the-contract)
  - [Install the prerequisites](#install-the-prerequisites)
  - [Creating Keys](#creating-keys)
  - [Usage](#usage)
    - [Install](#install)
    - [Build Individual Smart Contract](#build-individual-smart-contract)
    - [Build All Smart Contracts](#build-all-smart-contracts)
    - [Individual Test Cases](#individual-test-cases)
    - [All Test Cases](#all-test-cases)
- [Liquidity Guard](#liquidity-guard)
  - [Deployment](#deploying-liquidity-guard-contract-manually)
  - [Entry Point methods](#liquidity-guard-entry-point-methods)
    - [`get_inflation`](#liquidity-guard-get-inflation)
    - [`assign_inflation`](#liquidity-guard-assign-inflation)
- [Stakeable Token](#stakeable-token)
  - [Deployment](#deploying-stakeable-token-contract-manually)
  - [Entry Point methods](#stakeable-token-entry-point-methods)
    - [`set_liquidity_transfomer`](#stakeable-token-set-liquidity-transfomer)
    - [`set_stable_usd`](#stakeable-token-set-stable_usd)
    - [`renounce_keeper`](#stakeable-token-renounce-keeper)
    - [`change_keeper`](#stakeable-token-change-keeper)
    - [`mint_supply`](#stakeable-token-mint-supply)
    - [`create_stake_with_cspr`](#stakeable-token-create-stake-with-cspr)
    - [`create_stake_with_token`](#stakeable-token-create-stake-with-token)
    - [`get_pair_address`](#stakeable-token-get-pair-address)
    - [`get_total_staked`](#stakeable-token-get-total-staked)
    - [`get_liquidity_transformer`](#stakeable-token-get-liquidity-transformer)
    - [`get_synthetic_token_address`](#stakeable-token-get-synthetic-token-address)
    - [`extend_lt_auction`](#stakeable-token-extend-lt-auction)
    - [`transfer`](#erc20-transfer)
    - [`transfer_from`](#erc20-transfer-from)
    - [`permit`](#erc20-permit)
    - [`approve`](#erc20-approve)
    - [`balance_of`](#erc20-balance_of)
    - [`nonce`](#erc20-nonce)
    - [`allowance`](#erc20-allowance)
    - [`total_supply`](#erc20-total-supply)
    - [`mint`](#erc20-mint)
    - [`burn`](#erc20-burn)
    - [`name`](#erc20-name)
    - [`symbol`](#erc20-symbol)
    - [`current_stakeable_day`](#stakeable-token-current-stakeable-day)
    - [`liquidity_guard_trigger`](#stakeable-token-liquidity-guard-trigger)
    - [`manual_daily_snapshot`](#stakeable-token-manual-daily-snapshot)
    - [`get_stable_usd_equivalent`](#stakeable-token-get-stable-usd)
    - [`referrer_interest`](#stakeable-token-referrer-interest)
    - [`referrer_interest_bulk`](#stakeable-token-referrer-interest-bulk)
    - [`check_referrals_by_id`](#stakeable-token-check-referrals-by-id)
    - [`create_stake_bulk`](#stakeable-token-create-stake-bulk)
    - [`create_stake`](#stakeable-token-create-stake)
    - [`end_stake`](#stakeable-token-end-stake)
    - [`scrape_interest`](#stakeable-token-scrape-interest)
    - [`check_mature_stake`](#stakeable-token-check-mature-stake)
    - [`check_stake_by_id`](#stakeable-token-check-stake-by-id)
    - [`create_liquidity_stake`](#stakeable-token-create-liquidity-stake)
    - [`end_liquidity_stake`](#stakeable-token-end-liquidity-stake)
    - [`check_liquidity_stake_by_id`](#stakeable-token-check-liquidity-stake-by-id)

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

### Usage

To run the Contracts make sure you are in the folder of your required contract.

##### Install

Make sure `wasm32-unknown-unknown` is installed.

```
make prepare
```

It's also recommended to have [wasm-strip](https://github.com/WebAssembly/wabt)
available in your PATH to reduce the size of compiled Wasm.

##### Build Dependencies of Contract
Run this command to build dependencies of contracts.

```
make build-dependencies
```

##### Build Individual Smart Contract

Run this command to build Smart Contracts individually.

```
make build-stakeable-token
make build-liquidity-guard
```

<br>**Note:** User needs to be in the desired project folder to build contracts and User needs to run `make build-contract` in every project to make wasms to avoid errors

##### Build All Smart Contracts

Run this command in main folder to build all Smart Contract.

```
make build-all
```

##### Individual Test Cases

Run this command to run all test Cases.

```
make test-stakeable-token
make test-liquidity-guard
```

<br>**Note:** User needs to be in the desired project folder to run test cases

##### All Test Cases

Run this command in main folder to run all contract's Test Cases.

```
make test-all
```

##### Liquidity Guard <a id="liquidity-guard"></a>

##### Deployment <a id="deploying-liquidity-guard-contract-manually"></a>

If you need to deploy the `Liquidity Guard` manually you need to pass the some parameters. Following is the command to deploy the `Liquidity Guard`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 100000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="contract_name:string='contract_name'"
```

#### Entry Point methods <a id="liquidity-guard-entry-point-methods"></a>

Following are the Liquidity Guard's entry point methods.

- ##### get_inflation <a id="liquidity-guard-get-inflation"></a>
  Returns the inflation calculated at a certain amount.

| Parameter Name | Type |
| -------------- | ---- |
| amount         | u32 |

This method **returns** U256.

- ##### assign_inflation <a id="liquidity-guard-assign-inflation"></a>
  Assigns inflation as set in contract to inflations dictionary.
  <br> Contract reverts if inflation is assigned already.

| Parameter Name | Type |
| -------------- | ---- |
| ---            | ---  |

This method **returns** nothing.

##### Stakable Token <a id="stakeable-token"></a>
##### Deployment <a id="deploying-stakeable-token-contract-manually"></a>

If you need to deploy the `Stakeable Token` manually you need to pass the some parameters. Following is the command to deploy the `Stakeable Token`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 200000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="contract_name:string='contract_name'"\
    --session-arg="synthetic_cspr_address:string='synthetic_cspr_address-contract-hash'"\
    --session-arg="router_address:string='router_contract_hash'"\
    --session-arg="launch_time:string='launch_time'"\
    --session-arg="factory_address:string='factory_contract_hash'" \
    --session-arg="pair_address:string='pair_contract_hash'"\
    --session-arg="liquidity_guard:string='liquidity_guard_contract_hash'"\
    --session-arg="wcspr:string='wcspr_contract_hash'"
```

#### Entry Point methods <a id="stakeable-token-entry-point-methods"></a>

Following are the Stakeable Token's entry point methods.

- ##### set_liquidity_transfomer
  <a id="stakeable-token-get-set-liquidity-transfomer"></a>
  Sets Liquidity Transformer's hash and it's purse's uref to stakeable token contract global state.

| Parameter Name        | Type |
| --------------------- | ---- |
| immutable_transformer | Key  |
| transformer_purse     | URef |

This method **returns** U256.

- ##### set_stable_usd <a id="stakeable-token-update-set-stable_usd"></a>
  Sets Stable USD's contract hash to Stakeable token contract's global state.
  Parameter Name | Type
  |---|--- |
  | equalizer_address | Key |

This method **returns** nothing.

- ##### renounce_keeper <a id="stakeable-token-update-renounce-keeper"></a>
  Sets Transformer Gatekeeper named key to a hash of zero address.
  <br>Contract reverts if `self.get_caller()` is not the `transformer_gate_keeper`.

| Parameter Name | Type |
| -------------- | ---- |
| ---            | ---  |

This method **returns** nothing.

- ##### change_keeper <a id="stakeable-token-update-change-keeper"></a>
  Sets Transformer Gatekeeper named key to a provided address.
  <br>Contract reverts if `self.get_caller()` is not the `transformer_gate_keeper`.

| Parameter Name | Type |
| -------------- | ---- |
| keeper         | Key  |

This method **returns** nothing.

- ##### mint_supply <a id="stakeable-token-mint-supply"></a>
  Mints tokens to an address.
  <br>Contract reverts if `self.get_caller()` is not the Liquidity Transformer contract.

| Parameter Name   | Type |
| ---------------- | ---- |
| investor_address | Key  |
| amount           | U256 |

This method **returns** nothing.

- ##### create_stake_with_cspr <a id="stakeable-token-create-stake-with-cspr"></a>
  Creates a stake by withdrawing a cspr amount from a provided purse.
  Parameter Name | Type
  |---|--- |
  | referrer | Key |
  | amount | U256 |
  | lock_days | u64 |
  | purse | URef |

This method **returns** a tuple of order 3, described below.
| Tuple Index | Item Name | Type |
| --- | --- | --- |
|0|stake_id | Vec\<u32>
|1|start_day | u64
|2| referrer_id | Vec\<u32>

- ##### create_stake_with_token <a id="stakeable-token-create-stake-with-token"></a>
  Creates a stake by withdrawing an amount of tokens from a provided token contract againts `self.get_caller()`.
  <br> `self.get_caller()` must have given Stakeable Token contract allowance of 'token_amount' atleast before calling this entry point.
  Parameter Name | Type
  |---|--- |
  | referrer | Key |
  | lock_days | u64 |
  | token_amount | U256 |
  | token_address | URef |

This method **returns** a tuple of order 3, described below.
| Tuple Index | Item Name | Type |
| --- | --- | --- |
|0|stake_id | Vec\<u32>
|1|start_day | u64
|2| referrer_id | Vec\<u32>

- ##### get_pair_address <a id="stakeable-token-get-pair-address"></a>
  Returns the address of Uniswap V2 Pair contract on the CasperLabs Blockchain.
  Parameter Name | Type
  |---|--- |
  | --- | --- |

This method **returns** a Key type.

- ##### get_total_staked <a id="stakeable-token-get-total-staked"></a>
  Returns the total amount of tokens staked for stakes in Stakeable Token.
  Parameter Name | Type
  |---|--- |
  | --- | --- |

This method **returns** a U256 type.

- ##### get_liquidity_transformer <a id="stakeable-token-get-liquidity-transformer"></a>
  Returns the `liquidity_transformer` address.

| Parameter Name | Type |
| -------------- | ---- |
| ---            | ---  |

This method **returns** a Key type.

- ##### get_synthetic_token_address <a id="stakeable-token-get-synthetic-token-address"></a>
  Returns the `synthetic_token` address.

| Parameter Name | Type |
| -------------- | ---- |
| ---            | ---  |

This method **returns** a Key type.

- ##### current_stakeable_day <a id="stakeable-token-current-stakeable-day"></a>
  Returns the day since launch of WISE.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** u64.

- ##### liquidity_guard_trigger <a id="stakeable-token-liquidity-guard-trigger"></a>
  Enables the liquidity guard if it is disabled.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** nothing.

- ##### manual_daily_snapshot <a id="stakeable-token-manual-daily-snapshot"></a>
  Creates a snapshot from `update_day` till the current stakeable day.

Following is the table of parameters.

Parameter Name | Type
update_day|u64

This method **returns** nothing.

- ##### get_stable_usd_equivalent <a id="stakeable-token-get-stable-usd"></a>
  Returns the value of stable usd.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** U256.

- ##### referrer_interest <a id="stakeable-token-referrer-interest"></a>
  Returns the calculated interest on a particular referral for `scrape_days` duration and mints equivalend WISE tokens to `self.get_caller()`

Following is the table of parameters.

| Parameter Name | Type      |
| -------------- | --------- |
| referreral_id  | Vec\<u32> |
| scrape_days    | U256      |

This method **returns** nothing.

- ##### referrer_interest_bulk <a id="stakeable-token-referrer-interest-bulk"></a>
  Returns the calculated interest on a several referrals, each for for several `scrape_days` duration and mints equivalend WISE tokens to `self.get_caller()` in each case.

Following is the table of parameters.

| Parameter Name | Type            |
| -------------- | --------------- |
| referreral_ids | Vec\<Vec\<u32>> |
| scrape_days    | Vec\<U256>      |

This method **returns** nothing.

- ##### check_referrals_by_id <a id="stakeable-token-check-referrals-by-id"></a>
  Calculates rewards and shares for a referrer on a partical referral, and returns all information as a serialized struct.

Following is the table of parameters.

| Parameter Name | Type      |
| -------------- | --------- |
| referreral_ids | Vec\<u32> |
| referrer       | Key       |

This method **returns** StakeInfo type serialized as Vec\<u8>.

- ##### create_stake_bulk <a id="stakeable-token-create_stake_bulk"></a>
  Creates several stakes for `self.get_caller()` each with a referrer.

Following is the table of parameters.

| Parameter Name | Type      |
| -------------- | --------- |
| staked_amount  | U256      |
| lock_days      | Vec\<u64> |
| referrer       | Vec\<Key> |

This method **returns** nothing.

- ##### create_stake <a id="stakeable-token-create_stake"></a>
  Creates a stake for `self.get_caller()` with a referrer.

Following is the table of parameters.

| Parameter Name | Type      |
| -------------- | --------- |
| stake_id       | Vec\<u32> |
| start_day      | u64       |
| referral_id    | Vec\<u32> |

This method **returns** a tupe of order 3 described below.

| Tuple Index | Item Name   | Type      |
| ----------- | ----------- | --------- |
| 0           | stake_id    | Vec\<u32> |
| 1           | start_day   | u64       |
| 2           | referrer_id | Vec\<u32> |

- ##### end_stake <a id="stakeable-token-end-stake"></a>
  Ends a stakes of given `stake_id` having been created by `self.get_caller()`.

Following is the table of parameters.

| Parameter Name | Type      |
| -------------- | --------- |
| stake_id       | Vec\<u32> |

This method **returns** nothing.

- ##### scrape_interest <a id="stakeable-token-scrape-interest"></a>
  Calculates interests, rewards and penalties for a stake created by ``self.get_caller()`.`

Following is the table of parameters.

| Parameter Name | Type      |
| -------------- | --------- |
| stake_id       | Vec\<u32> |
| scrape_days    | u64       |

This method **returns** a Vec<u32>, described below.

| Vector Index | Item Name        | Type |
| ------------ | ---------------- | ---- |
| 0            | scrape_day       | U256 |
| 1            | scrape_amount    | U256 |
| 2            | remaining_days   | U256 |
| 3            | stakers_penalty  | U256 |
| 4            | referrer_penalty | U256 |

- ##### check_mature_stake <a id="stakeable-token-check-mature-stake"></a>
  Retrns true if a stake of `stake_id` created by a `staker` has matured.

Following is the table of parameters.

| Parameter Name | Type      |
| -------------- | --------- |
| stake_id       | Vec\<u32> |
| staker         | Key       |

This method **returns** Bool

- ##### check_stake_by_id <a id="stakeable-token-check-stake-by-id"></a>
  Retrns true if a stake of `stake_id` created by a `staker` has matured.

Following is the table of parameters.

| Parameter Name | Type      |
| -------------- | --------- |
| stake_id       | Vec\<u32> |
| staker         | Key       |

This method **returns** Bool

- ##### create_liquidity_stake <a id="stakeable-token-create-liquidity-stake"></a>
  Creates a liquidity stake for `self.get_caller()` staking `liquidity_token` of token amount.

Following is the table of parameters.

| Parameter Name  | Type |
| --------------- | ---- |
| liquidity_token | U256 |

This method **returns** Vec<u32> type of stake id.

- ##### end_liquidity_stake <a id="stakeable-token-end-liquidity-stake"></a>
  End a liquidity stake for `self.get_caller()` having.

Following is the table of parameters.

| Parameter Name  | Type |
| --------------- | ---- |
| liquidity_token | U256 |

This method **returns** nothing.

- ##### check_liquidity_stake_by_id <a id="stakeable-token-check-liquidity-stake-by-id"></a>
  End a liquidity stake for `self.get_caller()` having.

Following is the table of parameters.

| Parameter Name     | Type      |
| ------------------ | --------- |
| liquidity_stake_id | Vec\<u32> |
| staker             | Key       |

This method **returns** Stake type serlialized as Vec<u8>.
