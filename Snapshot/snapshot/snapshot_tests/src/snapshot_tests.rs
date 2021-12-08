use crate::constants::*;
use crate::snapshot_instance::SnapshotInstance;
use casper_engine_test_support::AccountHash;
use casper_types::{runtime_args, ContractPackageHash, Key, RuntimeArgs, U256, U512};
use test_env::{Sender, TestContract, TestEnv};

fn deploy_declaration(
    env: &TestEnv,
    owner: AccountHash,
    launch_time: U256,
    uniswap_router: Key,
    factory: Key,
    pair: Key,
    liquidity_guard: Key,
    synthetic_bnb: Key,
    wbnb: Key,
) -> TestContract {
    TestContract::new(
        &env,
        "declaration.wasm",
        "declaration",
        Sender(owner),
        runtime_args! {
            "uniswap_router"=>uniswap_router,
            "pair"=>pair,
            "factory"=>factory,
            "liquidity_guard"=>liquidity_guard,
            "synthetic_bnb"=>synthetic_bnb,
            "wbnb"=>wbnb,
            "launch_time"=>launch_time
        },
    )
}

fn deploy_globals(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        &env,
        "globals.wasm",
        "globals",
        Sender(owner),
        runtime_args! {},
    )
}

fn deploy_timing(
    env: &TestEnv,
    owner: AccountHash,
    declaration_contract_hash: Key,
) -> TestContract {
    TestContract::new(
        &env,
        "timing.wasm",
        "timing",
        Sender(owner),
        runtime_args! {
            "declaration_contract_hash"=>declaration_contract_hash
        },
    )
}

fn deploy_helper(
    env: &TestEnv,
    owner: AccountHash,
    declaration: Key,
    timing: Key,
    globals: Key,
) -> TestContract {
    TestContract::new(
        &env,
        "helper.wasm",
        "helper",
        Sender(owner),
        runtime_args! {
            "declaration"=>declaration,
            "globals"=>globals,
            "timing"=>timing
        },
    )
}

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
            "symbol" => symbol.to_string()
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
            "name" => "pair".to_string(),
            "symbol" => "pair".to_string(),
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
            "library_hash" => library
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

fn deploy_snapshot(
    env: &TestEnv,
    owner: AccountHash,
    timing: Key,
    declaration: Key,
    globals: Key,
    helper: Key,
    sbnb: Key,
    pair: Key,
    bep20: Key,
    guard: Key,
) -> TestContract {
    TestContract::new(
        &env,
        "snapshot.wasm",
        "snapshot",
        Sender(owner),
        runtime_args! {
        "timing" => timing,
        "declaration" => declaration,
        "globals"=> globals,
        "helper" => helper,
        "sbnb"=>sbnb,
        "pair"=>pair,
        "bep20"=>bep20,
        "guard"=>guard
        },
    )
}

fn deploy() -> (
    TestEnv,
    AccountHash,
    SnapshotInstance,
    TestContract,
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
    let globals = deploy_globals(&env, owner);
    let library = deploy_library(&env, owner);
    let wcspr = deploy_wcspr(&env, owner);
    let dai = deploy_dai(&env, owner);
    let wbnb = deploy_wbnb(&env, owner, "wbnb", "ERC");
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
    let synthetic_bnb = deploy_synthetic_bnb(
        &env,
        owner,
        Key::Hash(factory.contract_hash()),
        Key::Hash(synthetic_helper.contract_hash()),
        Key::Hash(synthetic_token.contract_hash()),
        Key::Hash(bep20.contract_hash()),
    );
    let declaration = deploy_declaration(
        &env,
        owner,
        U256::from(0),
        Key::Hash(router.contract_hash()),
        Key::Hash(factory.contract_hash()),
        Key::Hash(pair.contract_hash()),
        Key::Hash(liquidity_guard.contract_hash()),
        Key::Hash(synthetic_bnb.contract_hash()),
        Key::Hash(wbnb.contract_hash()),
    );
    // deploy helper
    let timing = deploy_timing(&env, owner, Key::Hash(declaration.contract_hash()));
    let helper = deploy_helper(
        &env,
        owner,
        Key::Hash(declaration.contract_hash()),
        Key::Hash(timing.contract_hash()),
        Key::Hash(globals.contract_hash()),
    );

    // deploying snapshot
    let snapshot = deploy_snapshot(
        &env,
        owner,
        Key::Hash(timing.contract_hash()),
        Key::Hash(declaration.contract_hash()),
        Key::Hash(globals.contract_hash()),
        Key::Hash(helper.contract_hash()),
        Key::Hash(synthetic_bnb.contract_hash()),
        Key::Hash(pair.contract_hash()),
        Key::Hash(bep20.contract_hash()),
        Key::Hash(liquidity_guard.contract_hash()),
    );

    (env, owner, SnapshotInstance::instance(snapshot), timing, declaration, globals, helper, synthetic_bnb, pair, bep20, liquidity_guard)
}

#[test]
fn test_deploy(){
    let (_,_,_,_,_,_,_,_,_,_,_) = deploy();
}


// #[test]
// fn test_manual_daily_snapshot(){
//     let (env, owner, snapshot, timing, declaration, globals, helper, sbnb, pair, bep20, liquidity_guard) = deploy();
//     let globals_as_snapshot_instance: SnapshotInstance::instance(globals);

//     let current_wise_day = globals.current_wise_day();
    
//     // 
//     snapshot.manual_daily_snapshot();

//     let new_current_wise_day:  U256 = globals.current_wise_day();
//     assert_eq!(new_current_wise_day, U256::from(0));
// }

// #[test]
// fn test_manual_daily_snapshot_point(){

// }