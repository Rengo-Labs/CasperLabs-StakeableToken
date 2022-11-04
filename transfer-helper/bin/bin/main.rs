#![no_main]
#![no_std]

extern crate alloc;
use alloc::{collections::BTreeSet, format, vec};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},
    runtime_args, CLType, CLTyped, CLValue, EntryPoint, EntryPointAccess, EntryPointType,
    EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256,
};
use casperlabs_contract_utils::{ContractContext, OnChainContractStorage};
use transfer_helper_crate::{self, src::TRANSFERHELPER};

#[derive(Default)]
struct TransferHelperStruct(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for TransferHelperStruct {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}
impl TRANSFERHELPER<OnChainContractStorage> for TransferHelperStruct {}

impl TransferHelperStruct {
    fn constructor(&self, contract_hash: Key, package_hash: Key, transfer_invoker: Key) {
        TRANSFERHELPER::init(self, contract_hash, package_hash, transfer_invoker);
    }
}

#[no_mangle]
fn constructor() {
    let transfer_invoker: Key = runtime::get_named_arg("transfer_invoker");
    let contract_hash: Key = runtime::get_named_arg("contract_hash");
    let package_hash: Key = runtime::get_named_arg("package_hash");
    TransferHelperStruct::default().constructor(contract_hash, package_hash, transfer_invoker);
}

#[no_mangle]
fn forward_funds() {
    let token_address: Key = runtime::get_named_arg("token_address");
    let forward_amount: U256 = runtime::get_named_arg("forward_amount");

    let ret: bool = TransferHelperStruct::default().forward_funds(token_address, forward_amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_transfer_invoker_address() {
    let address: Key = TransferHelperStruct::default().get_transfer_invoker_address();
    runtime::ret(CLValue::from_t(address).unwrap_or_revert());
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("transfer_invoker", Key::cl_type()),
            Parameter::new("contract_hash", Key::cl_type()),
            Parameter::new("package_hash", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "forward_funds",
        vec![
            Parameter::new("token_address", Key::cl_type()),
            Parameter::new("forward_amount", U256::cl_type()),
        ],
        CLType::Bool,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_transfer_invoker_address",
        vec![],
        CLType::Key,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}

// All session code must have a `call` entrypoint.
#[no_mangle]
pub extern "C" fn call() {
    // Store contract in the account's named keys. Contract name must be same for all new versions of the contracts
    let contract_name: alloc::string::String = runtime::get_named_arg("contract_name");

    // If this is the first deployment
    if !runtime::has_key(&format!("{}_package_hash", contract_name)) {
        // Build new package.
        let (package_hash, access_token) = storage::create_contract_package_at_hash();
        // add a first version to this package
        let (contract_hash, _): (ContractHash, _) =
            storage::add_contract_version(package_hash, get_entry_points(), Default::default());

        let transfer_invoker: Key = runtime::get_named_arg("transfer_invoker");

        // Prepare constructor args
        let constructor_args = runtime_args! {
            "transfer_invoker" => transfer_invoker,
            "contract_hash" => Key::from(contract_hash),
            "package_hash" => Key::from(package_hash)
        };

        // Add the constructor group to the package hash with a single URef.
        let constructor_access: URef =
            storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
                .unwrap_or_revert()
                .pop()
                .unwrap_or_revert();

        // Call the constructor entry point
        let _: () =
            runtime::call_versioned_contract(package_hash, None, "constructor", constructor_args);

        // Remove all URefs from the constructor group, so no one can call it for the second time.
        let mut urefs = BTreeSet::new();
        urefs.insert(constructor_access);
        storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
            .unwrap_or_revert();

        runtime::put_key(
            &format!("{}_package_hash", contract_name),
            package_hash.into(),
        );
        runtime::put_key(
            &format!("{}_package_hash_wrapped", contract_name),
            storage::new_uref(package_hash).into(),
        );
        runtime::put_key(
            &format!("{}_contract_hash", contract_name),
            contract_hash.into(),
        );
        runtime::put_key(
            &format!("{}_contract_hash_wrapped", contract_name),
            storage::new_uref(contract_hash).into(),
        );
        runtime::put_key(
            &format!("{}_package_access_token", contract_name),
            access_token.into(),
        );
    }
    // If contract package did already exist
    else {
        // get the package
        let package_hash: ContractPackageHash =
            runtime::get_key(&format!("{}_package_hash", contract_name))
                .unwrap_or_revert()
                .into_hash()
                .unwrap()
                .into();
        // create new version and install it
        let (contract_hash, _): (ContractHash, _) =
            storage::add_contract_version(package_hash, get_entry_points(), Default::default());

        // update contract hash
        runtime::put_key(
            &format!("{}_contract_hash", contract_name),
            contract_hash.into(),
        );
        runtime::put_key(
            &format!("{}_contract_hash_wrapped", contract_name),
            storage::new_uref(contract_hash).into(),
        );
    }
}
