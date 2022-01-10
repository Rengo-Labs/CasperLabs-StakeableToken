use crate::busd_equivalent_instance::BUSDEquivalentInstance;
use crate::constants::*;
use casper_engine_test_support::AccountHash;
use casper_types::{runtime_args, ContractPackageHash, Key, RuntimeArgs, U256, U512};
use std::time::{SystemTime, UNIX_EPOCH};
use test_env::{Sender, TestContract, TestEnv};
use wise_token_utils::commons::key_names;

fn deploy_liquidity_guard(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        &env,
        "liquidity_guard.wasm",
        "liquidity_guard",
        Sender(owner),
        runtime_args! {},
    )
}

fn deploy_bep20(env: &TestEnv, owner: AccountHash, name: &str, symbol: &str) -> TestContract {
    let decimals: u8 = 18;
    let supply: U256 = 1000.into();

    TestContract::new(
        &env,
        "bep20-token.wasm",
        "bep20",
        Sender(owner),
        runtime_args! {
            "initial_supply" => supply,
            "name" => name.to_string(),
            "symbol" => symbol.to_string(),
            "decimals" => decimals
        },
    )
}

fn deploy_wcspr(env: &TestEnv, owner: AccountHash) -> TestContract {
    let decimals: u8 = 18;
    TestContract::new(
        &env,
        "wcspr-token.wasm",
        "wcspr",
        Sender(owner),
        runtime_args! {
            "name" => "wcspr",
            "symbol" => "ERC",
            "decimals" => decimals
        },
    )
}

fn deploy_dai(env: &TestEnv, owner: AccountHash) -> TestContract {
    let decimals: u8 = 18;
    // deploy wcspr contract
    TestContract::new(
        &env,
        "wcspr-token.wasm",
        "dai",
        Sender(owner),
        runtime_args! {
            "name" => "dai",
            "symbol" => "dai",
            "decimals" => decimals
        },
    )
}

fn deploy_flash_swapper(
    env: &TestEnv,
    owner: AccountHash,
    uniswap_v2_factory: Key,
    wcspr: Key,
    dai: Key,
) -> TestContract {
    // deploy flash swapper
    TestContract::new(
        &env,
        "flash-swapper.wasm",
        "flash_swapper",
        Sender(owner),
        runtime_args! {
            "uniswap_v2_factory" => uniswap_v2_factory,
            "wcspr" => wcspr,
            "dai" => dai
        },
    )
}

fn deploy_pair(
    env: &TestEnv,
    owner: AccountHash,
    factory_contract: Key,
    flash_swapper: Key,
) -> TestContract {
    let decimals: u8 = 18;
    let init_total_supply: U256 = 0.into();

    TestContract::new(
        &env,
        "pair-token.wasm",
        "pair",
        Sender(owner),
        runtime_args! {
            "name" => "pair",
            "symbol" => "pair",
            "decimals" => decimals,
            "initial_supply" => init_total_supply,
            "factory_hash" => factory_contract,
            "callee_contract_hash" => flash_swapper
        },
    )
}
fn deploy_uniswap_router(
    env: &TestEnv,
    owner: AccountHash,
    library: Key,
    wcspr: Key,
    factory: Key,
) -> TestContract {
    // Deploy Router Contract
    TestContract::new(
        &env,
        "uniswap-v2-router.wasm",
        "router",
        Sender(owner),
        runtime_args! {
            "factory" => factory,
            "wcspr" => wcspr,
            "library" => library
        },
    )
}

fn deploy_factory(env: &TestEnv, owner: AccountHash) -> (TestContract, TestContract) {
    let token = deploy_erc20(&env, owner, "erc20", "erc");
    let factory = TestContract::new(
        &env,
        "factory.wasm",
        "factory",
        Sender(owner),
        runtime_args! {
            "fee_to_setter" => Key::Hash(token.contract_hash())
        },
    );
    (factory, token)
}

fn deploy_library(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        &env,
        "uniswap-v2-library.wasm",
        "library",
        Sender(owner),
        runtime_args! {},
    )
}

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
    synthetic_helper: Key,
    bep20: Key,
    pair: Key,
    router: Key,
    wbnb: Key,
) -> TestContract {
    TestContract::new(
        &env,
        "synthetic_token.wasm",
        "synthetic_token",
        Sender(owner),
        runtime_args! {
            "wbnb"=>wbnb,
            "uniswap_pair"=>pair,
            "uniswap_router"=>router,
            "bep20"=>bep20,
            "synthetic_helper"=>synthetic_helper
        },
    )
}

fn deploy_synthetic_bnb(
    env: &TestEnv,
    owner: AccountHash,
    factory: Key,
    synthetic_helper: Key,
    synthetic_token: Key,
    bep20: Key,
) -> TestContract {
    TestContract::new(
        &env,
        "sbnb.wasm",
        "sbnb",
        Sender(owner),
        runtime_args! {
            "bep20" => bep20,
            "uniswap_factory" => factory,
            "synthetic_helper" => synthetic_helper,
            "synthetic_token" => synthetic_token
        },
    )
}

fn deploy_wbnb(env: &TestEnv, owner: AccountHash, name: &str, symbol: &str) -> TestContract {
    TestContract::new(
        &env,
        "wbnb-token.wasm",
        "wbnb",
        Sender(owner),
        runtime_args! {
            "name" => "wbnb",
            "symbol" => "ERC",
        },
    )
}

fn deploy_busd_equivalent() -> (
    TestEnv,
    AccountHash,
    BUSDEquivalentInstance,
    BUSDEquivalentInstance,
    TestContract,
    TestContract,
    TestContract,
    TestContract,
    TestContract,
    TestContract,
    TestContract,
) {
    let env = TestEnv::new();
    let owner = env.next_user();

    // deploy contract with no dependencies
    let liquidity_guard = deploy_liquidity_guard(&env, owner);
    let bep20 = deploy_bep20(&env, owner, "bep20", "BEP");
    let library = deploy_library(&env, owner);
    let wcspr = deploy_wcspr(&env, owner);
    let dai = deploy_dai(&env, owner);
    let wbnb = deploy_erc20(&env, owner, "wbnb", "wbnb");
    let busd = deploy_erc20(&env, owner, "BUSD", "BUSD");
    // deploying declaration

    let (factory, factory_token) = deploy_factory(&env, owner);
    let flash_swapper = deploy_flash_swapper(
        &env,
        owner,
        Key::Hash(factory.contract_hash()),
        Key::Hash(wcspr.contract_hash()),
        Key::Hash(dai.contract_hash()),
    );
    let router = deploy_uniswap_router(
        &env,
        owner,
        Key::Hash(library.contract_hash()),
        Key::Hash(wcspr.contract_hash()),
        Key::Hash(factory.contract_hash()),
    );
    let pair = deploy_pair(
        &env,
        owner,
        Key::Hash(factory.contract_hash()),
        Key::Hash(flash_swapper.contract_hash()),
    );
    let synthetic_helper = deploy_synthetic_helper(&env, owner);
    let synthetic_token = deploy_synthetic_token(
        &env,
        owner,
        Key::Hash(synthetic_helper.contract_hash()),
        Key::Hash(bep20.contract_hash()),
        Key::Hash(pair.contract_hash()),
        Key::Hash(router.contract_hash()),
        Key::Hash(wbnb.contract_hash()),
    );
    // let synthetic_bnb = deploy_synthetic_bnb(
    //     &env,
    //     owner,
    //     Key::Hash(factory.contract_hash()),
    //     Key::Hash(synthetic_helper.contract_hash()),
    //     Key::Hash(synthetic_token.contract_hash()),
    //     Key::Hash(bep20.contract_hash()),
    // );
    let synthetic_bnb = deploy_erc20(&env, owner, "Synthetic BNB", "sbnb");
    let wise = deploy_erc20(&env, owner, "WISE token", "wise");
    let busd_equivalent = BUSDEquivalentInstance::new(
        &env,
        "busd_equivalent",
        Sender(owner),
        Key::Hash(wise.contract_hash()),
        Key::Hash(synthetic_bnb.contract_hash()),
        Key::Hash(wbnb.contract_hash()),
        Key::Hash(busd.contract_hash()),
        Key::Hash(router.contract_hash()),
        Key::Hash(factory.contract_hash()),
    );

    // router package hash
    let router_package_hash: ContractPackageHash =
        router.query_named_key("package_hash".to_string());

    // set router to factory's whitelist
    factory.call_contract(
        Sender(owner),
        "set_white_list",
        runtime_args! {"white_list" => Key::from(router_package_hash)},
    );

    // let busd_equivalent_key: Key = Key::Hash(busd_equivalent.contract_hash());
    let busd_equivalent = BUSDEquivalentInstance::instance(busd_equivalent);
    let busd_equivalent_key: Key = busd_equivalent.contract_hash_result();
    // deploy proxy
    let proxy: TestContract =
        BUSDEquivalentInstance::proxy(&env, Sender(owner), Key::from(busd_equivalent_key));

    let proxy_package_hash: ContractPackageHash = proxy.query_named_key("package_hash".to_string());
    // factory_token.call_contract(
    //     Sender(owner),
    //     "mint",
    //     runtime_args! {
    //         "to"=>Key::from(proxy_package_hash),
    //         "amount"=>U256::from(100000)
    //     },
    // );
    // deploy transfer helper with transfer_invoker being proxy

    (
        env,
        owner,
        busd_equivalent,
        BUSDEquivalentInstance::instance(proxy),
        wise,
        synthetic_bnb,
        wbnb,
        busd,
        router,
        factory,
        flash_swapper,
    )
}

#[test]
fn test_deploy() {
    let (_, _, _, _, _, _, _, _, _, _, _) = deploy_busd_equivalent();
}

#[test]
fn test_get_busd_equivalent() {
    let (
        env,
        owner,
        busd_equivalent,
        proxy,
        wise,
        sbnb,
        wbnb,
        busd,
        router,
        factory,
        flash_swapper,
    ) = deploy_busd_equivalent();
    let user = env.next_user();
    let proxy_package_hash: ContractPackageHash = proxy.package_hash_result();
    let proxy_package_hash_as_key: Key = Key::from(proxy_package_hash);
    let factory_key: Key = Key::Hash(factory.contract_hash());
    let flash_swapper_key: Key = Key::Hash(flash_swapper.contract_hash());
    let router_key: Key = Key::Hash(router.contract_hash());
    // for adding liquidity
    let amount_a_desired: U256 = U256::from("100000000000000000000");
    let amount_b_desired: U256 = U256::from("100000000000000000000");
    let amount_a_min: U256 = U256::from("10000000000000000");
    let amount_b_min: U256 = U256::from("10000000000000000");
    let mint_amount: U256 = U256::from("900000000000000000000");
    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    let mut pairs: Vec<TestContract> = Vec::new();
    // all 4 are erc20 tokens for simplicity
    let path = vec![wise, sbnb, wbnb, busd]; // as is set in busd_eq contract
                                             // each token will mint to proxy contract
    for i in 0..4 {
        path[i].call_contract(
            Sender(owner),
            "mint",
            runtime_args! {
                "to"=>proxy_package_hash_as_key.clone(),
                "amount"=>mint_amount.clone()
            },
        );
    }

    let balance: U256 = proxy.balance_of(&path[1], proxy_package_hash_as_key);
    assert_eq!(balance, mint_amount);
    // // create pairs of tokens, and add liqudity
    for i in 0..3 {
        let token_a = Key::Hash(path[i].contract_hash());
        let token_b = Key::Hash(path[i + 1].contract_hash());

        let tokens_pair = deploy_pair(&env, owner, factory_key, flash_swapper_key);

        // we also need to initialize pair
        // tokens_pair.call_contract(Sender(owner), entry_point: &str, session_args: RuntimeArgs)
        //
        let tokens_pair_key: Key = Key::Hash(tokens_pair.contract_hash());
        pairs.push(tokens_pair);
        proxy.add_liquidity(
            Sender(owner),
            token_a,
            token_b,
            amount_a_desired,
            amount_b_desired,
            amount_a_min,
            amount_b_min,
            Key::Account(owner),
            U256::from(deadline),
            Some(tokens_pair_key),
            router_key,
        );
    }

    // }
    // yodas_per_wise is 0, therefore amount_in to router is 0
    // router will then revert
    proxy.get_busd_equivalent(Sender(owner));
    let ret: U256 = proxy.get_busd_equivalent_result();
    assert_ne!(ret, 0.into());
}

#[test]
fn test_update_busd_equivalent() {
    let (
        env,
        owner,
        busd_equivalent,
        proxy,
        wise,
        sbnb,
        wbnb,
        busd,
        router,
        factory,
        flash_swapper,
    ) = deploy_busd_equivalent();
    // get the latest busd eq
    let mut ret: U256 = busd_equivalent.get_update_busd_equivalent_result();
    assert_eq!(ret, 0.into());

    let user = env.next_user();
    let proxy_package_hash: ContractPackageHash = proxy.package_hash_result();
    let proxy_package_hash_as_key: Key = Key::from(proxy_package_hash);
    let factory_key: Key = Key::Hash(factory.contract_hash());
    let flash_swapper_key: Key = Key::Hash(flash_swapper.contract_hash());
    let router_key: Key = Key::Hash(router.contract_hash());
    // for adding liquidity
    let amount_a_desired: U256 = U256::from("100000000000000000000");
    let amount_b_desired: U256 = U256::from("100000000000000000000");
    let amount_a_min: U256 = U256::from("10000000000000000");
    let amount_b_min: U256 = U256::from("10000000000000000");
    let mint_amount: U256 = U256::from("900000000000000000000");
    let deadline: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis() + (1000 * (30 * 60)), // current epoch time in milisecond + 30 minutes
        Err(_) => 0,
    };

    let mut pairs: Vec<TestContract> = Vec::new();
    // all 4 are erc20 tokens for simplicity
    let path = vec![wise, sbnb, wbnb, busd]; // as is set in busd_eq contract
                                             // each token will mint to proxy contract
    for i in 0..4 {
        path[i].call_contract(
            Sender(owner),
            "mint",
            runtime_args! {
                "to"=>proxy_package_hash_as_key.clone(),
                "amount"=>mint_amount.clone()
            },
        );
    }

    let balance: U256 = proxy.balance_of(&path[1], proxy_package_hash_as_key);
    assert_eq!(balance, mint_amount);
    // // create pairs of tokens, and add liqudity
    for i in 0..3 {
        let token_a = Key::Hash(path[i].contract_hash());
        let token_b = Key::Hash(path[i + 1].contract_hash());

        let tokens_pair = deploy_pair(&env, owner, factory_key, flash_swapper_key);
        let tokens_pair_key: Key = Key::Hash(tokens_pair.contract_hash());
        pairs.push(tokens_pair);
        proxy.add_liquidity(
            Sender(owner),
            token_a,
            token_b,
            amount_a_desired,
            amount_b_desired,
            amount_a_min,
            amount_b_min,
            Key::Account(owner),
            U256::from(deadline),
            Some(tokens_pair_key),
            router_key,
        );
    }
    busd_equivalent.update_busd_equivalent(Sender(owner));
    ret = busd_equivalent.get_update_busd_equivalent_result();
    assert_ne!(ret, 0.into());
}

// #[test]
// #[should_panic]
// fn test_calling_construction() {
//     let (_, helper, _,owner, invoker) = deploy();
//     helper.constructor(Sender(owner), NAME, SYMBOL);
// }
