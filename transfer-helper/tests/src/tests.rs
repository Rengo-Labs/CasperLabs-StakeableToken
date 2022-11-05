use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U256};
use casperlabs_test_env::{TestContract, TestEnv};
use tests_common::{deploys::*, helpers::*, keys::SESSION_WASM_TRANSFER_HELPER};

fn deploy() -> (TestEnv, AccountHash, TestContract, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let transfer_helper = deploy_transfer_helper(&env, owner, Key::Account(owner), now());
    let erc20: TestContract = deploy_erc20(
        &env,
        owner,
        "erc20-token".into(),
        "ERC20".into(),
        9,
        0.into(),
        now(),
    );
    (env, owner, erc20, transfer_helper)
}

#[test]
fn get_transfer_invoker_address() {
    let (env, owner, _, transfer_helper) = deploy();
    call(
        &env,
        owner,
        SESSION_WASM_TRANSFER_HELPER,
        runtime_args! {
            "entrypoint" => "get_transfer_invoker_address",
            "package_hash" => Key::Hash(transfer_helper.package_hash()),
        },
        now(),
    );
    let ret: Key = result_key(&env, owner, "get_transfer_invoker_address");
    assert_eq!(ret, Key::Account(owner), "Owner not set invoker at default");
}

#[test]
fn should_forward_amount_from_transfer_invoker() {
    let (env, owner, erc20, transfer_helper) = deploy();
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Hash(transfer_helper.package_hash()),
            "amount" => U256::from(100)
        },
        now(),
    );
    assert_eq!(
        U256::from(100),
        result_dict(
            &env,
            erc20.contract_hash(),
            "balances",
            key_to_str(&Key::Hash(transfer_helper.package_hash()))
        ),
        "Tokens not minted"
    );
    assert_eq!(
        U256::from(0),
        result_dict(
            &env,
            erc20.contract_hash(),
            "balances",
            key_to_str(&Key::Account(owner))
        ),
        "Tokens not default"
    );

    transfer_helper.call_contract(
        owner,
        "forward_funds",
        runtime_args! {
            "token_address" => Key::Hash(erc20.package_hash()),
            "forward_amount" => U256::from(100)
        },
        now(),
    );

    assert_eq!(
        U256::from(0),
        result_dict(
            &env,
            erc20.contract_hash(),
            "balances",
            key_to_str(&Key::Hash(transfer_helper.contract_hash()))
        ),
        "Tokens not transfered"
    );
    assert_eq!(
        U256::from(100),
        result_dict(
            &env,
            erc20.contract_hash(),
            "balances",
            key_to_str(&Key::Account(owner))
        ),
        "Tokens not transfered"
    );
}

#[test]
#[should_panic]
fn should_not_forward_amount_without_transfer_invoker() {
    let (env, owner, erc20, transfer_helper) = deploy();
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Hash(transfer_helper.package_hash()),
            "amount" => U256::from(100)
        },
        now(),
    );
    assert_eq!(
        U256::from(100),
        result_dict(
            &env,
            erc20.contract_hash(),
            "balances",
            key_to_str(&Key::Hash(transfer_helper.package_hash()))
        ),
        "Tokens not minted"
    );
    assert_eq!(
        U256::from(0),
        result_dict(
            &env,
            erc20.contract_hash(),
            "balances",
            key_to_str(&Key::Account(owner))
        ),
        "Tokens not default"
    );

    transfer_helper.call_contract(
        env.next_user(),
        "forward_funds",
        runtime_args! {
            "token_address" => Key::Hash(erc20.package_hash()),
            "forward_amount" => U256::from(100)
        },
        now(),
    );

    assert_eq!(
        U256::from(0),
        result_dict(
            &env,
            erc20.contract_hash(),
            "balances",
            key_to_str(&Key::Hash(transfer_helper.contract_hash()))
        ),
        "Tokens not transfered"
    );
    assert_eq!(
        U256::from(100),
        result_dict(
            &env,
            erc20.contract_hash(),
            "balances",
            key_to_str(&Key::Account(owner))
        ),
        "Tokens not transfered"
    );
}
