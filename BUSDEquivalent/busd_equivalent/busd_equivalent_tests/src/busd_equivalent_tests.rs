use crate::constants::*;
use crate::busd_equivalent_instance::BUSDEquivalentInstance;
use casper_engine_test_support::AccountHash;
use casper_types::{Key, U256, U512, runtime_args, RuntimeArgs, ContractPackageHash};
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
            "declaration_contract"=>declaration_contract_hash
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

fn deploy_referral_token(
    env: &TestEnv,
    owner: AccountHash,
    declaration: Key,
    timing: Key,
    helper: Key,
    bep20: Key,
    snapshot: Key
)-> TestContract{
    TestContract::new(
        &env,
        "referral-token-main.wasm",
        "referral-token",
        Sender(owner),
        runtime_args!{
            "declaration_hash"=>declaration,
            "timing_hash"=>timing,
            "helper_hash"=>helper,
            "bep20_hash"=>bep20,
            "snapshot_hash"=>snapshot,
        }
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

fn deploy_wise_token(env: &TestEnv, owner: AccountHash, declaration: Key, globals: Key, sbnb: Key, bep20: Key, router: Key, staking_token: Key, timing: Key)->TestContract{
    TestContract::new(
        &env,
        "wisetoken.wasm",
        "wise",
        Sender(owner),
        runtime_args! {
            "declaration_contract" => declaration,
            "globals_contract" => globals,
            "synthetic_bnb_address" => sbnb,
            "bep20_address" => bep20,
            "router_address" => router,
            "staking_token_address" => staking_token,
            "timing_address" => timing
        }
    )
}

fn deploy_staking_token(env: &TestEnv, owner: AccountHash, declaration: Key, timing: Key, helper: Key, globals: Key, bep20: Key, snapshot: Key, referral_token: Key)->TestContract{
    TestContract::new(
        &env,
        "staking-token-main.wasm",
        "staking-token",
        Sender(owner),
        runtime_args! {
            "declaration_hash"=>declaration,
            "timing_hash"=>timing,
            "helper_hash"=>helper,
            "globals_hash"=>globals,
            "bep20_hash"=>bep20,
            "snapshot_hash"=>snapshot,
            "referral_token_hash"=>referral_token,
        }
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
    let globals = deploy_globals(&env, owner);
    let library = deploy_library(&env, owner);
    let wcspr = deploy_wcspr(&env, owner);
    let dai = deploy_dai(&env, owner);
    let wbnb = deploy_wbnb(&env, owner, "wbnb", "ERC");
    let busd = deploy_erc20(&env, owner, "BUSD", "ERC");
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

    let timing = deploy_timing(
        &env,
        owner,
        Key::Hash(declaration.contract_hash())
    );

    let helper = deploy_helper(
        &env,
        owner,
        Key::Hash(declaration.contract_hash()), 
        Key::Hash(timing.contract_hash()), 
        Key::Hash(globals.contract_hash())
    );

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

    let referral_token = deploy_referral_token(
        &env, 
        owner,
        Key::Hash(declaration.contract_hash()),
        Key::Hash(timing.contract_hash()),
        Key::Hash(helper.contract_hash()),
        Key::Hash(bep20.contract_hash()),
        Key::Hash(snapshot.contract_hash())
    );

    let staking_token = deploy_staking_token(
        &env,
        owner,
        Key::Hash(declaration.contract_hash()), 
        Key::Hash(timing.contract_hash()), 
        Key::Hash(helper.contract_hash()), 
        Key::Hash(globals.contract_hash()), 
        Key::Hash(bep20.contract_hash()), 
        Key::Hash(snapshot.contract_hash()), 
        Key::Hash(referral_token.contract_hash())
    );
    
    let wise = deploy_wise_token(
        &env,
        owner,
        Key::Hash(declaration.contract_hash()),
        Key::Hash(globals.contract_hash()),
        Key::Hash(synthetic_bnb.contract_hash()),
        Key::Hash(bep20.contract_hash()),
        Key::Hash(router.contract_hash()),
        Key::Hash(staking_token.contract_hash()), 
        Key::Hash(timing.contract_hash())
    );

    let busd_equivalent = BUSDEquivalentInstance::new(
        &env, 
        "busd_equivalent", 
        Sender(owner),
        Key::Hash(wise.contract_hash()), 
        Key::Hash(synthetic_bnb.contract_hash()), 
        Key::Hash(wbnb.contract_hash()), 
        Key::Hash(busd.contract_hash()), 
        Key::Hash(router.contract_hash()), 
        Key::Hash(declaration.contract_hash()), 
        Key::Hash(factory.contract_hash())
    );

    // let busd_equivalent_key: Key = Key::Hash(busd_equivalent.contract_hash());
    let busd_equivalent_key: Key = busd_equivalent.query_named_key("self_hash".to_string());
    // deploy proxy
    let proxy: TestContract = BUSDEquivalentInstance::proxy(&env, Sender(owner), busd_equivalent_key);

    // deploy transfer helper with transfer_invoker being proxy

    (
        env,
        owner,
        BUSDEquivalentInstance::instance(busd_equivalent),
        BUSDEquivalentInstance::instance(proxy),
        wise,
        synthetic_bnb,
        wbnb,
        busd,
        router,
        declaration,
        factory,
    )
}

// #[test]
// fn test_deploy() {
//     let (
//         _,
//         _,
//         _,
//         _,
//         _,
//         _,
//         _,
//         _,
//         _,
//         _,
//         _,
//     ) = deploy_busd_equivalent();
// }

#[test]
#[should_panic]
fn test_get_busd_equivalent(){
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
        declaration,
        factory,
    ) = deploy_busd_equivalent();

    // yodas_per_wise is 0, therefore amount_in to router is 0
    // router will then revert
    proxy.get_busd_equivalent(Sender(owner));
}

// #[test]
// fn test_get_busd_equivalent_with_invalid_path(){
//     let (
//         env,
//         owner,
//         busd_equivalent,
//         proxy,
//         wise,
//         sbnb,
//         wbnb,
//         busd,
//         router,
//         declaration,
//         factory,
//     ) = deploy_busd_equivalent();
// }

#[test]
#[should_panic]
fn test_update_busd_equivalent(){
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
        declaration,
        factory,
    ) = deploy_busd_equivalent();

    // yodas_per_wise is 0, so contract will revert
    busd_equivalent.update_busd_equivalent(Sender(owner));
}

// #[test]
// #[should_panic]
// fn test_calling_construction() {
//     let (_, helper, _,owner, invoker) = deploy();
//     helper.constructor(Sender(owner), NAME, SYMBOL);
// }
