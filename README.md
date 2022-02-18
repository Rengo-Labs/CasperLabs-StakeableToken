# CasperLabs-Wise-WiseToken
Implementation of `Transfer Helper`, `Stable USD`, `Liquidity Guard` and `Wise Token` contracts for CasperLabs Blockchain.

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
- [Transfer Helper](#transfer-helper)
  - [Deployment](#deploying-transfer-helper-contract-manually)
  - [Entry Point methods](#transfer-helper-entry-point-methods)
    - [```get_transfer_invoker_address```](#transfer-helper-get-transfer-invoker-address)
    - [```forward_funds```](#transfer-helper-forward-funds)
- [Liquidity Guard](#liquidity-guard)
  - [Deployment](#deploying-liquidity-guard-contract-manually)
  - [Entry Point methods](#liquidity-guard-entry-point-methods)
    - [```get_inflation```](#liquidity-guard-get-inflation)
    - [```assign_inflation```](#liquidity-guard-assign-inflation)
- [Stable USD](#stable-usd)
  - [Deployment](#deploying-stable-usd-contract-manually)
  - [Entry Point methods](#stable-usd-entry-point-methods)
    - [```get_stable_usd_equivalent```](#stable-usd-get-stable-usd)
    - [```update_stable_usd_equivalent```](#stable-usd-update-stable-usd)
- [Wise Token](#wise-token)
  - [Deployment](#deploying-wise-token-contract-manually)
  - [Entry Point methods](#wise-token-entry-point-methods)
    - [```set_liquidity_transfomer```](#wise-token-set-liquidity-transfomer)
    - [```set_stable_usd```](#wise-token-set-stable_usd)
    - [```renounce_keeper```](#wise-token-renounce-keeper)
    - [```change_keeper```](#wise-token-change-keeper)
    - [```mint_supply```](#wise-token-mint-supply)
    - [```create_stake_with_cspr```](#wise-token-create-stake-with-cspr)
    - [```create_stake_with_token```](#wise-token-create-stake-with-token)
    - [```get_pair_address```](#wise-token-get-pair-address)
    - [```get_total_staked```](#wise-token-get-total-staked)
    - [```get_liquidity_transformer```](#wise-token-get-liquidity-transformer)
    - [```get_synthetic_token_address```](#wise-token-get-synthetic-token-address)
    - [```extend_lt_auction```](#wise-token-extend-lt-auction)
    - [```transfer```](#erc20-transfer)
    - [```transfer_from```](#erc20-transfer-from)
    - [```permit```](#erc20-permit)
    - [```approve```](#erc20-approve)
    - [```balance_of```](#erc20-balance_of)
    - [```nonce```](#erc20-nonce)
    - [```allowance```](#erc20-allowance)
    - [```total_supply```](#erc20-total-supply)
    - [```mint```](#erc20-mint)
    - [```burn```](#erc20-burn)
    - [```name```](#erc20-name)
    - [```symbol```](#erc20-symbol)
    - [```current_wise_day```](#wise-token-current-wise-day)
    - [```liquidity_guard_trigger```](#wise-token-liquidity-guard-trigger)
    - [```manual_daily_snapshot```](#wise-token-manual-daily-snapshot)
    - [```get_stable_usd_equivalent```](#wise-token-get-stable-usd)
    - [```referrer_interest```](#wise-token-referrer-interest)
    - [```referrer_interest_bulk```](#wise-token-referrer-interest-bulk)
    - [```check_referrals_by_id```](#wise-token-check-referrals-by-id)
    - [```create_stake_bulk```](#wise-token-create-stake-bulk)
    - [```create_stake```](#wise-token-create-stake)
    - [```end_stake```](#wise-token-end-stake)
    - [```scrape_interest```](#wise-token-scrape-interest)
    - [```check_mature_stake```](#wise-token-check-mature-stake)
    - [```check_stake_by_id```](#wise-token-check-stake-by-id)
    - [```create_liquidity_stake```](#wise-token-create-liquidity-stake)
    - [```end_liquidity_stake```](#wise-token-end-liquidity-stake)
    - [```check_liquidity_stake_by_id```](#wise-token-check-liquidity-stake-by-id)

## Interacting with the contract
You need to have `casper-client` and `jq` installed on your system to run the examples. The instructions have been tested on Ubuntu 20.04.0 LTS.

### Install the prerequisites

You can install the required software by issuing the following commands. If you are on an up-to-date Casper node, you probably already have all of the prerequisites installed so you can skip this step.

```bash
# Update package repositories
sudo apt update

# Install the command-line JSON processor
sudo apt install jq -y

# Install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

#Install the nightly version (by default stable toolchain is installed)
rustup install nightly

#Check that nightly toolchain version is installed(this will list stable and nightly versions)
rustup toolchain list

#Set rust nightly as default
rustup default nightly

# Install wasm32-unknown-unknown
rustup target add wasm32-unknown-unknown

#rust Version
rustup --version

#Install Cmake
 sudo apt-get -y install cmake

Note:https://cgold.readthedocs.io/en/latest/first-step/installation.html

#cmake Version
cmake --version

#Installing the Casper Crates
cargo install cargo-casper

# Add Casper repository
echo "deb https://repo.casperlabs.io/releases" bionic main | sudo tee -a /etc/apt/sources.list.d/casper.list
curl -O https://repo.casperlabs.io/casper-repo-pubkey.asc
sudo apt-key add casper-repo-pubkey.ascr
sudo apt update

# Install the Casper client software
Install Casper-client

cargo +nightly install casper-client

# To check Casper Client Version
Casper-client --version

# Commands for help
casper-client --help

casper-client <command> --help

```
### Creating Keys

```bash
# Create keys
casper-client keygen <TARGET DIRECTORY>
```

### Usage
To run the Contracts make sure you are in the folder of your required contract.
#### Install
Make sure `wasm32-unknown-unknown` is installed.
```
make prepare
```

It's also recommended to have [wasm-strip](https://github.com/WebAssembly/wabt)
available in your PATH to reduce the size of compiled Wasm.

#### Build Individual Smart Contract
Run this command to build Smart Contract.
```
make build-contract
```
<br>**Note:** User needs to be in the desired project folder to build contracts and User needs to run `make build-contract` in every project to make wasms to avoid errors

#### Build All Smart Contracts
Run this command in main folder to build all Smart Contract.
```
make all
```

#### Individual Test Cases
Run this command to run Test Cases.
```
make test
```
<br>**Note:** User needs to be in the desired project folder to run test cases

#### All Test Cases
Run this command in main folder to run all contract's Test Cases.
```
make test
```

### Transfer Helper <a id="transfer-helper"></a>

#### Deployment <a id="deploying-transfer-helper-contract-manually"></a>
If you need to deploy the `Transfer Helper` manually you need to pass the some parameters. Following is the command to deploy the `Transfer Helper`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="transfer_invoker:string='transfer_invoker_address'" \
    --session-arg="contract_name:string='contract_name'"
```

#### Entry Point methods <a id="transfer-helper-entry-point-methods"></a>

Following are the Transfer Helper's entry point methods.

- ##### get_transfer_invoker_address <a id="transfer-helper-get-transfer-invoker-address"></a>
Returns the current transfer invoker address.

Parameter Name | Type
|---|--- |
| --- | --- |

This method **returns** an address hash.

- ##### forward_funds <a id="transfer-helper-forward-funds"></a>
Forwards tokens owned by transfer helper contract in a given token contract to a recipient.
<br>Funds intended for forwarding must be owned by transfer helper.
<br>Only `transfer_invoker` can call this entrypoint.

Parameter Name | Type
---|---
token_address | Key
forward_amount | U256

This method **returns** a `Boolean` value indicating success/failure of funds transfer.

### Liquidity Guard <a id="liquidity-guard"></a>

#### Deployment <a id="deploying-liquidity-guard-contract-manually"></a>
If you need to deploy the `Liquidity Guard` manually you need to pass the some parameters. Following is the command to deploy the `Liquidity Guard`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="contract_name:string='contract_name'"
```

#### Entry Point methods <a id="liquidity-guard-entry-point-methods"></a>

Following are the Liquidity Guard's entry point methods.

- ##### get_inflation <a id="liquidity-guard-forward-funds"></a>
Returns the inflation calculated at a certain amount.

Parameter Name | Type
---|---
amount | U256

This method **returns** U256.

- ##### assign_inflation <a id="liquidity-guard-assign-inflation"></a>
Assigns inflation as set in contract to inflations dictionary.
<br> Contract reverts if inflation is assigned already.

Parameter Name | Type
|---|--- |
| --- | --- |

This method **returns** nothing.

### Stable USD <a id="stable-usd"></a>

#### Deployment <a id="deploying-stable-usd-contract-manually"></a>
If you need to deploy the `Stable USD` manually you need to pass the some parameters. Following is the command to deploy the `Stable USD`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="contract_name:string='contract_name'"\
    --session-arg="wise:string='wise-token-contract-hash'"\
    --session-arg="scspr:string='scspr_contract_hash'"\
    --session-arg="wcspr:string='wcspr_contract_hash'"\
    --session-arg="stable_usd:string='stable_usd_contract_hash'" \
    --session-arg="router:string='router_contract_hash'"
```

#### Entry Point methods <a id="stable-usd-entry-point-methods"></a>

Following are the Stable USD's entry point methods.

- ##### get_stable_usd_equivalent <a id="stable-usd-get-stable-usd"></a>
Based on the `path`, a vector of contract hashes of length 4 that is *[wise_contract_hash, scspr_contract_hash, wcspr_contract_hash, stable_usd_contract_hash]* set at deployment,calculates the latest Stable USD value.
Parameter Name | Type
|---|--- |
| --- | --- |

This method **returns** U256.

- ##### update_stable_usd_equivalent <a id="stable-usd-update-stable-usd"></a>
Based on the `path`, a vector of contract hashes of length 4 that is *[wise_contract_hash, scspr_contract_hash, wcspr_contract_hash, stable_usd_contract_hash]* set at deployment,calculates the latest Stable USD value and sets in to contract global state as named key.
<br> Contract reverts if inflation is assigned already.
Parameter Name | Type
|---|--- |
| --- | --- |

This method **returns** nothing.

### Wise Token <a id="wise-token"></a>

#### Deployment <a id="deploying-wise-token-contract-manually"></a>
If you need to deploy the `Wise Token` manually you need to pass the some parameters. Following is the command to deploy the `Wise Token`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
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

#### Entry Point methods <a id="wise-token-entry-point-methods"></a>

Following are the Wise Token's entry point methods.

- ##### set_liquidity_transfomer
 <a id="wise-token-get-set-liquidity-transfomer"></a>
Sets Liquidity Transformer's hash and it's purse's uref to wise token contract global state.

Parameter Name | Type
|---|--- |
| immutable_transformer | Key |
| transformer_purse | URef |

This method **returns** U256.

- ##### set_stable_usd <a id="wise-token-update-set-stable_usd"></a>
Sets Stable USD's contract hash to Wise token contract's global state.
Parameter Name | Type
|---|--- |
| equalizer_address | Key |

This method **returns** nothing.

- ##### renounce_keeper <a id="wise-token-update-renounce-keeper"></a>
Sets Transformer Gatekeeper named key to a hash of zero address.
<br>Contract reverts if ```self.get_caller()``` is not the `transformer_gate_keeper`.

Parameter Name | Type
|---|--- |
| --- | --- |

This method **returns** nothing.

- ##### change_keeper <a id="wise-token-update-change-keeper"></a>
Sets Transformer Gatekeeper named key to a provided address.
<br>Contract reverts if ```self.get_caller()``` is not the `transformer_gate_keeper`.

Parameter Name | Type
|---|--- |
| keeper | Key |

This method **returns** nothing.

- ##### mint_supply <a id="wise-token-mint-supply"></a>
Mints tokens to an address.
<br>Contract reverts if ```self.get_caller()``` is not the Liquidity Transformer contract.

Parameter Name | Type
|---|--- |
| investor_address | Key |
| amount | U256 |


This method **returns** nothing.

- ##### create_stake_with_cspr <a id="wise-token-create-stake-with-cspr"></a>
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

- ##### create_stake_with_token <a id="wise-token-create-stake-with-token"></a>
Creates a stake by withdrawing an amount of tokens from a provided token contract againts `self.get_caller()`.
<br> `self.get_caller()` must have given Wise Token contract allowance of 'token_amount' atleast before calling this entry point.
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

- ##### get_pair_address <a id="wise-token-get-pair-address"></a>
Returns the address of Uniswap V2 Pair contract on the CasperLabs Blockchain.
Parameter Name | Type
|---|--- |
| --- | --- |

This method **returns** a Key type.

- ##### get_total_staked <a id="wise-token-get-total-staked"></a>
Returns the total amount of tokens staked for stakes in Wise Token.
Parameter Name | Type
|---|--- |
| --- | --- |

This method **returns** a U256 type.

- ##### get_liquidity_transformer <a id="wise-token-get-liquidity-transformer"></a>
Returns the `liquidity_transformer` address.

Parameter Name | Type
|---|--- |
| --- | --- |

This method **returns** a Key type.

- ##### get_synthetic_token_address <a id="wise-token-get-synthetic-token-address"></a>
Returns the `synthetic_token` address.

Parameter Name | Type
|---|--- |
| --- | --- |

This method **returns** a Key type.

- ##### extend_lt_auction <a id="wise-token-extend-lt-auction"></a>
Updates the current launch time for Wise Token.

Parameter Name | Type
|---|--- |
| --- | --- |

This method **returns** nothing.

- ##### transfer <a id="erc20-transfer"></a>
Lets ` self.get_caller() ` send pool tokens to a recipient hash.

Following is the table of parameters.

Parameter Name | Type
---|---
recipient | Key
amount | U256


This method **returns** nothing.

- ##### transfer_from <a id="erc20-transfer-from"></a>
Sends pool tokens from one hash to another.
<br>User needs to call approve method before calling the ` tranfer_from `.

Following is the table of parameters.

Parameter Name | Type
---|---
owner | Key
recipient | Key
amount | U256


This method **returns** nothing.
<br>**Recommendation:** 
The exploit is mitigated through use of functions that increase/decrease the allowance relative to its current value, such as `increaseAllowance()` and `decreaseAllowance()`,
Pending community agreement on an ERC standard that would protect against this exploit, we recommend that developers of applications dependent on approve() / transferFrom()
should keep in mind that they have to set allowance to 0 first and verify if it was used before setting the new value.
<br>**Note:**  Teams who decide to wait for such a standard should make these
recommendations to app developers who work with their token contract.

- ##### permit <a id="erc20-permit"></a>
Sets the allowance for a spender where approval is granted via a signature.

Following is the table of parameters.

Parameter Name | Type
---|---
public | String
signature | String
owner | Key
spender | Key
value | U256
deadline | u64


This method **returns** nothing.


- ##### approve <a id="erc20-approve"></a>
Lets ` self.get_caller() ` set their allowance for a spender.
<br>user needs to call this `approve` method before calling the `transfer_from` method.

Following is the table of parameters.

Parameter Name | Type
---|---
spender | Key
amount | U256

This method **returns** nothing.
<br>**Recommendation:** 
The exploit is mitigated through use of functions that increase/decrease the allowance relative to its current value, such as `increaseAllowance()` and `decreaseAllowance()`,
Pending community agreement on an ERC standard that would protect against this exploit, we recommend that developers of applications dependent on approve() / transferFrom()
should keep in mind that they have to set allowance to 0 first and verify if it was used before setting the new value.
<br>**Note:**  Teams who decide to wait for such a standard should make these
recommendations to app developers who work with their token contract.

- ##### balance_of <a id="erc20-balance-of"></a>
This method will return the balance of owner in `ERC20 Contract`.

Following is the table of parameters.

Parameter Name | Type
---|---
owner | Key


This method **returns** U256.


- ##### nonce <a id="erc20-nonce"></a>
Returns the current `nonce` for an address for use in ` permit `.

Following is the table of parameters.

Parameter Name | Type
---|---
owner | Key


This method **returns** U256.


- ##### allowance <a id="erc20-allowance"></a>
Returns the amount of liquidity tokens owned by an hash that a spender is allowed to transfer via ` transfer_from `.

Following is the table of parameters.

Parameter Name | Type
---|---
owner | Key
spender | Key


This method **returns** U256.


- ##### total_supply <a id="erc20-total-supply"></a>
Returns the total amount of pool tokens for a pair.

Following is the table of parameters.

Parameter Name | Type
---|---


This method **returns** U256.


- ##### mint <a id="erc20-mint"></a>
This method mints the number of tokens provided by user against the hash provided by user.

Following is the table of parameters.

Parameter Name | Type
---|---
to | Key
amount | U256

This method **returns** nothing.


- ##### burn <a id="erc20-burn"></a>
This method burns the number of tokens provided by user against the hash provided by user.

Following is the table of parameters.

Parameter Name | Type
---|---
from | Key
amount | U256

This method **returns** nothing.
<br>**Note:** To `burn` the tokens against the hash provided by user, User needs to `mint` tokens first in `ERC20`.

- ##### name <a id="erc20-name"></a>
Returns the `name` of tokens for a pair.

Following is the table of parameters.

Parameter Name | Type
---|---

This method **returns** String.

- ##### symbol <a id="erc20-symbol"></a>
Returns the `symbol` of tokens for a pair.

Following is the table of parameters.

Parameter Name | Type
---|---

This method **returns** String.

- ##### current_wise_day <a id="wise-token-current-wise-day"></a>
Returns the day since launch of WISE.

Following is the table of parameters.

Parameter Name | Type
---|---

This method **returns** u64.

- ##### liquidity_guard_trigger <a id="wise-token-liquidity-guard-trigger"></a>
Enables the liquidity guard if it is disabled.

Following is the table of parameters.

Parameter Name | Type
---|---

This method **returns** nothing.

- ##### manual_daily_snapshot <a id="wise-token-manual-daily-snapshot"></a>
Creates a snapshot from ```update_day``` till the current wise day.

Following is the table of parameters.

Parameter Name | Type
update_day|u64

This method **returns** nothing.

- ##### get_stable_usd_equivalent <a id="wise-token-get-stable-usd"></a>
Returns the value of stable usd.

Following is the table of parameters.

Parameter Name | Type
---|---
This method **returns** U256.

- ##### referrer_interest <a id="wise-token-referrer-interest"></a>
Returns the calculated interest on a particular referral for ```scrape_days``` duration and mints equivalend WISE tokens to ```self.get_caller()```

Following is the table of parameters.

Parameter Name | Type
 --- | ---
referreral_id|Vec\<u32>
scrape_days|U256

This method **returns** nothing.

- ##### referrer_interest_bulk <a id="wise-token-referrer-interest-bulk"></a>
Returns the calculated interest on a several referrals, each for for several ```scrape_days``` duration and mints equivalend WISE tokens to ```self.get_caller()``` in each case.

Following is the table of parameters.

Parameter Name | Type
 --- | ---
referreral_ids | Vec\<Vec\<u32>>
scrape_days | Vec\<U256>

This method **returns** nothing.

- ##### check_referrals_by_id <a id="wise-token-check-referrals-by-id"></a>
Calculates rewards and shares for a referrer on a partical referral, and returns all information as a serialized struct.

Following is the table of parameters.

Parameter Name | Type
 --- | ---
referreral_ids | Vec\<u32>
referrer | Key

This method **returns** StakeInfo type serialized as Vec\<u8>.

- ##### create_stake_bulk <a id="wise-token-create_stake_bulk"></a>
Creates several stakes for ```self.get_caller()``` each with a referrer.

Following is the table of parameters.

Parameter Name | Type
 --- | ---
staked_amount | U256
lock_days | Vec\<u64>
referrer | Vec\<Key>

This method **returns** nothing.

- ##### create_stake <a id="wise-token-create_stake"></a>
Creates a stake for ```self.get_caller()``` with a referrer.

Following is the table of parameters.

Parameter Name | Type
 --- | ---
stake_id | Vec\<u32>
start_day | u64
referral_id | Vec\<u32>

This method **returns** a tupe of order 3 described below.

| Tuple Index | Item Name | Type |
| --- | --- | --- |
|0|stake_id | Vec\<u32>
|1|start_day | u64
|2| referrer_id | Vec\<u32>

- ##### end_stake <a id="wise-token-end-stake"></a>
Ends a stakes of given ```stake_id``` having been created by ```self.get_caller()```.

Following is the table of parameters.

Parameter Name | Type
 --- | ---
stake_id | Vec\<u32>

This method **returns** nothing.

- ##### scrape_interest <a id="wise-token-scrape-interest"></a>
Calculates interests, rewards and penalties for a stake created by ```self.get_caller()``.`

Following is the table of parameters.

Parameter Name | Type
 --- | ---
stake_id | Vec\<u32>
scrape_days | u64

This method **returns** a Vec<u32>, described below.

| Vector Index | Item Name | Type |
| --- | --- | --- |
|0|scrape_day | U256
|1|scrape_amount | U256
|2|remaining_days | U256
|3|stakers_penalty |U256
|4|referrer_penalty |U256

- ##### check_mature_stake <a id="wise-token-check-mature-stake"></a>
Retrns true if a stake of ```stake_id``` created by a ```staker``` has matured.

Following is the table of parameters.

Parameter Name | Type
 --- | ---
stake_id | Vec\<u32>
staker | Key

This method **returns** Bool

- ##### check_stake_by_id <a id="wise-token-check-stake-by-id"></a>
Retrns true if a stake of ```stake_id``` created by a ```staker``` has matured.

Following is the table of parameters.

Parameter Name | Type
 --- | ---
stake_id | Vec\<u32>
staker | Key

This method **returns** Bool

- ##### create_liquidity_stake <a id="wise-token-create-liquidity-stake"></a>
Creates a liquidity stake for ```self.get_caller()``` staking ```liquidity_token``` of token amount.

Following is the table of parameters.

Parameter Name | Type
 --- | ---
liquidity_token | U256

This method **returns** Vec<u32> type of stake id.

- ##### end_liquidity_stake <a id="wise-token-end-liquidity-stake"></a>
End a liquidity stake for ```self.get_caller()``` having.

Following is the table of parameters.

Parameter Name | Type
 --- | ---
liquidity_token | U256

This method **returns** nothing.

- ##### check_liquidity_stake_by_id <a id="wise-token-check-liquidity-stake-by-id"></a>
End a liquidity stake for ```self.get_caller()``` having.

Following is the table of parameters.

Parameter Name | Type
 --- | ---
liquidity_stake_id | Vec\<u32>
staker | Key

This method **returns** Stake type serlialized as Vec<u8>.
