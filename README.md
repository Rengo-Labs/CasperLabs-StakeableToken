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
    - [`get_transformer_gate_keeper`](#stakeable-token-get-transformer-gate-keeper)
    - [`set_stable_usd`](#stakeable-token-set-stable_usd)
    - [`renounce_keeper`](#stakeable-token-renounce-keeper)
    - [`change_keeper`](#stakeable-token-change-keeper)
    - [`mint_supply`](#stakeable-token-mint-supply)
    - [`create_stake_with_cspr`](#stakeable-token-create-stake-with-cspr)
    - [`get_liquidity_rate`](#stakeable-token-get-liquidity-rate)
    - [`get_scspr`](#stakeable-token-get-scspr)
    - [`get_uniswap_pair`](#stakeable-token-get-uniswap-pair)
    - [`get_inflation_rate`](#stakeable-token-get-inflation-rate)
    - [`create_pair`](#stakeable-token-create-pair)
    - [`get_scheduled_to_end`](#stakeable-token-get-scheduled-to-end)
    - [`get_total_penalties`](#stakeable-token-get-total-penalties)
    - [`get_scrapes`](#stakeable-token-get-scrapes)
    - [`get_stake_count`](#stakeable-token-get-stake-count)
    - [`get_referral_count`](#stakeable-token-get-referral-count)
    - [`get_liquidity_stake_count`](#stakeable-token-get-liquidity-stake-count)
    - [`get_referral_shares_to_end`](#stakeable-token-get-referral-shares-to-end)
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

### Install

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

### Build Individual Smart Contract

Run this command to build Smart Contracts individually.

```
make build-stakeable-token
make build-liquidity-guard
```

<br>**Note:** User needs to be in the desired project folder to build contracts and User needs to run `make build-contract` in every project to make wasms to avoid errors

### Build All Smart Contracts

Run this command in main folder to build all Smart Contract.

```
make build-all
```

### Individual Test Cases

Run this command to run all test Cases.

```
make test-stakeable-token
make test-liquidity-guard
```

<br>**Note:** User needs to be in the desired project folder to run test cases

### All Test Cases

Run this command in main folder to run all contract's Test Cases.

```
make test-all
```

### Liquidity Guard <a id="liquidity-guard"></a>

### Deployment <a id="deploying-liquidity-guard-contract-manually"></a>

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
| ---            | ---  |

This method **returns** nothing.

### Stakable Token <a id="stakeable-token"></a>

### Deployment <a id="deploying-stakeable-token-contract-manually"></a>

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
    --session-arg="stable_usd:Key='stable usd address'"\
    --session-arg="scspr:Key='synthetic_cspr_address-contract-hash'"\
    --session-arg="wcspr:Key='wcspr_contract_hash'"
    --session-arg="router_address:Key='router_contract_hash'"\
    --session-arg="factory_address:Key='factory_contract_hash'" \
    --session-arg="pair_address:Key='pair_contract_hash'"\
    --session-arg="liquidity_guard:Key='liquidity_guard_contract_hash'"\
```

### Entry Point methods <a id="stakeable-token-entry-point-methods"></a>

Following are the Stakeable Token's entry point methods.

- #### set_liquidity_transfomer

  <a id="stakeable-token-set-liquidity-transfomer"></a>
  Set Liquidity Transformer's hash and it's purse's uref to stakeable token contract global state.

  | Parameter Name        | Type |
  | --------------------- | ---- |
  | immutable_transformer | Key  |
  | transformer_purse     | URef |

  This method **returns** U256.

- #### get_transformer_gate_keeper

  <a id="stakeable-token-get-transformer-gate-keeper"></a>
  Return the transformer gate keeper.

  | Parameter Name | Type |
  | -------------- | ---- |

  This method **returns** `Key`.

- #### mint_supply <a id="stakeable-token-mint-supply"></a>

  Mints tokens to an address.
  <br>Contract reverts if `self.get_caller()` is not the Liquidity Transformer contract.

  | Parameter Name   | Type |
  | ---------------- | ---- |
  | investor_address | Key  |
  | amount           | U256 |

  This method **returns** nothing.

- #### create_stake_with_cspr <a id="stakeable-token-create-stake-with-cspr"></a>

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
  |staker|U256 |

  This method **returns** `U256`

- #### create_stake_with_token <a id="stakeable-token-create-stake-with-token"></a>

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
  | ---            | ---  |

  This method **returns** a Key type.

- #### current_stakeable_day <a id="stakeable-token-current-stakeable-day"></a>

  Returns the day since launch of WISE.

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

  Creates a snapshot from `update_day` till the current stakeable day.

  Following is the table of parameters.

  Parameter Name | Type
  update_day|u64

  This method **returns** nothing.

- #### get_stable_usd_equivalent <a id="stakeable-token-get-stable-usd"></a>

  Returns the value of stable usd.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |

This method **returns** U256.

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

- #### check_referrals_by_id <a id="stakeable-token-check-referrals-by-id"></a>
  Calculates rewards and shares for a referrer on a partical referral, and returns all information as a serialized struct.

  Following is the table of parameters.

  | Parameter Name | Type      |
  | -------------- | --------- |
  | referreral_ids | Vec\<u32> |
  | referrer       | Key       |

  This method **returns** StakeInfo type serialized as Vec\<u8>.

- #### create_stake_bulk <a id="stakeable-token-create_stake_bulk"></a>
  Creates several stakes for `self.get_caller()` each with a referrer.

  Following is the table of parameters.

  | Parameter Name | Type      |
  | -------------- | --------- |
  | staked_amount  | U256      |
  | lock_days      | Vec\<u64> |
  | referrer       | Vec\<String> |

  This method **returns** nothing.

- #### create_stake <a id="stakeable-token-create_stake"></a>
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

- #### end_stake <a id="stakeable-token-end-stake"></a>
  Ends a stakes of given `stake_id` having been created by `self.get_caller()`.

  Following is the table of parameters.

  | Parameter Name | Type      |
  | -------------- | --------- |
  | stake_id       | Vec\<u32> |

  This method **returns** nothing.

- #### scrape_interest <a id="stakeable-token-scrape-interest"></a>
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

- #### check_mature_stake <a id="stakeable-token-check-mature-stake"></a>
  Retrns true if a stake of `stake_id` created by a `staker` has matured.

  Following is the table of parameters.

  | Parameter Name | Type      |
  | -------------- | --------- |
  | stake_id       | Vec\<u32> |
  | staker         | Key       |

  This method **returns** Bool

- #### check_stake_by_id <a id="stakeable-token-check-stake-by-id"></a>
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

  | Parameter Name  | Type |
  | --------------- | ---- |
  | liquidity_token | U256 |

  This method **returns** Vec<u32> type of stake id.

- #### end_liquidity_stake <a id="stakeable-token-end-liquidity-stake"></a>
  End a liquidity stake for `self.get_caller()` having.

  Following is the table of parameters.

  | Parameter Name  | Type |
  | --------------- | ---- |
  | liquidity_token | U256 |

  This method **returns** nothing.

- #### check_liquidity_stake_by_id <a id="stakeable-token-check-liquidity-stake-by-id"></a>
  End a liquidity stake for `self.get_caller()` having.

  Following is the table of parameters.

  | Parameter Name     | Type      |
  | ------------------ | --------- |
  | liquidity_stake_id | Vec\<u32> |
  | staker             | Key       |

  This method **returns** Stake type serlialized as Vec<u8>.
