# CasperLabs-Wise-WiseToken
Implementation of `Transfer Helper`, `BUSD Equivalent`, `Liquidity Guard` and `Wise Token` contracts for CasperLabs Blockchain.

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
- [BUSD Equivalent](#busd-equivalent)
  - [Deployment](#deploying-busd-equivalent-contract-manually)
  - [Entry Point methods](#busd-equivalent-entry-point-methods)
    - [```get_busd_equivalent```](#busd-equivalent-get-busd-equivalent)
    - [```update_busd_equivalent```](#busd-equivalent-update-busd-equivalent)
- [Wise Token](#wise-token)
  - [Deployment](#deploying-wise-token-contract-manually)
  - [Entry Point methods](#wise-token-entry-point-methods)
    - [```set_liquidity_transfomer```](#wise-token-set-liquidity-transfomer)
    - [```set_busd```](#wise-token-set-busd)
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

### BUSD Eqiavalent <a id="busd-equivalent"></a>

#### Deployment <a id="deploying-busd-equivalent-contract-manually"></a>
If you need to deploy the `BUSD Equivalent` manually you need to pass the some parameters. Following is the command to deploy the `BUSD Equivalent`.

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
    --session-arg="busd:string='busd_contract_hash'" \
    --session-arg="router:string='router_contract_hash'"
```

#### Entry Point methods <a id="busd-equivalent-entry-point-methods"></a>

Following are the BUSD Equivalent's entry point methods.

- ##### get_busd_equivalent <a id="busd-equivalent-get-busd-equivalent"></a>
Based on the `path`, a vector of contract hashes of length 4 that is *[wise_contract_hash, scspr_contract_hash, wcspr_contract_hash, busd_contract_hash]* set at deployment,calculates the latest busd equivalent value.
Parameter Name | Type
|---|--- |
| --- | --- |

This method **returns** U256.

- ##### update_busd_equivalent <a id="busd-equivalent-update-busd-equivalent"></a>
Based on the `path`, a vector of contract hashes of length 4 that is *[wise_contract_hash, scspr_contract_hash, wcspr_contract_hash, busd_contract_hash]* set at deployment,calculates the latest busd equivalent value and sets in to contract global state as named key.
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

- ##### set_busd <a id="wise-token-update-set-busd"></a>
Sets BUSD Equivalent's contract hash to Wise token contract's global state.
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