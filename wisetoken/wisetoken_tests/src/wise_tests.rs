use once_cell::sync::Lazy;

use casper_engine_test_support::{
    internal::{ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_RUN_GENESIS_REQUEST},
    DEFAULT_ACCOUNT_ADDR, MINIMUM_ACCOUNT_CREATION_BALANCE,
};
use casper_execution_engine::core::{
    engine_state::{Error as CoreError, ExecuteRequest},
    execution::Error as ExecError,
};
use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, system::mint, ApiError, CLTyped,
    ContractHash, ContractPackageHash, Key, PublicKey, RuntimeArgs, SecretKey, U256,
};

use crate::wise_instance::WiseTestInstance;
use std::time::{SystemTime, UNIX_EPOCH};
use test_env::{Sender, TestContract, TestEnv};

fn deploy_erc20(env: &TestEnv, owner: AccountHash, name: &str, symbol: &str) -> TestContract {
    let decimals: u8 = 18;
    let init_total_supply: U256 = 1000.into();

    TestContract::new(
        &env,
        "erc20-token.wasm",
        // "erc20",
        symbol,
        Sender(owner),
        runtime_args! {
            "initial_supply" => init_total_supply,
            "name" => name.to_string(),
            "symbol" => symbol.to_string(),
            "decimals" => decimals
        },
    )
}

fn deploy_stable_usd_equivalent(
    env: TestEnv,
    owner: AccountHash,
    wise: TestContract,
) -> TestContract {
    let router: Key = wise.query_named_key("router_contract_hash".to_string());
    let scspr: Key = wise.query_named_key("scspr_contract_hash".to_string());
    let wcspr: Key = wise.query_named_key("wcspr_contract_hash".to_string());
    // since stable_usd is an ERC20 token, using casper's erc20 as stable_usd
    let stable_usd: TestContract =
        deploy_erc20(&env, owner, "stable_usd stable coin", "stable_usd");
    // deploy stable_usd_equivalent eq. contract
    let stable_usd_equivalent = TestContract::new(
        &env,
        "stable_usd_equivalent.wasm",
        "stable_usd_equivalent",
        Sender(owner),
        runtime_args! {
            "wise" => Key::Hash(wise.contract_hash()),
            "scspr" => scspr,
            "wcspr" => wcspr,
            "stable_usd" => Key::Hash(stable_usd.contract_hash()),
            "router" => router
        },
    );

    stable_usd_equivalent
}

fn deploy_pair_contract(
    env: &TestEnv,
    owner: AccountHash,
    factory_contract: Key,
    flash_swapper: Key,
) -> TestContract {
    let decimals: u8 = 18;
    let init_total_supply: U256 = 0.into();

    let pair_contract = TestContract::new(
        &env,
        "pair-token.wasm",
        "pair",
        Sender(owner),
        runtime_args! {
            "name" => "erc20",
            "symbol" => "ERC",
            "decimals" => decimals,
            "initial_supply" => init_total_supply,
            "factory_hash" => factory_contract,
            "callee_contract_hash" => flash_swapper
        },
    );

    pair_contract
}

fn deploy_synthetic_helper(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        &env,
        "synthetic_helper.wasm",
        "synthetic_helper",
        Sender(owner),
        runtime_args! {},
    )
}

fn deploy_synthetic_token(
    env: &TestEnv,
    owner: AccountHash,
    wcspr: &TestContract,
    synthetic_helper: &TestContract,
    uniswap_pair: &TestContract,
    uniswap_router: &TestContract,
    erc20: &TestContract,
) -> TestContract {
    TestContract::new(
        &env,
        "synthetic_token.wasm",
        "synthetic_token",
        Sender(owner),
        runtime_args! {
            "wcspr" => Key::Hash(wcspr.contract_hash()),
            "synthetic_helper" => Key::Hash(synthetic_helper.contract_hash()),
            "uniswap_pair" => Key::Hash(uniswap_pair.contract_hash()),
            "uniswap_router" => Key::Hash(uniswap_router.contract_hash()),
            "erc20" => Key::Hash(erc20.contract_hash()),
        },
    )
}

fn deploy_scspr(
    env: &TestEnv,
    owner: AccountHash,
    erc20: &TestContract,
    uniswap_factory: &TestContract,
    synthetic_helper: &TestContract,
    synthetic_token: &TestContract,
) -> TestContract {
    TestContract::new(
        &env,
        "scspr.wasm",
        "scspr",
        Sender(owner),
        runtime_args! {
            "erc20" => Key::Hash(erc20.contract_hash()),
            "uniswap_factory" => Key::Hash(uniswap_factory.contract_hash()),
            "synthetic_helper" => Key::Hash(synthetic_helper.contract_hash()),
            "synthetic_token" => Key::Hash(synthetic_token.contract_hash())
        },
    )
}

fn deploy_wise() -> (
    TestEnv,          // env
    AccountHash,      // owner
    TestContract,     // wise contract
    WiseTestInstance, // WiseTestInstance
    TestContract,     // erc20
    TestContract,     // flash_swapper
    TestContract,     // factory
    TestContract,     // Router
    TestContract,     // WCSPR
    TestContract,     // SCSPR
) {
    let env = TestEnv::new();
    let owner = env.next_user();

    let erc20_token = deploy_erc20(&env, owner, "erc20 token", "erc20");

    // deploy factory contract
    let factory_contract = TestContract::new(
        &env,
        "factory.wasm",
        "factory",
        Sender(owner),
        runtime_args! {
            "fee_to_setter" => Key::Hash(erc20_token.contract_hash())
            // contract_name is passed seperately, so we don't need to pass it here.
        },
    );

    let decimals: u8 = 18;
    // deploy wcspr contract
    let wcspr = TestContract::new(
        &env,
        "wcspr-token.wasm",
        "wcspr",
        Sender(owner),
        runtime_args! {
            "name" => "wcspr",
            "symbol" => "ERC",
            "decimals" => decimals
        },
    );

    // deploy wcspr contract
    let dai = TestContract::new(
        &env,
        "wcspr-token.wasm",
        "dai",
        Sender(owner),
        runtime_args! {
            "name" => "dai",
            "symbol" => "dai",
            "decimals" => decimals
        },
    );

    // deploy flash swapper
    let flash_swapper = TestContract::new(
        &env,
        "flash-swapper.wasm",
        "flash_swapper",
        Sender(owner),
        runtime_args! {
            "uniswap_v2_factory" => Key::Hash(factory_contract.contract_hash()),
            "wcspr" => Key::Hash(wcspr.contract_hash()),
            "dai" => Key::Hash(dai.contract_hash())
        },
    );

    // deploy pair contract
    let pair_contract = TestContract::new(
        &env,
        "pair-token.wasm",
        "pair",
        Sender(owner),
        runtime_args! {
            "name" => "erc20",
            "symbol" => "ERC",
            "decimals" => decimals,
            "initial_supply" => U256::from(0),
            "factory_hash" => Key::Hash(factory_contract.contract_hash()),
            "callee_contract_hash" => Key::Hash(flash_swapper.contract_hash())
        },
    );

    // deploy library contract
    let library_contract = TestContract::new(
        &env,
        "uniswap-v2-library.wasm",
        "library",
        Sender(owner),
        runtime_args! {},
    );

    // Deploy Router Contract
    let router_contract = TestContract::new(
        &env,
        "uniswap-v2-router.wasm",
        "uniswap",
        Sender(owner),
        runtime_args! {
            "factory" => Key::Hash(factory_contract.contract_hash()),
            "wcspr" => Key::Hash(wcspr.contract_hash()),
            "library" => Key::Hash(library_contract.contract_hash())
        },
    );

    //Deploy Liquidity Guard
    let liquidity_guard_contract = TestContract::new(
        &env,
        "liquidity_guard.wasm",
        "Liquidity Guard",
        Sender(owner),
        runtime_args! {},
    );

    // Deploy Synthetic helper
    let synthetic_helper = deploy_synthetic_helper(&env, owner);
    // deploy erc20
    let erc20: TestContract = deploy_erc20(&env, owner, "erc token", "erc20");

    // Deploy synthetic token
    let synthetic_token = deploy_synthetic_token(
        &env,
        owner,
        &wcspr,
        &synthetic_helper,
        &pair_contract,
        &router_contract,
        &erc20,
    );

    //Deploy Scspr
    let scspr_contract = deploy_scspr(
        &env,
        owner,
        &erc20,
        &factory_contract,
        &synthetic_helper,
        &synthetic_token,
    );

    // Deploy Wise Contract
    let launch_time: U256 = 10.into();

    let wise_contract = TestContract::new(
        &env,
        "wisetoken.wasm",
        "Wisetoken",
        Sender(owner),
        runtime_args! {
            "scspr" => Key::Hash(scspr_contract.contract_hash()),
            "router" => Key::Hash(router_contract.contract_hash()),
            "factory" => Key::Hash(factory_contract.contract_hash()),
            "pair" => Key::Hash(pair_contract.contract_hash()),
            "liquidity_guard" => Key::Hash(liquidity_guard_contract.contract_hash()),
            "wcspr" => Key::Hash(wcspr.contract_hash()),
            "launch_time" => launch_time
        },
    );

    // deploy Test contract
    let test_contract = WiseTestInstance::new(
        &env,
        Key::Hash(wise_contract.contract_hash()),
        Key::Hash(erc20.contract_hash()),
        Sender(owner),
    );

    // change keeper to test contract
    wise_contract.call_contract(
        Sender(owner),
        "change_keeper",
        runtime_args! {"keeper" => test_contract.test_contract_package_hash()},
    );

    // insert router to the factory's white-list
    let router_package_hash: ContractPackageHash =
        router_contract.query_named_key("package_hash".to_string());
    factory_contract.call_contract(
        Sender(owner),
        "set_white_list",
        runtime_args! {"white_list" => Key::from(router_package_hash)},
    );

    (
        env,
        owner,
        wise_contract,
        test_contract,
        erc20,
        flash_swapper,
        factory_contract,
        router_contract,
        wcspr,
        scspr_contract,
    )
}

#[test]
fn test_erc20_deploy() {
    let env = TestEnv::new();
    let owner = env.next_user();
    let test_context = deploy_erc20(&env, owner, "erc20 token", "erc20");
    assert_ne!(
        Key::Hash(test_context.contract_hash()),
        Key::Hash([0u8; 32])
    );
}

#[test]
fn test_wise_deploy() {
    let (env, owner, wise_contract, test_contract, _, _, _, _, _, _) = deploy_wise();
    assert_ne!(
        Key::from(test_contract.test_contract_hash()),
        Key::Hash([0u8; 32])
    );
}

#[test]
fn test_stable_usd_equivalent_deploy() {
    let (env, owner, wise_contract, _, _, _, _, _, _, _) = deploy_wise();
    let stable_usd_equivalent_contract = deploy_stable_usd_equivalent(env, owner, wise_contract);

    assert_ne!(
        Key::Hash(stable_usd_equivalent_contract.contract_hash()),
        Key::Hash([0u8; 32])
    );
}

#[test]
fn set_liquidity_transfomer() {
    let (_, owner, _, test_contract, _, _, _, _, _, _) = deploy_wise();
    test_contract.set_liquidity_transfomer(Sender(owner), Key::from(owner));
}

#[test]
fn set_stable_usd_equivalent() {
    let (env, owner, wise_contract, test_contract, _, _, _, _, _, _) = deploy_wise();
    let stable_usd_equivalent_contract = deploy_stable_usd_equivalent(env, owner, wise_contract);

    test_contract.set_stable_usd_equivalent(
        Sender(owner),
        Key::Hash(stable_usd_equivalent_contract.contract_hash()),
    );
}

#[test]
fn renounce_keeper() {
    let (_, owner, _, test_contract, _, _, _, _, _, _) = deploy_wise();
    test_contract.renounce_keeper(Sender(owner));
}

#[test]
fn mint_supply() {
    let (env, owner, wise, test_contract, _, _, _, _, _, _) = deploy_wise();
    let user: AccountHash = env.next_user();

    test_contract
        .set_liquidity_transfomer(Sender(owner), test_contract.test_contract_package_hash()); // make test contract as liquidity_transformer

    test_contract.mint_supply(Sender(owner), Key::from(user), 50.into());
    let amount: U256 = test_contract.balance_of(&wise, Key::from(user));

    assert_eq!(amount, 50.into()) // 1000 + 50 = 1050
}

//#[test]
fn create_stake_with_cspr() {
    let (
        env,
        owner,
        wise_contract,
        test_contract,
        _,
        flash_swapper,
        factory_contract,
        router_contract,
        wcspr,
        scspr,
    ) = deploy_wise();
    let user = env.next_user();

    // let router_library: ContractHash = router_contract.query_named_key("library_hash".to_string());
    // println!("LIbrary: {}", router_library);

    // let router_scspr: ContractHash = router_contract.query_named_key("wcspr".to_string());
    // let wise_scspr: Key = wise_contract.query_named_key("wcspr_contract_hash".to_string());
    // assert_eq!(Key::from(router_scspr), wise_scspr);

    // let router_hash: Key = Key::Hash(wcspr.contract_hash());
    // let router_hash: ContractHash = ContractHash::from(router_hash.into_hash().unwrap_or_default());

    // let wise_hash: Key = Key::Hash(wcspr.contract_hash());

    // let zero: Key = Key::Hash([0u8;32]);
    // assert_eq!(zero, Key::from(router_hash));

    // mint scspr, and wise to test_contract
    let _:() = scspr.call_contract(Sender(owner), "mint", runtime_args!{"account" => test_contract.test_contract_package_hash(), "amount" => U256::from("900000000000000000000")});
    let _:() = wise_contract.call_contract(Sender(owner), "mint", runtime_args!{"account" => test_contract.test_contract_package_hash(), "amount" => U256::from("900000000000000000000")});
    // create pair of wcspr and scspr
    let pair: TestContract = deploy_pair_contract(
        &env,
        owner,
        Key::Hash(factory_contract.contract_hash()),
        Key::Hash(flash_swapper.contract_hash()),
    );
    add_liquidity_cspr(
        &test_contract,
        &owner,
        &scspr,
        &Key::from(user),
        &Key::Hash(router_contract.contract_hash()),
        &Key::Hash(pair.contract_hash()),
    );
    // create pair of scspr and wise contract
    let pair: TestContract = deploy_pair_contract(
        &env,
        owner,
        Key::Hash(factory_contract.contract_hash()),
        Key::Hash(flash_swapper.contract_hash()),
    );
    add_liquidity(
        &test_contract,
        &owner,
        &scspr,
        &wise_contract,
        &Key::from(user),
        &Key::Hash(router_contract.contract_hash()),
        &Key::Hash(pair.contract_hash()),
    );

    let test_hash: Key = test_contract.test_contract_hash();
    let lock_days: u64 = 15;
    let referrer: Key = Key::from(owner);
    let amount: U256 = 40.into();

    test_contract.create_stake_with_cspr(Sender(owner), test_hash, lock_days, referrer, amount);
}

#[test]
fn extend_lt_auction() {
    let (
        env,
        owner,
        wise_contract,
        test_contract,
        _,
        flash_swapper,
        factory_contract,
        router_contract,
        wcspr,
        scspr,
    ) = deploy_wise();
    let user = env.next_user();

    test_contract
        .set_liquidity_transfomer(Sender(owner), test_contract.test_contract_package_hash()); // make test contract as liquidity_transformer
    test_contract.extend_lt_auction(Sender(owner));
}

fn add_liquidity_cspr(
    test_contract: &WiseTestInstance,
    owner: &AccountHash,
    token: &TestContract,
    to: &Key,
    router_hash: &Key,
    pair: &Key,
) {
    let amount_token_desired: U256 = U256::from("300000000000000000000");
    let amount_cspr_desired: U256 = U256::from("500");
    let amount_token_min: U256 = U256::from("200000000000000000");
    let amount_cspr_min: U256 = U256::from("300");
    let self_hash = test_contract.test_contract_hash();

    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    test_contract.add_liquidity_cspr_to_router(
        Sender(*owner),
        *router_hash,
        *&Key::Hash(token.contract_hash()),
        amount_token_desired,
        amount_cspr_desired,
        amount_token_min,
        amount_cspr_min,
        *to,
        U256::from(deadline),
        Some(*pair),
        self_hash,
    )
}

fn add_liquidity(
    test_contract: &WiseTestInstance,
    owner: &AccountHash,
    token_a: &TestContract,
    token_b: &TestContract,
    to: &Key,
    router_hash: &Key,
    pair: &Key,
) {
    let amount_a_desired: U256 = U256::from("300000000000000000000");
    let amount_b_desired: U256 = U256::from("300000000000000000000");
    let amount_a_min: U256 = U256::from("200000000000000000");
    let amount_b_min: U256 = U256::from("200000000000000000");

    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    test_contract.add_liquidity_to_router(
        Sender(*owner),
        *router_hash,
        *&Key::Hash(token_a.contract_hash()),
        *&Key::Hash(token_b.contract_hash()),
        amount_a_desired,
        amount_b_desired,
        amount_a_min,
        amount_b_min,
        *to,
        U256::from(deadline),
        Some(*pair),
    )
}
