use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U256, U512};
use casperlabs_test_env::{TestContract, TestEnv};
use num_traits::AsPrimitive;
use tests_common::{deploys::*, helpers::*, keys::*};

#[allow(clippy::type_complexity)]
fn deploy() -> (
    TestEnv,
    TestContract,
    AccountHash,
    TestContract,
    TestContract,
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
    let wcspr = deploy_wcspr(&env, owner, "Wrapped CSPR".into(), "WCSPR".into(), 9, now());
    let uniswap_library = deploy_uniswap_library(&env, owner, now());
    let uniswap_factory = deploy_uniswap_factory(&env, owner, Key::Account(owner), now());
    let uniswap_router = deploy_uniswap_router(
        &env,
        owner,
        &uniswap_factory,
        &wcspr,
        &uniswap_library,
        now(),
    );
    let erc20 = deploy_erc20(
        &env,
        owner,
        "erc20_token".into(),
        "ERC20".into(),
        9,
        0.into(),
        now(),
    );
    let flash_swapper = deploy_flash_swapper(&env, owner, &wcspr, &erc20, &uniswap_factory, now());
    let pair_scspr: TestContract = deploy_uniswap_pair(
        &env,
        owner,
        "pair-1",
        "scspr_wcspr_pair".into(),
        "SWP".into(),
        9,
        0.into(),
        &flash_swapper,
        &uniswap_factory,
        now(),
    );
    let pair_stakeable: TestContract = deploy_uniswap_pair(
        &env,
        owner,
        "pair-2",
        "stakeable_wcspr_pair".into(),
        "STWP".into(),
        9,
        0.into(),
        &flash_swapper,
        &uniswap_factory,
        now(),
    );
    let liquidity_guard = deploy_liquidity_guard(&env, owner, now());
    let scspr = deploy_scspr(
        &env,
        owner,
        &wcspr,
        &pair_scspr,
        &uniswap_router,
        &uniswap_factory,
        SCSPR_AMOUNT,
        now(),
    );
    let stakeable_token = deploy_stakeable(
        &env,
        owner,
        &erc20,
        &scspr,
        &wcspr,
        &uniswap_router,
        &uniswap_factory,
        &pair_stakeable,
        &liquidity_guard,
        STAKEABLE_AMOUNT,
        now() - (2 * MILLI_SECONDS_IN_DAY), // 172800000 == 2 days in ms (launch time set in past for testing)
    );
    let liquidity_transformer = deploy_liquidity_transformer(
        &env,
        "LIQUIDITY_TRANSFORMER",
        owner,
        Key::Hash(stakeable_token.package_hash()),
        Key::Hash(scspr.package_hash()),
        Key::Hash(pair_stakeable.package_hash()),
        Key::Hash(pair_scspr.package_hash()),
        Key::Hash(uniswap_router.package_hash()),
        Key::Hash(wcspr.package_hash()),
        TRANSFORMER_AMOUNT,
        now(),
    );
    (
        env,
        liquidity_transformer,
        owner,
        erc20,
        wcspr,
        uniswap_router,
        pair_scspr,
        stakeable_token,
        scspr,
        uniswap_factory,
        pair_stakeable,
        flash_swapper,
        liquidity_guard,
    )
}

fn add_liquidity(
    env: &TestEnv,
    owner: AccountHash,
    erc20: &TestContract,
    uniswap_router: &TestContract,
    uniswap_pair: &TestContract,
    wcspr: &TestContract,
    time: u64,
) {
    const AMOUNT: u128 = 100_000_000_000_000;
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Account(owner),
            "amount" => U256::from(AMOUNT)
        },
        time,
    );
    call(
        env,
        owner,
        SESSION_WASM_LIQUIDITY_TRANSFORMER,
        runtime_args! {
            ENTRYPOINT => "deposit_no_return",
            PACKAGE_HASH => Key::Hash(wcspr.package_hash()),
            "amount" => U512::from(AMOUNT),
        },
        time,
    );
    erc20.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::Hash(uniswap_router.package_hash()),
            "amount" => U256::from(AMOUNT)
        },
        time,
    );
    wcspr.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::Hash(uniswap_router.package_hash()),
            "amount" => U512::from(AMOUNT)
        },
        time,
    );
    let deadline = time + (30 * 60 * MILLI_SECONDS_IN_DAY);
    uniswap_router.call_contract(
        owner,
        "add_liquidity",
        runtime_args! {
            "token_a" => Key::Hash(erc20.package_hash()),
            "token_b" => Key::Hash(wcspr.package_hash()),
            "amount_a_desired" => U256::from(1_000_000_000_000_u128),
            "amount_b_desired" => U256::from(1_000_000_000_000_u128),
            "amount_a_min" => U256::from(100_000_000_000_u128),
            "amount_b_min" => U256::from(100_000_000_000_u128),
            "to" => Key::Hash(uniswap_pair.package_hash()),
            "pair" => Some(Key::Hash(uniswap_pair.package_hash())),
            "deadline" => U256::from(deadline),
        },
        time,
    );
}

#[allow(clippy::too_many_arguments)]
fn forward_liquidity(
    env: &TestEnv,
    lt: &TestContract,
    owner: AccountHash,
    token: &TestContract,
    scspr: &TestContract,
) -> u64 {
    scspr.call_contract(
        owner,
        "set_wise",
        runtime_args! {
            "wise" => Key::Hash(token.package_hash())
        },
        now(),
    );
    // Using session code as transformer purse fetch with access is required
    call(
        env,
        owner,
        SESSION_WASM_LIQUIDITY_TRANSFORMER,
        runtime_args! {
            ENTRYPOINT => "set_liquidity_transfomer",
            PACKAGE_HASH => Key::Hash(token.package_hash()),
            "immutable_transformer" => Key::Hash(lt.package_hash()),
        },
        now(),
    );
    // Forward liquidity to be done after investment days
    const INVESTMENT_DAY: u64 = 20 * MILLI_SECONDS_IN_DAY;
    lt.call_contract(
        owner,
        "forward_liquidity",
        runtime_args! {},
        now() + INVESTMENT_DAY,
    );
    now() + INVESTMENT_DAY
}

fn init() -> (TestEnv, AccountHash, TestContract, u64) {
    let (
        env,
        liquidity_transformer,
        owner,
        erc20,
        wcspr,
        uniswap_router,
        _,
        wise,
        scspr,
        uniswap_factory,
        _,
        flashswapper,
        liquidity_guard,
    ) = deploy();
    let stable_usd_wcspr_pair = deploy_uniswap_pair(
        &env,
        owner,
        "pair-3",
        "stable_usd_wcspr_pair".into(),
        "SUWP".into(),
        9,
        0.into(),
        &flashswapper,
        &uniswap_factory,
        now(),
    );
    uniswap_factory.call_contract(
        owner,
        "set_white_list",
        runtime_args! {
            "white_list" => Key::Hash(uniswap_router.package_hash())
        },
        now(),
    );
    call(
        &env,
        owner,
        SESSION_WASM_LIQUIDITY_TRANSFORMER,
        runtime_args! {
            ENTRYPOINT => "reserve_wise",
            PACKAGE_HASH => Key::Hash(liquidity_transformer.package_hash()),
            "investment_mode" => 1_u8,
            "amount" => TWOTHOUSEND_CSPR
        },
        now(),
    );
    let time = forward_liquidity(&env, &liquidity_transformer, owner, &wise, &scspr);
    let balance: U256 = wise
        .query_dictionary("balances", key_to_str(&Key::Account(owner)))
        .unwrap_or_default();
    assert_eq!(balance, 0.into(), "Already have some wise tokens");
    liquidity_transformer.call_contract(owner, "get_my_tokens", runtime_args! {}, time);
    let balance: U256 = wise
        .query_dictionary("balances", key_to_str(&Key::Account(owner)))
        .unwrap_or_default();
    assert_eq!(
        balance,
        2640002000000000u64.into(), // calculated amount in contract
        "Tokens not transfered to owner"
    );
    liquidity_guard.call_contract(owner, "assign_inflation", runtime_args! {}, time);
    add_liquidity(
        &env,
        owner,
        &erc20,
        &uniswap_router,
        &stable_usd_wcspr_pair,
        &wcspr,
        time,
    );

    (env, owner, wise, time)
}

#[test]
fn should_be_able_to_create_stake_with_cspr() {
    let (env, owner, wise, time) = init();
    call(
        &env,
        owner,
        SESSION_WASM_STAKEABLE,
        runtime_args! {
            ENTRYPOINT => CREATE_STAKE_WITH_CSPR,
            PACKAGE_HASH => Key::Hash(wise.package_hash()),
            "lock_days" => 20u64,
            "referrer" => account_zero_address(),
            "amount" => <casper_types::U256 as AsPrimitive<casper_types::U512>>::as_(ONETHOUSEND_CSPR)
        },
        time,
    );
}

#[test]
fn should_be_able_create_end_stake_and_scrape_interest() {
    let (env, owner, wise, mut time) = init();
    // CREATE STAKE
    call(
        &env,
        owner,
        SESSION_WASM_STAKEABLE,
        runtime_args! {
            ENTRYPOINT => CREATE_STAKE,
            PACKAGE_HASH => Key::Hash(wise.package_hash()),
            "staked_amount" => ONETHOUSEND_CSPR,
            "lock_days" => 20u64,
            "referrer" => account_zero_address()
        },
        time,
    );
    // STAKE_ID / START_DATE / REFERAL_ID
    let ret: (Vec<u32>, U256, Vec<u32>) = result_key(&env, owner, CREATE_STAKE);
    // SCRAPE INTEREST
    time += 2 * MILLI_SECONDS_IN_DAY;
    wise.call_contract(owner, "manual_daily_snapshot", runtime_args! {}, time);
    call(
        &env,
        owner,
        SESSION_WASM_STAKEABLE,
        runtime_args! {
            ENTRYPOINT => SCRAPE_INTEREST,
            PACKAGE_HASH => Key::Hash(wise.package_hash()),
            "stake_id" => ret.0.clone(),
            "scrape_days" => 1u64
        },
        time,
    );
    // END STAKE
    call(
        &env,
        owner,
        SESSION_WASM_STAKEABLE,
        runtime_args! {
            ENTRYPOINT => END_STAKE,
            PACKAGE_HASH => Key::Hash(wise.package_hash()),
            "stake_id" => ret.0
        },
        time,
    );
}
