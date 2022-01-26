#![cfg_attr(
    not(target_arch = "wasm32"),
    crate_type = "target arch should be wasm32"
)]
#![no_main]
#![no_std]

extern crate alloc;
use alloc::{boxed::Box, collections::BTreeSet, format, string::String, vec, vec::Vec};

use casper_types::ApiError;
use casper_contract::{
    contract_api::{runtime, storage, system, account},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},
    runtime_args, CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType,
    EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256, U128, U512
};

pub mod mappings;

#[no_mangle]
fn constructor() {
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    let wise_address: Key = runtime::get_named_arg("wise_address");
    let bep20_address: Key = runtime::get_named_arg("bep20_address");


    mappings::set_key(&mappings::self_hash_key(), contract_hash);
    mappings::set_key(&mappings::self_package_key(), package_hash);
    mappings::set_key(
        &mappings::wise_key(),
        ContractHash::from(wise_address.into_hash().unwrap_or_default()),
    );
    mappings::set_key(
        &mappings::bep20(),
        ContractHash::from(bep20_address.into_hash().unwrap_or_default()),
    );
}


#[no_mangle]
fn set_liquidity_transfomer() {

    let wise_contract: ContractHash = mappings::get_key(&mappings::wise_key());

    let immutable_transformer: Key = runtime::get_named_arg("immutable_transformer");
    let transformer_purse: URef = system::create_purse();

    let _ : () = runtime::call_contract(
        wise_contract, 
        "set_liquidity_transfomer", 
        runtime_args!{
            "immutable_transformer" => immutable_transformer,
            "transformer_purse" => transformer_purse
        }
    );
}

#[no_mangle]
fn set_stable_usd()
{
    let wise_contract: ContractHash = mappings::get_key(&mappings::wise_key());
    let equalizer_address: Key = runtime::get_named_arg("equalizer_address");

    let _ : () = runtime::call_contract(
        wise_contract, 
        "set_stable_usd", 
        runtime_args!{
            "equalizer_address" => equalizer_address
        }
    );
}

#[no_mangle]
fn renounce_keeper() {

    let wise_contract: ContractHash = mappings::get_key(&mappings::wise_key());
    let _ : () = runtime::call_contract(
        wise_contract, 
        "renounce_keeper", 
        runtime_args!{}
    );
}

#[no_mangle]
fn mint_supply() {

    let wise_contract: ContractHash = mappings::get_key(&mappings::wise_key());
    let investor_address: Key = runtime::get_named_arg("investor_address");
    let amount: U256 = runtime::get_named_arg("amount");

    let _: () = runtime::call_contract(
        wise_contract, 
        "mint_supply", 
        runtime_args!{
            "investor_address" => investor_address,
            "amount" => amount
        }
    );
}


#[no_mangle]
fn create_stake_with_cspr() {

    let self_hash: Key = runtime::get_named_arg("test_contract_hash");
    let self_hash: ContractHash = ContractHash::from(self_hash.into_hash().unwrap_or_revert());

    let lock_days: u64 = runtime::get_named_arg("lock_days");
    let referrer: Key = runtime::get_named_arg("referrer");
    let amount: U256 = runtime::get_named_arg("amount");
    let caller_purse: URef = account::get_main_purse();

    let constructor_args = runtime_args! {
        "lock_days" => lock_days,
        "referrer" => referrer,
        "amount" => amount,
        "purse" => caller_purse
    };

    let _: () = runtime::call_contract(self_hash, "create_stake_with_cspr_execute", constructor_args);
}


/*
    This is the actual function that calls the entrypoint of the wise.
*/
#[no_mangle]
fn create_stake_with_cspr_execute() {

    let wise_contract: ContractHash = mappings::get_key(&mappings::wise_key());

    let lock_days: u64 = runtime::get_named_arg("lock_days");
    let referrer: Key = runtime::get_named_arg("referrer");
    let amount: U256 = runtime::get_named_arg("amount");
    let purse: URef = runtime::get_named_arg("purse");

    let (stake_id, start_day, referrer_id):(Vec<u32>, U256 ,Vec<u32>) = runtime::call_contract(
        wise_contract, 
        "create_stake_with_cspr", 
        runtime_args!{
            "lock_days" => lock_days,
            "referrer" => referrer,
            "amount" => amount,
            "purse" => purse
        }
    );
}

#[no_mangle]
fn add_liquidity_cspr_to_router() {

    let router_address: Key = runtime::get_named_arg("router_address");
    let self_hash: Key = runtime::get_named_arg("self_hash");
    let token: Key = runtime::get_named_arg("token");

    let amount_token_desired: U256 = runtime::get_named_arg("amount_token_desired");
    let amount_cspr_desired: U256 = runtime::get_named_arg("amount_cspr_desired");
    let amount_token_min: U256 = runtime::get_named_arg("amount_token_min");
    let amount_cspr_min: U256 = runtime::get_named_arg("amount_cspr_min");
    let to: Key = runtime::get_named_arg("to");
    let deadline: U256 = runtime::get_named_arg("deadline");
    let pair: Option<Key> = runtime::get_named_arg("pair");

    // create dummy contract purse and send some cspr to it
    let self_purse: URef = system::create_purse();                        // create contract's purse 
    let caller_purse: URef = account::get_main_purse();

    // transfer dummy cspr to self_purse. 
    // Since system::transfer_from_purse_to_purse works with contracts having type 'Contract' and since this method has type 'Session' thus creating a sperate entry point for transfering funds between purses
    let _: () = runtime::call_contract(ContractHash::from(self_hash.into_hash().unwrap_or_default()), "transfer_cspr", runtime_args!{
        "src_purse" => caller_purse,
        "dest_purse" => self_purse,
        "amount" => U512::from(amount_cspr_desired.as_u32())
    });


    let args: RuntimeArgs = runtime_args! {
        "router_address" => Key::from(router_address),
        "token" => token,
        "amount_token_desired" => amount_token_desired,
        "amount_cspr_desired" => amount_cspr_desired,
        "amount_cspr_min" => amount_cspr_min,
        "amount_token_min" => amount_token_min,
        "to" => to,
        "deadline" => deadline,
        "pair" => pair,
        "purse" => self_purse
    };
    let _: () = runtime::call_contract(ContractHash::from(self_hash.into_hash().unwrap_or_default()), "router_add_liquidity_cspr", args);

    // this entry points context is session therefore it can't access contract keys. Therefore to set the keys, it calls new entrypoint method.
    // let _: () = runtime::call_contract(self_hash, "set_liquidity_cspr_keys", runtime_args! { "amount_token" => amount_token, "amount_cspr" => amount_cspr, "liquidity" => liquidity});
}

#[no_mangle]
fn router_add_liquidity_cspr() {

    let router_address: Key = runtime::get_named_arg("router_address");
    let router_address: ContractHash = ContractHash::from(router_address.into_hash().unwrap_or_revert());

    let token: Key = runtime::get_named_arg("token");
    let token_contract: ContractHash = ContractHash::from(token.into_hash().unwrap_or_default());

    let amount_token_desired: U256 = runtime::get_named_arg("amount_token_desired");
    let amount_cspr_desired: U256 = runtime::get_named_arg("amount_cspr_desired");
    let amount_token_min: U256 = runtime::get_named_arg("amount_token_min");
    let amount_cspr_min: U256 = runtime::get_named_arg("amount_cspr_min");
    let to: Key = runtime::get_named_arg("to");
    let deadline: U256 = runtime::get_named_arg("deadline");
    let purse: URef = runtime::get_named_arg("purse");
    let pair: Option<Key> = runtime::get_named_arg("pair");
    
    let router_package_hash: ContractPackageHash = runtime::call_contract(router_address, "package_hash", runtime_args!{});
    
    // Approve contract
    let _ : bool = runtime::call_contract(
        token_contract, 
        "approve",  
        runtime_args!{
        "spender" => Key::from(router_package_hash),
        "amount" => amount_token_desired
    });

    let args: RuntimeArgs = runtime_args! {
        "token" => token,
        "amount_token_desired" => amount_token_desired,
        "amount_cspr_desired" => amount_cspr_desired,
        "amount_token_min" => amount_token_min,
        "amount_cspr_min" => amount_cspr_min,
        "to" => to,
        "deadline" => deadline,
        "pair" => pair,
        "purse" => purse
    };

    let (amount_token, amount_cspr, liquidity): (U256, U256, U256) =
        runtime::call_contract(router_address, "add_liquidity_cspr", args);

    // this entry points context is session therefore it can't access contract keys. Therefore to set the keys, it calls new entrypoint method.
    // let _: () = runtime::call_contract(self_hash, "set_liquidity_cspr_keys", runtime_args! { "amount_token" => amount_token, "amount_cspr" => amount_cspr, "liquidity" => liquidity});
}

// need a seperate entry point methods to transfer cspr
#[no_mangle]
fn transfer_cspr() {

    let src_purse: URef = runtime::get_named_arg("src_purse");
    let dest_purse: URef = runtime::get_named_arg("dest_purse");
    let amount: U512 = runtime::get_named_arg("amount");

    let _:() = system::transfer_from_purse_to_purse(src_purse, dest_purse,  amount, None).unwrap_or_revert();
}

#[no_mangle]
fn extend_lt_auction()
{
    let wise_contract: ContractHash = mappings::get_key(&mappings::wise_key());

    let _ : () = runtime::call_contract(
        wise_contract,
        "extend_lt_auction",
        runtime_args!{}
    );
}

#[no_mangle]
fn router_add_liquidity() {

    let router_address: Key = runtime::get_named_arg("router_address");
    let router_address: ContractHash = ContractHash::from(router_address.into_hash().unwrap_or_revert());

    let token_a: Key = runtime::get_named_arg("token_a");
    let token_b: Key = runtime::get_named_arg("token_b");
    let amount_a_desired: U256 = runtime::get_named_arg("amount_a_desired");
    let amount_b_desired: U256 = runtime::get_named_arg("amount_b_desired");
    let amount_a_min: U256 = runtime::get_named_arg("amount_a_min");
    let amount_b_min: U256 = runtime::get_named_arg("amount_b_min");
    let to: Key = runtime::get_named_arg("to");
    let deadline: U256 = runtime::get_named_arg("deadline");
    let pair: Option<Key> = runtime::get_named_arg("pair");


    let router_package_hash: ContractPackageHash = runtime::call_contract(router_address, "package_hash", runtime_args!{});
    
    // Approve contract
    let _ : bool = runtime::call_contract(
        ContractHash::from(token_a.into_hash().unwrap()), 
        "approve",  
        runtime_args!{
        "spender" => Key::from(router_package_hash),
        "amount" => amount_a_desired
    });

    let _ : bool = runtime::call_contract(
        ContractHash::from(token_b.into_hash().unwrap()), 
        "approve",  
        runtime_args!{
        "spender" => Key::from(router_package_hash),
        "amount" => amount_b_desired
    });

    let args: RuntimeArgs = runtime_args! {
        "token_a" => token_a,
        "token_b" => token_b,
        "amount_a_desired" => amount_a_desired,
        "amount_b_desired" => amount_b_desired,
        "amount_a_min" => amount_a_min,
        "amount_b_min" => amount_b_min,
        "to" => to,
        "deadline" => deadline,
        "pair" => pair
    };

    let (amount_token, amount_cspr, liquidity): (U256, U256, U256) =
        runtime::call_contract(router_address, "add_liquidity", args);
}



fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();

    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
            Parameter::new("wise_address", Key::cl_type()),
            Parameter::new("bep20_address", Key::cl_type())
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "set_liquidity_transfomer",
        vec![         
            Parameter::new("immutable_transformer", CLType::Key)
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "set_stable_usd",
        vec![
            Parameter::new("equalizer_address", CLType::Key)
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "renounce_keeper",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "mint_supply",
        vec![
            Parameter::new("investor_address", CLType::Key),
            Parameter::new("amount", CLType::U256)
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "create_stake_with_cspr",
        vec![
            Parameter::new("test_contract_hash", CLType::Key),
            Parameter::new("lock_days", CLType::U64),
            Parameter::new("referrer", CLType::Key),
            Parameter::new("amount", CLType::U256)
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Session,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "create_stake_with_cspr_execute",
        vec![
            Parameter::new("lock_days", CLType::U64),
            Parameter::new("referrer", CLType::Key),
            Parameter::new("amount", CLType::U256),
            Parameter::new("purse", CLType::URef)
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("add_liquidity_cspr_to_router"),
        vec![
            Parameter::new("router_address", Key::cl_type()),
            Parameter::new("token", Key::cl_type()),
            Parameter::new("amount_token_desired", CLType::U256),
            Parameter::new("amount_cspr_desired", CLType::U256),
            Parameter::new("amount_token_min", CLType::U256),
            Parameter::new("amount_cspr_min", CLType::U256),
            Parameter::new("to", Key::cl_type()),
            Parameter::new("deadline", CLType::U256),
            Parameter::new("pair", CLType::Option(Box::new(CLType::Key))),
            Parameter::new("self_hash", CLType::Key)
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Session,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("router_add_liquidity_cspr"),
        vec![
            Parameter::new("router_address", Key::cl_type()),
            Parameter::new("token", Key::cl_type()),
            Parameter::new("amount_token_desired", CLType::U256),
            Parameter::new("amount_cspr_desired", CLType::U256),
            Parameter::new("amount_token_min", CLType::U256),
            Parameter::new("amount_cspr_min", CLType::U256),
            Parameter::new("to", Key::cl_type()),
            Parameter::new("deadline", CLType::U256),
            Parameter::new("pair", CLType::Option(Box::new(CLType::Key))),
            Parameter::new("purse", CLType::URef)
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("transfer_cspr"),
        vec![
            Parameter::new("src_purse", CLType::URef),
            Parameter::new("dest_purse", CLType::URef),
            Parameter::new("amount", CLType::U512),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("router_add_liquidity"),
        vec![
            Parameter::new("router_address", Key::cl_type()),
            Parameter::new("token_a", CLType::Key),
            Parameter::new("token_b", CLType::Key),
            Parameter::new("amount_a_desired", CLType::U256),
            Parameter::new("amount_b_desired", CLType::U256),
            Parameter::new("amount_a_min", CLType::U256),
            Parameter::new("amount_b_min", CLType::U256),
            Parameter::new("to", CLType::Key),
            Parameter::new("deadline", CLType::U256),
            Parameter::new("pair", CLType::Option(Box::new(CLType::Key)))
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "extend_lt_auction",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points
}

// All session code must have a `call` entrypoint.
#[no_mangle]
pub extern "C" fn call() {
    // Build new package with initial a first version of the contract.
    let (package_hash, access_token) = storage::create_contract_package_at_hash();
    let (contract_hash, _): (ContractHash, _) =
        storage::add_contract_version(package_hash, get_entry_points(), Default::default());

    let wise_address: Key = runtime::get_named_arg("wise_address");
    let bep20_address: Key = runtime::get_named_arg("bep20_address");

    // Get parameters and pass it to the constructors
    // Prepare constructor args
    let constructor_args = runtime_args! {
        "contract_hash" => contract_hash,
        "package_hash" => package_hash,
        "wise_address" => wise_address,
        "bep20_address" => bep20_address
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

    // Store contract in the account's named keys.
    let contract_name: alloc::string::String = runtime::get_named_arg("contract_name");

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
