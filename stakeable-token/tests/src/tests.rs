use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U256, U512};
use casperlabs_test_env::{TestContract, TestEnv};
use num_traits::AsPrimitive;
use tests_common::{data::Globals, deploys::*, helpers::*, keys::*};

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
    u64,
) {
    let time = now();
    let env = TestEnv::new();
    let owner = env.next_user();
    let wcspr = deploy_wcspr(&env, owner, "Wrapped CSPR".into(), "WCSPR".into(), 9, time);
    let uniswap_library = deploy_uniswap_library(&env, owner, time);
    let uniswap_factory = deploy_uniswap_factory(&env, owner, Key::Account(owner), time);
    let uniswap_router = deploy_uniswap_router(
        &env,
        owner,
        &uniswap_factory,
        &wcspr,
        &uniswap_library,
        time,
    );
    let erc20 = deploy_erc20(
        &env,
        owner,
        "erc20_token".into(),
        "ERC20".into(),
        9,
        0.into(),
        time,
    );
    let flash_swapper = deploy_flash_swapper(&env, owner, &wcspr, &erc20, &uniswap_factory, time);
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
        time,
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
        time,
    );
    let liquidity_guard = deploy_liquidity_guard(&env, owner, time);
    let scspr = deploy_scspr(
        &env,
        owner,
        &wcspr,
        &pair_scspr,
        &uniswap_router,
        &uniswap_factory,
        SCSPR_AMOUNT,
        time,
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
        time - (2 * MILLI_SECONDS_IN_DAY), // 172800000 == 2 days in ms (launch time set in past for testing)
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
        time,
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
        time,
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
    const AMOUNT: u128 = 100_000_000_000;
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
            "amount_a_desired" => U256::from(10_000_000_000_u128),
            "amount_b_desired" => U256::from(10_000_000_000_u128),
            "amount_a_min" => U256::from(1_000_000_000_u128),
            "amount_b_min" => U256::from(1_000_000_000_u128),
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
    time: u64,
) -> u64 {
    scspr.call_contract(
        owner,
        "set_wise",
        runtime_args! {
            "wise" => Key::Hash(token.package_hash())
        },
        time,
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
        time,
    );
    // Forward liquidity to be done after investment days
    const INVESTMENT_DAY: u64 = 20 * MILLI_SECONDS_IN_DAY;
    lt.call_contract(
        owner,
        "forward_liquidity",
        runtime_args! {},
        time + INVESTMENT_DAY,
    );
    time + INVESTMENT_DAY
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
        time,
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
        time,
    );
    uniswap_factory.call_contract(
        owner,
        "set_white_list",
        runtime_args! {
            "white_list" => Key::Hash(uniswap_router.package_hash())
        },
        time,
    );
    call(
        &env,
        owner,
        SESSION_WASM_LIQUIDITY_TRANSFORMER,
        runtime_args! {
            ENTRYPOINT => "reserve_wise",
            PACKAGE_HASH => Key::Hash(liquidity_transformer.package_hash()),
            "investment_mode" => 1_u8,
            "amount" => TWOHUNDRED_CSPR
        },
        time,
    );
    let time = forward_liquidity(&env, &liquidity_transformer, owner, &wise, &scspr, time);
    let balance: U256 = wise
        .query_dictionary("balances", key_to_str(&Key::Account(owner)))
        .unwrap_or_default();
    assert_eq!(balance, 0.into(), "Already have some wise tokens");
    liquidity_transformer.call_contract(owner, "get_my_tokens", runtime_args! {}, time);
    let balance: U256 = wise
        .query_dictionary("balances", key_to_str(&Key::Account(owner)))
        .unwrap_or_default();
    assert_eq!(balance, RESERVED_WISE, "Tokens not transfered to owner");
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

fn default_check(wise: &TestContract, owner: AccountHash) {
    let ret: Globals = wise.query_named_key(GLOBALS.into());
    let balance: U256 = wise
        .query_dictionary("balances", owner.to_string())
        .unwrap_or_default();
    assert_eq!(ret, DEFAULT_GLOBALS, "Not default globals");
    assert_eq!(balance, RESERVED_WISE, "Not default wise amount");
}

// #[test]
fn should_be_able_to_create_stake_with_cspr() {
    let (env, owner, wise, time) = init();
    default_check(&wise, owner);
    call(
        &env,
        owner,
        SESSION_WASM_STAKEABLE,
        runtime_args! {
            ENTRYPOINT => CREATE_STAKE_WITH_CSPR,
            PACKAGE_HASH => Key::Hash(wise.package_hash()),
            "lock_days" => 20u64,
            "referrer" => account_zero_address(),
            "amount" => <casper_types::U256 as AsPrimitive<casper_types::U512>>::as_(ONEHUNDRED_CSPR)
        },
        time,
    );
    let ret: Globals = wise.query_named_key(GLOBALS.into());
    assert_eq!(
        ret,
        Globals {
            total_staked: 87823823823823u64.into(),
            total_shares: 880644370373725u64.into(),
            share_price: 100000000.into(),
            current_stakeable_day: 22.into(),
            referral_shares: 0.into(),
            liquidity_shares: 0.into(),
        },
        "Globals not updated accordingly"
    );
}

#[test]
fn should_be_able_to_create_stake_and_end_stake_immature_no_penalty() {
    let (env, owner, wise, time) = init();
    default_check(&wise, owner);
    // CREATE STAKE
    call(
        &env,
        owner,
        SESSION_WASM_STAKEABLE,
        runtime_args! {
            ENTRYPOINT => CREATE_STAKE,
            PACKAGE_HASH => Key::Hash(wise.package_hash()),
            "staked_amount" => ONEHUNDRED_CSPR,
            "lock_days" => 20u64,
            "referrer" => account_zero_address()
        },
        time,
    );
    let ret: Globals = wise.query_named_key(GLOBALS.into());
    let balance: U256 = wise
        .query_dictionary("balances", owner.to_string())
        .unwrap_or_default();
    assert_eq!(
        balance,
        RESERVED_WISE - ONEHUNDRED_CSPR,
        "Required amount not staked for owner"
    );
    assert_eq!(
        ret,
        Globals {
            total_staked: ONEHUNDRED_CSPR,
            total_shares: 1002739726000u64.into(),
            share_price: 100000000.into(),
            current_stakeable_day: 22.into(),
            referral_shares: 0.into(),
            liquidity_shares: 0.into(),
        },
        "Globals not updated accordingly"
    );
    // STAKE_ID / START_DATE / REFERAL_ID
    let ret: (Vec<u32>, U256, Vec<u32>) = result_key(&env, owner, CREATE_STAKE);
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
    let ret: Globals = wise.query_named_key(GLOBALS.into());
    let balance: U256 = wise
        .query_dictionary("balances", owner.to_string())
        .unwrap();
    assert_eq!(
        balance, RESERVED_WISE,
        "Required amount not unstaked for owner (immature stake, no penalty)"
    );
    assert_eq!(
        ret,
        Globals {
            total_staked: 0.into(),
            total_shares: 0.into(),
            share_price: 100000000.into(),
            current_stakeable_day: 22.into(),
            referral_shares: 0.into(),
            liquidity_shares: 0.into(),
        },
        "Globals not updated accordingly"
    );
}

#[test]
fn should_be_able_to_create_stake_and_end_stake_immature_penalty() {
    let (env, owner, wise, time) = init();
    default_check(&wise, owner);
    let ret: Globals = wise.query_named_key(GLOBALS.into());
    let balance: U256 = wise
        .query_dictionary("balances", owner.to_string())
        .unwrap_or_default();
    assert_eq!(ret, DEFAULT_GLOBALS, "Not default globals");
    assert_eq!(balance, RESERVED_WISE, "Not default wise amount");
    // CREATE STAKE
    call(
        &env,
        owner,
        SESSION_WASM_STAKEABLE,
        runtime_args! {
            ENTRYPOINT => CREATE_STAKE,
            PACKAGE_HASH => Key::Hash(wise.package_hash()),
            "staked_amount" => ONEHUNDRED_CSPR,
            "lock_days" => 20u64,
            "referrer" => account_zero_address()
        },
        time,
    );
    let ret: Globals = wise.query_named_key(GLOBALS.into());
    let balance: U256 = wise
        .query_dictionary("balances", owner.to_string())
        .unwrap();
    assert_eq!(
        balance,
        RESERVED_WISE - ONEHUNDRED_CSPR,
        "Required amount not staked for owner"
    );
    assert_eq!(
        ret,
        Globals {
            total_staked: ONEHUNDRED_CSPR,
            total_shares: 1002739726000u64.into(),
            share_price: 100000000.into(),
            current_stakeable_day: 22.into(),
            referral_shares: 0.into(),
            liquidity_shares: 0.into(),
        },
        "Globals not updated accordingly"
    );
    // STAKE_ID / START_DATE / REFERAL_ID
    let ret: (Vec<u32>, U256, Vec<u32>) = result_key(&env, owner, CREATE_STAKE);
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
        time + (10 * MILLI_SECONDS_IN_DAY),
    );
    let ret: Globals = wise.query_named_key(GLOBALS.into());
    let balance: U256 = wise
        .query_dictionary("balances", owner.to_string())
        .unwrap();
    assert_eq!(
        balance,
        264332746893298u64.into(), // NOT FULY RECOVERED DUE TO PENALTY
        "Required amount not unstaked for owner (immature stake, no penalty)"
    );
    assert_eq!(
        ret,
        Globals {
            total_staked: 0.into(),
            total_shares: 0.into(),
            share_price: 110000000.into(),
            current_stakeable_day: 32.into(),
            referral_shares: 0.into(),
            liquidity_shares: 0.into(),
        },
        "Globals not updated accordingly"
    );
}

#[test]
fn should_be_able_to_create_stake_and_scrape_interest() {
    let (env, owner, wise, time) = init();
    default_check(&wise, owner);
    // CREATE STAKE
    call(
        &env,
        owner,
        SESSION_WASM_STAKEABLE,
        runtime_args! {
            ENTRYPOINT => CREATE_STAKE,
            PACKAGE_HASH => Key::Hash(wise.package_hash()),
            "staked_amount" => ONEHUNDRED_CSPR,
            "lock_days" => 20u64,
            "referrer" => account_zero_address()
        },
        time,
    );
    let ret: Globals = wise.query_named_key(GLOBALS.into());
    let balance: U256 = wise
        .query_dictionary("balances", owner.to_string())
        .unwrap_or_default();
    assert_eq!(
        balance,
        RESERVED_WISE - ONEHUNDRED_CSPR,
        "Required amount not staked for owner"
    );
    assert_eq!(
        ret,
        Globals {
            total_staked: ONEHUNDRED_CSPR,
            total_shares: 1002739726000u64.into(),
            share_price: 100000000.into(),
            current_stakeable_day: 22.into(),
            referral_shares: 0.into(),
            liquidity_shares: 0.into(),
        },
        "Globals not updated accordingly"
    );
    // STAKE_ID / START_DATE / REFERAL_ID
    let ret: (Vec<u32>, U256, Vec<u32>) = result_key(&env, owner, CREATE_STAKE);
    // SCRAPE INTEREST
    call(
        &env,
        owner,
        SESSION_WASM_STAKEABLE,
        runtime_args! {
            ENTRYPOINT => SCRAPE_INTEREST,
            PACKAGE_HASH => Key::Hash(wise.package_hash()),
            "stake_id" => ret.0.clone(),
            "scrape_days" => 0u64
        },
        time + (3 * MILLI_SECONDS_IN_DAY),
    );
    let ret: Globals = wise.query_named_key(GLOBALS.into());
    let balance: U256 = wise
        .query_dictionary("balances", owner.to_string())
        .unwrap_or_default();
    assert_eq!(
        balance,
        263985521531844u64.into(), // AFTER AMOUNT SCRAPED
        "Required amount not scraped for owner"
    );
    assert_eq!(
        ret,
        Globals {
            total_staked: ONEHUNDRED_CSPR,
            total_shares: 59894125637u64.into(),
            share_price: 110000000.into(),
            current_stakeable_day: 25.into(),
            referral_shares: 0.into(),
            liquidity_shares: 0.into(),
        },
        "Globals not updated accordingly"
    );
}
