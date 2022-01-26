use crate::constants::*;
use crate::transfer_helper_instance::TransferHelperInstance;
use casper_engine_test_support::AccountHash;
use casper_types::{runtime_args, ContractPackageHash, Key, RuntimeArgs, U256, U512};
use test_env::{Sender, TestContract, TestEnv};

fn deploy_erc20(env: &TestEnv, owner: AccountHash, name: &str, symbol: &str) -> TestContract {
    let decimals: u8 = 18;
    let init_total_supply: U256 = 1000.into();

    TestContract::new(
        &env,
        "erc20-token.wasm",
        "erc20",
        Sender(owner),
        runtime_args! {
            "initial_supply" => init_total_supply,
            "name" => name.to_string(),
            "symbol" => symbol.to_string(),
            "decimals" => decimals
        },
    )
}

fn deploy() -> (
    TestEnv,
    TransferHelperInstance,
    TransferHelperInstance,
    AccountHash,
    Key,
    TestContract,
) {
    let env = TestEnv::new();
    let owner = env.next_user();
    // let transfer_invoker = env.next_user();
    // Key::Hash(transfer_helper.contract_hash()),

    // deploy proxy
    let proxy: TestContract = TransferHelperInstance::proxy(&env, Sender(owner));

    // deploy transfer helper with transfer_invoker being proxy
    let proxy_as_transfer_helper: TransferHelperInstance = TransferHelperInstance::instance(proxy);
    let proxy_package_hash_as_key: Key = Key::from(proxy_as_transfer_helper.package_hash_result());
    let transfer_helper: TestContract = TransferHelperInstance::new(
        &env,
        CONTRACT_NAME,
        Sender(owner),
        proxy_package_hash_as_key,
    );
    // set transfer_helper in proxy
    let transfer_helper_instance = TransferHelperInstance::instance(transfer_helper);
    let transfer_helper_contract_hash_as_key =
        Key::from(transfer_helper_instance.self_hash_result());
    proxy_as_transfer_helper.set_key_by_name(
        Sender(owner),
        TRANSFER_HELPER_HASH_KEY_NAME.to_string(),
        transfer_helper_contract_hash_as_key,
    );

    let erc20: TestContract = deploy_erc20(&env, owner, "erc20", "erc");
    let erc20_hash: Key = Key::Hash(erc20.contract_hash());

    proxy_as_transfer_helper.set_key_by_name(Sender(owner), "erc20".to_string(), erc20_hash);

    (
        env,
        transfer_helper_instance,
        proxy_as_transfer_helper,
        owner,
        proxy_package_hash_as_key,
        erc20,
    )
}

#[test]
fn test_deploy() {
    let (env, transfer_helper, proxy, owner, transfer_invoker, _) = deploy();
}

#[test]
fn get_transfer_invoker_address() {
    let (env, transfer_helper, proxy, owner, transfer_invoker, _) = deploy();

    proxy.get_transfer_invoker_address(Sender(owner));
    let result: Key = proxy.get_transfer_invoker_address_result();
    assert_eq!(result, transfer_invoker);
}

#[test]
fn test_forward_amount_with_transfer_invoker() {
    let (env, transfer_helper, proxy, owner, transfer_invoker, erc20) = deploy();
    let proxy_package_hash_as_key: Key = Key::from(proxy.package_hash_result());
    let transfer_helper_package_hash_as_key: Key = Key::from(transfer_helper.package_hash_result());
    let werc20 = TransferHelperInstance::instance(erc20);
    // mint to proxy contract
    werc20.mint(
        Sender(owner),
        transfer_helper_package_hash_as_key,
        U256::from(100),
    );
    // transfer helper now has 100 balance and proxy has 0
    assert_eq!(werc20.balance_of(proxy_package_hash_as_key), U256::from(0));
    assert_eq!(
        werc20.balance_of(transfer_helper_package_hash_as_key),
        U256::from(100)
    );
    // proxy, the transfer invoker,  now calls forward_funds, and gains balance 50
    proxy.forward_funds(Sender(owner), werc20.self_hash_result(), U256::from(50));
    // check that proxy has it's balance baxk
    assert_eq!(werc20.balance_of(proxy_package_hash_as_key), U256::from(50));
}

#[test]
#[should_panic]
fn test_forward_amount_without_transfer_invoker() {
    let (env, transfer_helper, proxy, owner, transfer_invoker, erc20) = deploy();
    let transfer_helper_package_hash_as_key: Key = Key::from(transfer_helper.package_hash_result());
    let werc20 = TransferHelperInstance::instance(erc20);
    let transfer_helper_contract_hash_as_key = Key::from(transfer_helper.self_hash_result());

    // deploy a new proxy
    let proxy2 = TransferHelperInstance::proxy(&env, Sender(owner));
    let proxy2 = TransferHelperInstance::instance(proxy2);
    proxy2.set_key_by_name(
        Sender(owner),
        TRANSFER_HELPER_HASH_KEY_NAME.to_string(),
        transfer_helper_contract_hash_as_key,
    );
    let proxy2_package_hash_as_key: Key = Key::from(proxy2.package_hash_result());

    // mint to transfer helper
    werc20.mint(
        Sender(owner),
        transfer_helper_package_hash_as_key,
        U256::from(100),
    );
    // transfer helper now has 100 balance and proxy2 has 0
    assert_eq!(werc20.balance_of(proxy2_package_hash_as_key), U256::from(0));
    assert_eq!(
        werc20.balance_of(transfer_helper_package_hash_as_key),
        U256::from(100)
    );
    // proxy2, not the transfer invoker,  now calls forward_funds. contract will revert
    proxy.forward_funds(
        Sender(owner),
        werc20.self_contract_hash_result(),
        U256::from(50),
    );
}

// #[test]
// #[should_panic]
// fn test_calling_construction() {
//     let (_, helper, _,owner, invoker) = deploy();
//     helper.constructor(Sender(owner), NAME, SYMBOL);
// }
