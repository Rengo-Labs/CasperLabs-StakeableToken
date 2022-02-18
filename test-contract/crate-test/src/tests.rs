use crate::instance::TestInstance;
use casper_engine_test_support::AccountHash;
use casper_types::{
    bytesrepr::{Bytes, FromBytes, ToBytes},
    runtime_args, ContractPackageHash, Key, RuntimeArgs, U256,
};
use std::time::{SystemTime, UNIX_EPOCH};
use test_env::{Sender, TestContract, TestEnv};
use wise_token_utils::commons::key_names::*;
use wise_token_utils::declaration::structs::*;
use wise_token_utils::key_gen;
use wise_token_utils::key_gen::generate_key_for_dictionary;
use wise_token_utils::snapshot::structs::*;
use wise_token_utils::timing;

extern crate alloc;
use alloc::{
    string::{String, ToString},
    vec::Vec,
};

pub const CURRENT_WISE_DAY: u64 = 5; // as is set by test-env

fn deploy_stable_usd_equivalent(
    env: &TestEnv,
    owner: AccountHash,
    wise: &TestContract,
    scspr: &TestContract,
    wcspr: &TestContract,
    stable_usd: &TestContract,
    router: &TestContract,
) -> TestContract {
    TestContract::new(
        &env,
        "stable_usd_equivalent.wasm",
        "stable_usd_equivalent",
        Sender(owner),
        runtime_args! {
            "wise" => Key::Hash(wise.contract_hash()),
            "scspr" => Key::Hash(scspr.contract_hash()),
            "wcspr" => Key::Hash(wcspr.contract_hash()),
            "stable_usd" => Key::Hash(stable_usd.contract_hash()),
            "router"=>Key::Hash(router.contract_hash()),
        },
    )
}
fn deploy_uniswap_router(
    env: &TestEnv,
    owner: AccountHash,
    uniswap_factory: &TestContract,
    wcspr: &TestContract,
    uniswap_library: &TestContract,
) -> TestContract {
    TestContract::new(
        &env,
        "uniswap-v2-router.wasm",
        "uniswap-v2-router",
        Sender(owner),
        runtime_args! {
            "factory" => Key::Hash(uniswap_factory.contract_hash()),
            "wcspr" => Key::Hash(wcspr.contract_hash()),
            "library" => Key::Hash(uniswap_library.contract_hash())
        },
    )
}

fn deploy_uniswap_factory(env: &TestEnv, owner: AccountHash) -> TestContract {
    let uniswap_factory = TestContract::new(
        &env,
        "factory.wasm",
        "factory",
        Sender(owner),
        runtime_args! {
            "fee_to_setter" => Key::from(owner)
        },
    );
    uniswap_factory.call_contract(
        Sender(owner),
        "set_white_list",
        runtime_args! {
            "white_list" => Key::Account(owner)
        },
    );
    uniswap_factory
}

fn deploy_uniswap_pair(
    env: &TestEnv,
    owner: AccountHash,
    flash_swapper: &TestContract,
    uniswap_factory: &TestContract,
) -> TestContract {
    TestContract::new(
        &env,
        "pair-token.wasm",
        "Pair",
        Sender(owner),
        runtime_args! {
            "name" => "pair",
            "symbol" => "PAIR",
            "decimals" => 18 as u8,
            // "initial_supply" => U256::from(404000000000000000 as u128)
            "initial_supply" => U256::from(0),
            "callee_contract_hash" => Key::Hash(flash_swapper.contract_hash()),
            "factory_hash" => Key::Hash(uniswap_factory.contract_hash()),
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

fn deploy_wcspr(env: &TestEnv, owner: AccountHash) -> TestContract {
    let decimals: u8 = 18;
    TestContract::new(
        &env,
        "wcspr-token.wasm",
        "wcspr",
        Sender(owner),
        runtime_args! {
            "name" => "Wrapper Casper",
            "symbol" => "WCSPR",
            "decimals" => decimals
        },
    )
}

fn deploy_flash_swapper(
    env: &TestEnv,
    owner: AccountHash,
    wcspr: &TestContract,
    uniswap_factory: &TestContract,
) -> TestContract {
    TestContract::new(
        &env,
        "flash-swapper.wasm",
        "flash_swapper",
        Sender(owner),
        runtime_args! {
            "wcspr" => Key::Hash(wcspr.contract_hash()),
            "dai" => Key::Hash(wcspr.contract_hash()),
            "uniswap_v2_factory" => Key::Hash(uniswap_factory.contract_hash())
        },
    )
}

fn deploy_uniswap_library(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        &env,
        "uniswap-v2-library.wasm",
        "library",
        Sender(owner),
        runtime_args! {},
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

fn deploy() -> (
    TestEnv,
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
    let wcspr = deploy_wcspr(&env, owner);
    let launch_time: U256 = 0.into();
    let uniswap_factory = deploy_uniswap_factory(&env, owner);
    let flash_swapper = deploy_flash_swapper(&env, owner, &wcspr, &uniswap_factory);
    let uniswap_pair = deploy_uniswap_pair(&env, owner, &flash_swapper, &uniswap_factory);
    let uniswap_library = deploy_uniswap_library(&env, owner);
    let uniswap_router =
        deploy_uniswap_router(&env, owner, &uniswap_factory, &wcspr, &uniswap_library);
    let liquidity_guard = deploy_liquidity_guard(&env, owner);
    let erc20 = deploy_erc20(&env, owner, "erc20 token", "erc20");
    // let synthetic_helper = deploy_synthetic_helper(&env, owner);
    let synthetic_helper = deploy_erc20(&env, owner, "erc20 token", "erc20");
    let synthetic_token = deploy_synthetic_token(
        &env,
        owner,
        &wcspr,
        &synthetic_helper,
        &uniswap_pair,
        &uniswap_router,
        &erc20,
    );
    let scspr = deploy_scspr(
        &env,
        owner,
        &erc20,
        &uniswap_factory,
        &synthetic_helper,
        &synthetic_token,
    );
    let test = TestInstance::new(
        &env,
        "test_contract",
        Sender(owner),
        launch_time,
        Key::Hash(uniswap_router.contract_hash()),
        Key::Hash(uniswap_factory.contract_hash()),
        Key::Hash(uniswap_pair.contract_hash()),
        Key::Hash(liquidity_guard.contract_hash()),
        Key::Hash(scspr.contract_hash()),
        Key::Hash(wcspr.contract_hash()),
        Key::Hash(erc20.contract_hash()),
    );
    (
        env,
        owner,
        test,
        uniswap_factory,
        wcspr,
        flash_swapper,
        uniswap_pair,
        uniswap_library,
        uniswap_router,
        liquidity_guard,
        erc20,
        scspr,
    )
}

#[test]
fn test_deploy() {
    let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
}

/////////////////
// DECLARATION //
/////////////////

/* Related crate methods cannot be unit tested,
 * as much depends on integration with crates further down the inheritance line
 * Methods are later tests as integrations.
*/
// // #[test]
// fn test_set_liquidity_stake() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     let staker: Key = Key::Account(owner);
//     let mut id: Vec<u32> = Vec::new();
//     id.push(1);
//     let _value: LiquidityStake = LiquidityStake {
//         staked_amount: U256::from(100),
//         reward_amount: U256::from(100),
//         start_day: 1,
//         close_day: 1000,
//         is_active: true,
//     };
//     let value: Vec<u8> = _value.into_bytes().unwrap();

//     test.call_contract(
//         Sender(owner),
//         "set_liquidity_stake",
//         runtime_args! {
//             "staker" => staker,
//             "id" => id,
//             "value" => Bytes::from(value),
//         },
//     );
// }

// // #[test]
// fn test_get_liquidity_stake() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     let staker: Key = Key::Account(owner);
//     let mut id: Vec<u32> = Vec::new();
//     id.push(1);
//     let _value: LiquidityStake = LiquidityStake {
//         staked_amount: U256::from(100),
//         reward_amount: U256::from(100),
//         start_day: 1,
//         close_day: 1000,
//         is_active: true,
//     };
//     let value: Vec<u8> = _value.into_bytes().unwrap();

//     test.call_contract(
//         Sender(owner),
//         "set_liquidity_stake",
//         runtime_args! {
//             "staker" => staker,
//             "id" => id,
//             "value" => Bytes::from(value.clone()),
//         },
//     );

//     let mut id: Vec<u32> = Vec::new();
//     id.push(1);
//     test.call_contract(
//         Sender(owner),
//         "get_liquidity_stake",
//         runtime_args! {
//             "staker" => staker,
//             "id" => id
//         },
//     );
//     let ret: Bytes = TestInstance::instance(test).result();
//     let ret: Vec<u8> = Vec::from(ret);
// }

// // #[test]
// fn test_launch_time() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     test.call_contract(Sender(owner), "launch_time", runtime_args! {});
//     let ret: U256 = TestInstance::instance(test).result();
// }

// // #[test]
// fn test_get_stake_count() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     let staker: Key = Key::Account(owner);
//     test.call_contract(
//         Sender(owner),
//         "get_stake_count",
//         runtime_args! {
//             "staker" => staker
//         },
//     );
//     let ret: U256 = TestInstance::instance(test).result();
// }

// // #[test]
// fn test_set_stake_count() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     let staker: Key = Key::Account(owner);
//     let value: U256 = 100.into();
//     test.call_contract(
//         Sender(owner),
//         "set_stake_count",
//         runtime_args! {
//             "staker" => staker,
//             "value" => value
//         },
//     );
// }

// // #[test]
// fn test_get_referral_count() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     let referrer: Key = Key::Account(owner);
//     test.call_contract(
//         Sender(owner),
//         "get_referral_count",
//         runtime_args! {
//             "referrer" => referrer
//         },
//     );
//     let ret: U256 = TestInstance::instance(test).result();
// }

// // #[test]
// fn test_set_referral_count() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     let referrer: Key = Key::Account(owner);
//     let value: U256 = 100.into();
//     test.call_contract(
//         Sender(owner),
//         "set_referral_count",
//         runtime_args! {
//             "referrer" => referrer,
//             "value" => value
//         },
//     );
// }

// // #[test]
// fn test_get_liquidity_stake_count() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     let staker: Key = Key::Account(owner);
//     test.call_contract(
//         Sender(owner),
//         "get_liquidity_stake_count",
//         runtime_args! {
//             "staker" => staker
//         },
//     );
//     let ret: U256 = TestInstance::instance(test).result();
// }

// // #[test]
// fn test_set_liquidity_stake_count() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     let staker: Key = Key::Account(owner);
//     let value: U256 = 100.into();
//     test.call_contract(
//         Sender(owner),
//         "set_liquidity_stake_count",
//         runtime_args! {
//             "staker" => staker,
//             "value" => value
//         },
//     );
// }

// // #[test]
// fn test_get_referral_shares_to_end() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     let key: U256 = 1.into();
//     test.call_contract(
//         Sender(owner),
//         "get_referral_shares_to_end",
//         runtime_args! {
//             "key" => key,
//         },
//     );
//     let ret: U256 = TestInstance::instance(test).result();
// }

// // #[test]
// fn test_set_referral_shares_to_end() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     let key: U256 = 1.into();
//     let value: U256 = 100.into();
//     test.call_contract(
//         Sender(owner),
//         "set_referral_shares_to_end",
//         runtime_args! {
//             "key" => key,
//             "value" => value
//         },
//     );
// }

// // #[test]
// fn test_get_scheduled_to_end() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     let key: U256 = 1.into();
//     test.call_contract(
//         Sender(owner),
//         "get_scheduled_to_end",
//         runtime_args! {
//             "key" => key,
//         },
//     );
//     let ret: U256 = TestInstance::instance(test).result();
// }

// // #[test]
// fn test_set_scheduled_to_end() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     let key: U256 = 1.into();
//     let value: U256 = 100.into();
//     test.call_contract(
//         Sender(owner),
//         "set_scheduled_to_end",
//         runtime_args! {
//             "key" => key,
//             "value" => value
//         },
//     );
// }

// // #[test]
// fn test_get_total_penalties() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     let key: U256 = 1.into();
//     test.call_contract(
//         Sender(owner),
//         "get_total_penalties",
//         runtime_args! {
//             "key" => key,
//         },
//     );
//     let ret: U256 = TestInstance::instance(test).result();
// }

// // #[test]
// fn test_set_total_penalties() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     let key: U256 = 1.into();
//     let value: U256 = 100.into();
//     test.call_contract(
//         Sender(owner),
//         "set_total_penalties",
//         runtime_args! {
//             "key" => key,
//             "value" => value
//         },
//     );
// }

// // #[test]
// fn test_get_declaration_constants() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     test.call_contract(Sender(owner), "get_declaration_constants", runtime_args! {});
//     let ret: Bytes = TestInstance::instance(test).result();
//     let ret: Vec<u8> = Vec::from(ret);
// }

// // #[test]
// fn test_set_inflation_rate() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     let value: U256 = 100.into();
//     test.call_contract(
//         Sender(owner),
//         "set_inflation_rate",
//         runtime_args! {
//             "value" => value
//         },
//     );
// }

// // #[test]
// fn test_get_inflation_rate() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     test.call_contract(Sender(owner), "get_inflation_rate", runtime_args! {});
//     let ret: U256 = TestInstance::instance(test).result();
// }

// // #[test]
// fn test_get_liquidity_rate() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     test.call_contract(Sender(owner), "get_liquidity_rate", runtime_args! {});
//     let ret: U256 = TestInstance::instance(test).result();
// }

// // #[test]
// fn test_set_liquidity_rate() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     let value: U256 = 100.into();
//     test.call_contract(
//         Sender(owner),
//         "set_liquidity_rate",
//         runtime_args! {
//             "value" => value
//         },
//     );
// }

// // #[test]
// fn test_get_liquidity_guard_status() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     test.call_contract(
//         Sender(owner),
//         "get_liquidity_guard_status",
//         runtime_args! {},
//     );
//     let ret: bool = TestInstance::instance(test).result();
// }

// // #[test]
// fn test_set_liquidity_guard_status() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     let value: bool = false;
//     test.call_contract(
//         Sender(owner),
//         "set_liquidity_guard_status",
//         runtime_args! {
//             "value" => value
//         },
//     );
// }

// // #[test]
// fn test_get_scspr() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     test.call_contract(Sender(owner), "get_scspr", runtime_args! {});
//     let ret: Key = TestInstance::instance(test).result();
// }

// // #[test]
// fn test_set_scspr() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     let scspr: Key = Key::Account(owner);
//     test.call_contract(
//         Sender(owner),
//         "set_scspr",
//         runtime_args! {
//             "scspr" => scspr
//         },
//     );
// }

// // #[test]
// fn test_get_wcspr() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     test.call_contract(Sender(owner), "get_scspr", runtime_args! {});
//     let ret: Key = TestInstance::instance(test).result();
// }

// // #[test]
// fn test_get_stable_usd_equivalent() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     let stable_usd_equivalent: Key = Key::Account(owner);
//     test.call_contract(
//         Sender(owner),
//         "set_stable_usd_equivalent",
//         runtime_args! {
//             "stable_usd_equivalent" => stable_usd_equivalent
//         },
//     );
//     test.call_contract(Sender(owner), "get_stable_usd_equivalent", runtime_args! {});
//     let ret: Key = TestInstance::instance(test).result();
// }

// // #[test]
// fn test_set_stable_usd_equivalent() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     let stable_usd_equivalent: Key = Key::Account(owner);
//     test.call_contract(
//         Sender(owner),
//         "set_stable_usd_equivalent",
//         runtime_args! {
//             "stable_usd_equivalent" => stable_usd_equivalent
//         },
//     );
// }

// // #[test]
// // fn test_create_pair() {
// //     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
// //     test.call_contract(Sender(owner), "create_pair", runtime_args! {});
// // }

// // #[test]
// // fn test_get_unsiwap_pair() {
// //     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
// //     test.call_contract(Sender(owner), "get_unsiwap_pair", runtime_args! {});
// //     let ret: Key = TestInstance::instance(test).result();
// // }

// // #[test]
// fn test_get_lt_balance() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     test.call_contract(Sender(owner), "get_lt_balance", runtime_args! {});
//     let ret: U256 = TestInstance::instance(test).result();
// }

// // #[test]
// fn test_set_lt_balance() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     let value: U256 = 100.into();
//     test.call_contract(
//         Sender(owner),
//         "set_lt_balance",
//         runtime_args! {
//             "value" => value
//         },
//     );
// }

// // #[test]
// fn test_get_launchtime() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     test.call_contract(Sender(owner), "get_launchtime", runtime_args! {});
//     let ret: U256 = TestInstance::instance(test).result();
// }

// // #[test]
// fn test_set_launchtime() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     let value: U256 = 100.into();
//     test.call_contract(
//         Sender(owner),
//         "set_launchtime",
//         runtime_args! {
//             "value" => value
//         },
//     );
// }

// // #[test]
// fn test_get_scrapes() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     let key: String = "key".into();
//     test.call_contract(
//         Sender(owner),
//         "get_scrapes",
//         runtime_args! {
//             "key" => key
//         },
//     );
//     let ret: U256 = TestInstance::instance(test).result();
// }

// // #[test]
// fn test_set_scrapes() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     let key: String = "key".into();
//     let value: U256 = 100.into();
//     test.call_contract(
//         Sender(owner),
//         "set_scrapes",
//         runtime_args! {
//             "key" => key,
//             "value" => value
//         },
//     );
// }

/////////////
// GLOBALS //
/////////////

#[test]
fn test_increase_globals() {
    let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
    let _staked: U256 = 10.into();
    let _shares: U256 = 20.into();
    let _rshares: U256 = 30.into();
    test.call_contract(
        Sender(owner),
        "increase_globals",
        runtime_args! {
            "_staked" => _staked,
            "_shares" => _shares,
            "_rshares" => _rshares
        },
    );
    let total_staked: U256 = test
        .query_dictionary(GLOBALS_GLOBALS_STRUCT, GLOBALS_TOTAL_STAKED.to_string())
        .unwrap();
    let total_shares: U256 = test
        .query_dictionary(GLOBALS_GLOBALS_STRUCT, GLOBALS_TOTAL_SHARES.to_string())
        .unwrap();
    let referral_shares: U256 = test
        .query_dictionary(GLOBALS_GLOBALS_STRUCT, GLOBALS_REFERRAL_SHARES.to_string())
        .unwrap();

    assert_eq!(total_shares, _shares);
    assert_eq!(total_staked, _staked);
    assert_eq!(referral_shares, _rshares);
}
#[test]
fn test_decrease_globals() {
    let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
    let _staked: U256 = 10.into();
    let _shares: U256 = 20.into();
    let _rshares: U256 = 30.into();
    test.call_contract(
        Sender(owner),
        "decrease_globals",
        runtime_args! {
            "_staked" => _staked,
            "_shares" => _shares,
            "_rshares" => _rshares
        },
    );
    let total_staked: U256 = test
        .query_dictionary(GLOBALS_GLOBALS_STRUCT, GLOBALS_TOTAL_STAKED.to_string())
        .unwrap();
    let total_shares: U256 = test
        .query_dictionary(GLOBALS_GLOBALS_STRUCT, GLOBALS_TOTAL_SHARES.to_string())
        .unwrap();
    let referral_shares: U256 = test
        .query_dictionary(GLOBALS_GLOBALS_STRUCT, GLOBALS_REFERRAL_SHARES.to_string())
        .unwrap();

    assert_eq!(total_shares, U256::from(0));
    assert_eq!(total_staked, U256::from(0));
    assert_eq!(referral_shares, U256::from(0));
}

#[test]
fn test_set_globals() {
    let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
    let field: String = GLOBALS_CURRENT_WISE_DAY.into();
    let value: U256 = 100.into();
    test.call_contract(
        Sender(owner),
        "set_globals",
        runtime_args! {
            "field" => field.clone(),
            "value" => value
        },
    );
    let current_wise_way: U256 = test
        .query_dictionary(GLOBALS_GLOBALS_STRUCT, field)
        .unwrap();
    assert_eq!(current_wise_way, value);
}

#[test]
fn test_get_globals() {
    let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
    let field: String = GLOBALS_SHARE_PRICE.into();

    // number is set in deployment of globals
    let mut number: U256 = U256::from(10).pow(15.into()); // 10 ^ 15
    number = U256::from(100) * number; // 100 * (10 ^ 15)  =  100E15;

    test.call_contract(
        Sender(owner),
        "get_globals",
        runtime_args! {
            "field" => field
        },
    );
    let ret: U256 = TestInstance::instance(test).result();
    assert_eq!(ret, number);
}

////////////
// HELPER //
////////////

#[test]
fn test_get_lock_days() {
    let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
    let _stake: Stake = Stake {
        stakes_shares: U256::from(10),
        staked_amount: U256::from(10),
        reward_amount: U256::from(10),
        start_day: 45,
        lock_days: 45,
        final_day: 55,
        close_day: 45,
        scrape_day: U256::from(10),
        dai_equivalent: U256::from(10),
        referrer_shares: U256::from(10),
        referrer: Key::from(owner),
        is_active: false,
    };
    let stake: Vec<u8> = _stake.clone().into_bytes().unwrap();

    test.call_contract(
        Sender(owner),
        "get_lock_days",
        runtime_args! {
            "stake" => Bytes::from(stake)
        },
    );
    let ret: U256 = TestInstance::instance(test).result();
    assert_eq!(ret, (_stake.lock_days - 1).into());
}

#[test]
fn test_generate_liquidity_stake_id() {
    let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
    let staker: Key = Key::Account(owner);
    test.call_contract(
        Sender(owner),
        "generate_liquidity_stake_id",
        runtime_args! {
            "staker" => staker
        },
    );
    let ret: Vec<u32> = TestInstance::instance(test).result();
    let empty_vec: Vec<u32> = Vec::new();
    assert_ne!(ret, empty_vec);
}

#[test]
fn test_increase_liquidity_stake_count() {
    let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
    let staker: Key = Key::Account(owner);

    test.call_contract(
        Sender(owner),
        "set_liquidity_stake_count",
        runtime_args! {
            "staker"=> staker,
            "value"=> U256::from(0)
        },
    );

    test.call_contract(
        Sender(owner),
        "increase_liquidity_stake_count",
        runtime_args! {
            "staker" => staker
        },
    );

    test.call_contract(
        Sender(owner),
        "get_liquidity_stake_count",
        runtime_args! {
            "staker"=>staker
        },
    );

    let count: U256 = test.query_named_key("result".to_string());
    assert_eq!(count, 1.into());
}

#[test]
fn test_stake_not_started() {
    let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
    let _stake: Stake = Stake {
        stakes_shares: U256::from(10),
        staked_amount: U256::from(10),
        reward_amount: U256::from(10),
        start_day: 45,
        lock_days: 45,
        final_day: 55,
        close_day: 45,
        scrape_day: U256::from(10),
        dai_equivalent: U256::from(10),
        referrer_shares: U256::from(10),
        referrer: Key::from(owner),
        is_active: false,
    };
    let stake: Vec<u8> = _stake.into_bytes().unwrap();

    test.call_contract(
        Sender(owner),
        "stake_not_started",
        runtime_args! {
            "stake" => Bytes::from(stake)
        },
    );
    let ret: bool = TestInstance::instance(test).result();
    assert_eq!(ret, false);
}

// #[test]
// fn test_transfer_from() {
//     let (env, _owner, test) = deploy();
//     let token: Key = Key::Hash(deploy_erc20(&env, _owner).contract_hash());
//     let recipient: Key = Key::Account(env.next_user());
//     let owner: Key = Key::Account(_owner);
//     let amount: U256 = 10.into();
//     test.call_contract(
//         Sender(_owner),
//         "approve",
//         runtime_args! {
//             "token" => token,
//             "spender" => recipient,
//             "amount" => amount
//         },
//     );
//     test.call_contract(
//         Sender(_owner),
//         "transfer_from",
//         runtime_args! {
//             "token" => token,
//             "recipient" => recipient,
//             "owner" => owner,
//             "amount" => amount
//         },
//     );
//     let ret: Result<(), u32> = TestInstance::instance(test).result();
// }
#[test]
fn test_stake_ended() {
    let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
    let _stake: Stake = Stake {
        stakes_shares: U256::from(10),
        staked_amount: U256::from(10),
        reward_amount: U256::from(10),
        start_day: 45,
        lock_days: 45,
        final_day: 55,
        close_day: 45,
        scrape_day: U256::from(10),
        dai_equivalent: U256::from(10),
        referrer_shares: U256::from(10),
        referrer: Key::from(owner),
        is_active: false,
    };
    let stake: Vec<u8> = _stake.into_bytes().unwrap();

    test.call_contract(
        Sender(owner),
        "stake_ended",
        runtime_args! {
            "stake" => Bytes::from(stake)
        },
    );
    let ret: bool = TestInstance::instance(test).result();
    assert_eq!(ret, true);
}

#[test]
fn test_days_diff() {
    let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
    let start_date: U256 = 10.into();
    let end_date: U256 = 20.into();
    test.call_contract(
        Sender(owner),
        "days_diff",
        runtime_args! {
            "start_date" => start_date,
            "end_date" => end_date
        },
    );
    let ret: U256 = TestInstance::instance(test).result();
    assert_eq!(ret, end_date - start_date);
}

#[test]
fn test_is_mature_stake() {
    let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
    let _stake: Stake = Stake {
        stakes_shares: U256::from(10),
        staked_amount: U256::from(10),
        reward_amount: U256::from(10),
        start_day: 45,
        lock_days: 45,
        final_day: 55,
        close_day: 45,
        scrape_day: U256::from(10),
        dai_equivalent: U256::from(10),
        referrer_shares: U256::from(10),
        referrer: Key::from(owner),
        is_active: false,
    };
    let stake: Vec<u8> = _stake.into_bytes().unwrap();

    test.call_contract(
        Sender(owner),
        "is_mature_stake",
        runtime_args! {
            "stake" => Bytes::from(stake)
        },
    );
    let ret: bool = TestInstance::instance(test).result();
    assert_eq!(ret, false);
}

#[test]
fn test_days_left() {
    let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
    let _stake: Stake = Stake {
        stakes_shares: U256::from(10),
        staked_amount: U256::from(10),
        reward_amount: U256::from(10),
        start_day: 45,
        lock_days: 45,
        final_day: 55,
        close_day: 45,
        scrape_day: U256::from(10),
        dai_equivalent: U256::from(10),
        referrer_shares: U256::from(10),
        referrer: Key::from(owner),
        is_active: false,
    };
    let stake: Vec<u8> = _stake.clone().into_bytes().unwrap();

    test.call_contract(
        Sender(owner),
        "days_left",
        runtime_args! {
            "stake" => Bytes::from(stake)
        },
    );
    let ret: U256 = TestInstance::instance(test).result();

    assert_eq!(ret, (_stake.final_day - _stake.close_day).into());
}

#[test]
fn test_not_critical_mass_referrer() {
    let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
    let crtical_mass_struct: CriticalMass = CriticalMass {
        total_amount: U256::from(5),
        activation_day: U256::from(5),
    };
    let critical_mass_bytes: Vec<u8> = crtical_mass_struct.clone().into_bytes().unwrap();
    let referrer: Key = Key::from(owner);
    test.call_contract(
        Sender(owner),
        "set_struct_from_key",
        runtime_args! {
            "struct_name"=>DECLARATION_CRITICAL_MASS_DICT.to_string(),
            "value"=>Bytes::from(critical_mass_bytes),
            "key"=>referrer.to_formatted_string()
        },
    );

    test.call_contract(
        Sender(owner),
        "not_critical_mass_referrer",
        runtime_args! {
            "referrer" => referrer
        },
    );
    let ret: bool = TestInstance::instance(test).result();
    assert_eq!(ret, false);
}

#[test]
fn test_calculation_day() {
    let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
    let _stake: Stake = Stake {
        stakes_shares: U256::from(10),
        staked_amount: U256::from(10),
        reward_amount: U256::from(10),
        start_day: 45,
        lock_days: 45,
        final_day: 55,
        close_day: 45,
        scrape_day: U256::from(10),
        dai_equivalent: U256::from(10),
        referrer_shares: U256::from(10),
        referrer: Key::from(owner),
        is_active: false,
    };
    let stake: Vec<u8> = _stake.into_bytes().unwrap();

    test.call_contract(
        Sender(owner),
        "calculation_day",
        runtime_args! {
            "stake" => Bytes::from(stake)
        },
    );
    let ret: U256 = TestInstance::instance(test).result();
}

// // #[test]
// // fn test_not_past() {
// //     let (
//         env,
//         owner,
//         test,
//         uniswap_factory,
//         wcspr,
//         flash_swapper,
//         uniswap_pair,
//         uniswap_library,
//         uniswap_router,
//         liquidity_guard,
//         wcspr,
//         erc20,
//         scspr,
//     ) = deploy();
// //     let day: U256 = 10.into();
// //     test.call_contract(
// //         Sender(owner),
// //         "not_past",
// //         runtime_args! {
// //             "day" => day
// //         },
// //     );
// //     let ret: bool = TestInstance::instance(test).result();
// // }

// // #[test]
// // fn test_not_future() {
// //     let (
//         env,
//         owner,
//         test,
//         uniswap_factory,
//         wcspr,
//         flash_swapper,
//         uniswap_pair,
//         uniswap_library,
//         uniswap_router,
//         liquidity_guard,
//         wcspr,
//         erc20,
//         scspr,
//     ) = deploy();
// //     let day: U256 = 10.into();
// //     test.call_contract(
// //         Sender(owner),
// //         "not_future",
// //         runtime_args! {
// //             "day" => day
// //         },
// //     );
// //     let ret: bool = TestInstance::instance(test).result();
// // }

// // #[test]
// fn test_stakes_pagination() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     let staker: Key = Key::Account(owner);
//     let offset: U256 = 10.into();
//     let length: U256 = 20.into();
//     test.call_contract(
//         Sender(owner),
//         "stakes_pagination",
//         runtime_args! {
//             "staker" => staker,
//             "offset" => offset,
//             "length" => length
//         },
//     );
//     let ret: Vec<Vec<u32>> = TestInstance::instance(test).result();
// }

// // #[test]
// fn test_referrals_pagination() {
//     let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
//     let referrer: Key = Key::Account(owner);
//     let offset: U256 = 10.into();
//     let length: U256 = 20.into();
//     test.call_contract(
//         Sender(owner),
//         "referrals_pagination",
//         runtime_args! {
//             "referrer" => referrer,
//             "offset" => offset,
//             "length" => length
//         },
//     );
//     let ret: Vec<Vec<u32>> = TestInstance::instance(test).result();
// }

#[test]
fn test_starting_day() {
    let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
    let _stake: Stake = Stake {
        stakes_shares: U256::from(10),
        staked_amount: U256::from(10),
        reward_amount: U256::from(10),
        start_day: 45,
        lock_days: 45,
        final_day: 55,
        close_day: 45,
        scrape_day: U256::from(10),
        dai_equivalent: U256::from(10),
        referrer_shares: U256::from(10),
        referrer: Key::from(owner),
        is_active: false,
    };
    let stake: Vec<u8> = _stake.clone().into_bytes().unwrap();

    test.call_contract(
        Sender(owner),
        "starting_day",
        runtime_args! {
            "stake" => Bytes::from(stake)
        },
    );
    let ret: U256 = TestInstance::instance(test).result();
    assert_eq!(ret, _stake.scrape_day)
}

/////////////////////
// LIQUIDITY_TOKEN //
/////////////////////

#[test]
fn test_create_liquidity_stake() {
    let (
        env,
        owner,
        test,
        uniswap_factory,
        wcspr,
        flash_swapper,
        uniswap_pair,
        uniswap_library,
        uniswap_router,
        liquidity_guard,
        erc20,
        scspr,
    ) = deploy();
    let liquidity_tokens: U256 = 100.into();
    let update_day: u64 = 3;
    let globals_current_wise_day: U256 = 1.into();
    // setup for liquidity guard trigger
    // initialize liquidity guard status
    test.call_contract(
        Sender(owner),
        "set_liquidity_guard_status",
        runtime_args! {
            "value"=>false
        },
    );
    let status: bool = test.query_named_key(DECLARATION_LIQUIDITY_GUARD_STATUS.to_string());
    assert_eq!(status, false);
    // initialize pair
    let token0 = deploy_wcspr(&env, owner);
    let token1 = deploy_wcspr(&env, owner);
    let token0 = Key::Hash(token0.contract_hash());
    let token1 = Key::Hash(token1.contract_hash());
    let factory_hash = Key::Hash(uniswap_factory.contract_hash());
    uniswap_pair.call_contract(
        Sender(owner),
        "initialize",
        runtime_args! {
            "token0"=>token0,
            "token1"=>token1,
            "factory_hash"=> factory_hash
        },
    );

    // setup for snapshot trigger
    // init globals
    test.call_contract(
        Sender(owner),
        "set_globals",
        runtime_args! {
            "field"=>GLOBALS_CURRENT_WISE_DAY.to_string(),
            "value"=>globals_current_wise_day.clone()
        },
    );
    test.call_contract(
        Sender(owner),
        "set_globals",
        runtime_args! {
            "field"=>GLOBALS_TOTAL_STAKED.to_string(),
            "value"=>U256::from(608351533)
        },
    );
    // setup liquidity rate
    test.call_contract(
        Sender(owner),
        "set_liquidity_rate",
        runtime_args! {
            "value"=>U256::from(100000) // amount  = 60835153328
        },
    );

    // mint to owner in pair
    uniswap_pair.call_contract(
        Sender(owner),
        "erc20_mint",
        runtime_args! {
            "to"=> Key::from(owner),
            "amount"=> liquidity_tokens
        },
    );

    // owner approves test contract for transfer_from
    let package_hash: ContractPackageHash = test.query_named_key(SELF_PACKAGE_HASH.to_string());
    let package_hash: Key = Key::from(package_hash);
    uniswap_pair.call_contract(
        Sender(owner),
        "approve",
        runtime_args! {
            "spender"=> package_hash.clone(),
            "amount"=> liquidity_tokens.clone()
        },
    );

    // create liquidity stake
    test.call_contract(
        Sender(owner),
        "create_liquidity_stake",
        runtime_args! {
            "liquidity_tokens" => liquidity_tokens.clone()
        },
    );
    // read the liquidity stake id generated from return value
    let liquidity_stake_id: Vec<u32> = test.query_named_key("result".to_string());
    //TestInstance::instance(test).result();

    // get the liquidity stake struct
    test.call_contract(
        Sender(owner),
        "get_liquidity_stake",
        runtime_args! {
            "staker"=>Key::from(owner),
            "id"=>liquidity_stake_id.clone()
        },
    );

    let liquidity_stake_bytes: Bytes = TestInstance::instance(test).result();
    let liquidity_stake_bytes: Vec<u8> = Vec::from(liquidity_stake_bytes);
    let liquidity_stake_struct: LiquidityStake = LiquidityStake::from_bytes(&liquidity_stake_bytes)
        .unwrap()
        .0;
    // verify staked amount
    assert_eq!(liquidity_stake_struct.staked_amount, liquidity_tokens);
}

#[test]
fn test_end_liquidity_stake() {
    let (
        env,
        owner,
        test,
        uniswap_factory,
        wcspr,
        flash_swapper,
        uniswap_pair,
        uniswap_library,
        uniswap_router,
        liquidity_guard,
        erc20,
        scspr,
    ) = deploy();
    let liquidity_tokens: U256 = 100.into();
    let update_day: u64 = 3;
    let globals_current_wise_day: U256 = 1.into();
    // setup snapshot trigger
    test.call_contract(
        Sender(owner),
        "set_liquidity_guard_status",
        runtime_args! {
            "value"=>false
        },
    );
    let status: bool = test.query_named_key(DECLARATION_LIQUIDITY_GUARD_STATUS.to_string());
    assert_eq!(status, false);
    // initialize pair
    let token0 = deploy_wcspr(&env, owner);
    let token1 = deploy_wcspr(&env, owner);
    let token0 = Key::Hash(token0.contract_hash());
    let token1 = Key::Hash(token1.contract_hash());
    let factory_hash = Key::Hash(uniswap_factory.contract_hash());
    uniswap_pair.call_contract(
        Sender(owner),
        "initialize",
        runtime_args! {
            "token0"=>token0,
            "token1"=>token1,
            "factory_hash"=> factory_hash
        },
    );

    // setup for snapshot trigger
    // init globals
    test.call_contract(
        Sender(owner),
        "set_globals",
        runtime_args! {
            "field"=>GLOBALS_CURRENT_WISE_DAY.to_string(),
            "value"=>globals_current_wise_day.clone()
        },
    );
    test.call_contract(
        Sender(owner),
        "set_globals",
        runtime_args! {
            "field"=>GLOBALS_TOTAL_STAKED.to_string(),
            "value"=>U256::from(608351533)
        },
    );
    // setup liquidity rate
    test.call_contract(
        Sender(owner),
        "set_liquidity_rate",
        runtime_args! {
            "value"=>U256::from(100000) // amount  = 60835153328
        },
    );

    // mint to owner in pair
    uniswap_pair.call_contract(
        Sender(owner),
        "erc20_mint",
        runtime_args! {
            "to"=> Key::from(owner),
            "amount"=> liquidity_tokens
        },
    );

    // owner approves test contract for transfer_from
    let package_hash: ContractPackageHash = test.query_named_key(SELF_PACKAGE_HASH.to_string());
    let package_hash: Key = Key::from(package_hash);
    uniswap_pair.call_contract(
        Sender(owner),
        "approve",
        runtime_args! {
            "spender"=> package_hash.clone(),
            "amount"=> liquidity_tokens.clone()
        },
    );

    // create liquidity stake
    test.call_contract(
        Sender(owner),
        "create_liquidity_stake",
        runtime_args! {
            "liquidity_tokens" => liquidity_tokens.clone()
        },
    );
    // read the liquidity stake id generated from return value
    let liquidity_stake_id: Vec<u32> = test.query_named_key("result".to_string());
    //TestInstance::instance(test).result();

    // get the liquidity stake struct
    test.call_contract(
        Sender(owner),
        "get_liquidity_stake",
        runtime_args! {
            "staker"=>Key::from(owner),
            "id"=>liquidity_stake_id.clone()
        },
    );

    let liquidity_stake_bytes: Bytes = test.query_named_key("result".to_string());
    let liquidity_stake_bytes: Vec<u8> = Vec::from(liquidity_stake_bytes);
    let liquidity_stake_struct: LiquidityStake = LiquidityStake::from_bytes(&liquidity_stake_bytes)
        .unwrap()
        .0;
    // verify staked amount and status
    assert_eq!(liquidity_stake_struct.staked_amount, liquidity_tokens);
    assert_eq!(liquidity_stake_struct.is_active, true);

    // query balance of staker from erc20,
    // let staker_balance: U256 = erc20
    //     .query_dictionary(BALANCES_DICT, Key::from(owner).to_string())
    //     .unwrap();
    // assert_eq!(staker_balance, 0.into());

    // setup snapshots dict
    let mut lsnapshot = LSnapShot::new();
    lsnapshot.inflation_amount = 1.into();
    let lsnapshot = lsnapshot.clone().into_bytes().unwrap();
    for i in 0..100 {
        test.call_contract(
            Sender(owner),
            "snapshot_set_struct_from_key",
            runtime_args! {
                "key"=>U256::from(i),
                "struct_name"=>SNAPSHOT_LSNAPSHOTS_DICT.to_string(),
                "value"=>Bytes::from(lsnapshot.clone())
            },
        );
    }
    // // init GLOBALS_CURRENT_WISE_DAY
    // test.call_contract(
    //     Sender(owner),
    //     "set_globals",
    //     runtime_args! {
    //         "field"=>GLOBALS_CURRENT_WISE_DAY,
    //         "value"=> U256::from(0)
    //     },
    // );

    // now we end liquidity stake
    test.call_contract(
        Sender(owner),
        "end_liquidity_stake",
        runtime_args! {
            "id"=>liquidity_stake_id.clone()
        },
    );

    // now get liquidity stake and check it's status
    test.call_contract(
        Sender(owner),
        "get_liquidity_stake",
        runtime_args! {
            "staker"=>Key::from(owner),
            "id"=>liquidity_stake_id.clone()
        },
    );

    let liquidity_stake_bytes: Bytes = TestInstance::instance(test).result();
    let liquidity_stake_bytes: Vec<u8> = Vec::from(liquidity_stake_bytes);
    let liquidity_stake_struct: LiquidityStake = LiquidityStake::from_bytes(&liquidity_stake_bytes)
        .unwrap()
        .0;
    // verify staked amount and status
    assert_eq!(liquidity_stake_struct.is_active, false);
}

#[test]
fn test_check_liquidity_stake_by_id() {
    let (
        env,
        owner,
        test,
        uniswap_factory,
        wcspr,
        flash_swapper,
        uniswap_pair,
        uniswap_library,
        uniswap_router,
        liquidity_guard,
        erc20,
        scspr,
    ) = deploy();
    let liquidity_tokens: U256 = 100.into();
    let update_day: u64 = 3;
    let globals_current_wise_day: U256 = 1.into();
    // setup snapshot trigger
    test.call_contract(
        Sender(owner),
        "set_liquidity_guard_status",
        runtime_args! {
            "value"=>false
        },
    );
    let status: bool = test.query_named_key(DECLARATION_LIQUIDITY_GUARD_STATUS.to_string());
    assert_eq!(status, false);
    // initialize pair
    let token0 = deploy_wcspr(&env, owner);
    let token1 = deploy_wcspr(&env, owner);
    let token0 = Key::Hash(token0.contract_hash());
    let token1 = Key::Hash(token1.contract_hash());
    let factory_hash = Key::Hash(uniswap_factory.contract_hash());
    uniswap_pair.call_contract(
        Sender(owner),
        "initialize",
        runtime_args! {
            "token0"=>token0,
            "token1"=>token1,
            "factory_hash"=> factory_hash
        },
    );

    // setup for snapshot trigger
    // init globals
    test.call_contract(
        Sender(owner),
        "set_globals",
        runtime_args! {
            "field"=>GLOBALS_CURRENT_WISE_DAY.to_string(),
            "value"=>globals_current_wise_day.clone()
        },
    );
    test.call_contract(
        Sender(owner),
        "set_globals",
        runtime_args! {
            "field"=>GLOBALS_TOTAL_STAKED.to_string(),
            "value"=>U256::from(608351533)
        },
    );
    // setup liquidity rate
    test.call_contract(
        Sender(owner),
        "set_liquidity_rate",
        runtime_args! {
            "value"=>U256::from(100000) // amount  = 60835153328
        },
    );

    // mint to owner in pair
    uniswap_pair.call_contract(
        Sender(owner),
        "erc20_mint",
        runtime_args! {
            "to"=> Key::from(owner),
            "amount"=> liquidity_tokens
        },
    );

    // owner approves test contract for transfer_from
    let package_hash: ContractPackageHash = test.query_named_key(SELF_PACKAGE_HASH.to_string());
    let package_hash: Key = Key::from(package_hash);
    uniswap_pair.call_contract(
        Sender(owner),
        "approve",
        runtime_args! {
            "spender"=> package_hash.clone(),
            "amount"=> liquidity_tokens.clone()
        },
    );

    // create liquidity stake
    test.call_contract(
        Sender(owner),
        "create_liquidity_stake",
        runtime_args! {
            "liquidity_tokens" => liquidity_tokens.clone()
        },
    );
    // read the liquidity stake id generated from return value
    let liquidity_stake_id: Vec<u32> = test.query_named_key("result".to_string());
    //TestInstance::instance(test).result();

    // now that stake is generated, check it
    // check liquidity stake
    test.call_contract(
        Sender(owner),
        "check_liquidity_stake_by_id",
        runtime_args! {
            "staker"=>Key::from(owner),
            "id"=>liquidity_stake_id.clone()
        },
    );

    let liquidity_stake_bytes: Bytes = TestInstance::instance(test).result();
    let liquidity_stake_bytes: Vec<u8> = Vec::from(liquidity_stake_bytes);
    let liquidity_stake_struct: LiquidityStake = LiquidityStake::from_bytes(&liquidity_stake_bytes)
        .unwrap()
        .0;
    // verify staked amount
    assert_eq!(liquidity_stake_struct.staked_amount, liquidity_tokens);
}

////////////////////
// REFERRAL_TOKEN //
////////////////////

#[test]
fn test_add_referrer_shares_to_end() {
    let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
    let final_day: U256 = 10.into();
    let shares: U256 = 20.into();

    // init referrer shares dict with 'shares'a mount
    test.call_contract(
        Sender(owner),
        "set_referral_shares_to_end",
        runtime_args! {
            "key"=> final_day.clone(),
            "value"=> shares.clone()
        },
    );

    // this will double shares
    test.call_contract(
        Sender(owner),
        "add_referrer_shares_to_end",
        runtime_args! {
            "final_day" => final_day,
            "shares" => shares
        },
    );

    let ret: U256 = test
        .query_dictionary(
            DECLARATION_REFERRAL_SHARES_TO_END_DICT,
            final_day.to_string(),
        )
        .unwrap();
    assert_eq!(ret, shares * 2);
}

#[test]
fn test_remove_referrer_shares_to_end() {
    let (
        env,
        owner,
        test,
        uniswap_factory,
        wcspr,
        flash_swapper,
        uniswap_pair,
        uniswap_library,
        uniswap_router,
        liquidity_guard,
        erc20,
        scspr,
    ) = deploy();
    let final_day: U256 = 10.into();
    let shares: U256 = 20.into();

    // case 1 - is_final_day
    // init referrer shares dict with 'shares'a mount
    test.call_contract(
        Sender(owner),
        "set_referral_shares_to_end",
        runtime_args! {
            "key"=> final_day.clone(),
            "value"=> shares.clone() * 2
        },
    );

    test.call_contract(
        Sender(owner),
        "remove_referrer_shares_to_end",
        runtime_args! {
            "final_day" => final_day,
            "shares" => shares
        },
    );

    let ret: U256 = test
        .query_dictionary(
            DECLARATION_REFERRAL_SHARES_TO_END_DICT,
            final_day.to_string(),
        )
        .unwrap();
    assert_eq!(ret, shares);

    // case 2 - not final day
    let final_day: U256 = 1.into();
    let shares: U256 = 20.into();

    // init referrer shares dict with 'shares' amount
    test.call_contract(
        Sender(owner),
        "set_referral_shares_to_end",
        runtime_args! {
            "key"=> final_day.clone(),
            "value"=> shares.clone() * 2
        },
    );

    // setup a snapshot struct
    let snapshot: RSnapshot = RSnapshot {
        total_shares: U256::from(100),
        inflation_amount: U256::from(100),
        scheduled_to_end: U256::from(100),
    };
    let previous_wise_day: U256 = 4.into();
    let rsnapshot: Vec<u8> = snapshot.clone().into_bytes().unwrap();
    test.call_contract(
        Sender(owner),
        "snapshot_set_struct_from_key",
        runtime_args! {
            "key"=>previous_wise_day, //previous wise day is 4, current in 5
            "struct_name"=> SNAPSHOT_RSNAPSHOTS_DICT,
            "value"=>Bytes::from(rsnapshot)
        },
    );

    test.call_contract(
        Sender(owner),
        "remove_referrer_shares_to_end",
        runtime_args! {
            "final_day" => final_day,
            "shares" => shares
        },
    );

    let rsnapshot: Bytes = test
        .query_dictionary(SNAPSHOT_RSNAPSHOTS_DICT, previous_wise_day.to_string())
        .unwrap();
    let rsnapshot: Vec<u8> = Vec::from(rsnapshot);
    let rsnapshot: Snapshot = Snapshot::from_bytes(&rsnapshot).unwrap().0;
    assert_eq!(
        rsnapshot.scheduled_to_end,
        snapshot.scheduled_to_end - shares
    );
}

#[test]
fn test_add_critical_mass() {
    let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
    let referrer: Key = Key::Account(owner);
    let dai_equivalent: U256 = 50.into();
    let total_amount: U256 = 1000.into();
    let current_wise_day: U256 = 5.into();

    let critical_mass: CriticalMass = CriticalMass {
        total_amount,
        activation_day: U256::from(10),
    };
    let value: Vec<u8> = critical_mass.clone().into_bytes().unwrap();
    test.call_contract(
        Sender(owner),
        "set_struct_from_key",
        runtime_args! {
            "key" => referrer.to_formatted_string(),
            "value" => Bytes::from(value),
            "struct_name" => DECLARATION_CRITICAL_MASS_DICT.to_string()
        },
    );
    test.call_contract(
        Sender(owner),
        "add_critical_mass",
        runtime_args! {
            "referrer" => referrer,
            "dai_equivalent" => dai_equivalent
        },
    );

    // check updated total_amount on critical mass struct
    test.call_contract(
        Sender(owner),
        "get_struct_from_key",
        runtime_args! {
            "key"=>referrer.to_formatted_string(),
            "struct_name"=>DECLARATION_CRITICAL_MASS_DICT.to_string(),
        },
    );
    let ret: Bytes = test.query_named_key("result".to_string());
    // let ret: Bytes = test
    //     .query_dictionary(
    //         DECLARATION_CRITICAL_MASS_DICT,
    //         referrer.to_string(),
    //     )
    //     .unwrap();
    let ret: Vec<u8> = Vec::from(ret);
    let ret: CriticalMass = CriticalMass::from_bytes(&ret).unwrap().0;
    assert_eq!(ret.total_amount, total_amount + dai_equivalent);
    assert_eq!(ret.activation_day, critical_mass.activation_day);
}

#[test]
fn test_remove_critical_mass() {
    let (
        env,
        owner,
        test,
        uniswap_factory,
        wcspr,
        flash_swapper,
        uniswap_pair,
        uniswap_library,
        uniswap_router,
        liquidity_guard,
        erc20,
        scspr,
    ) = deploy();
    let referrer: Key = Key::Account(owner);
    let dai_equivalent: U256 = 50.into();
    let total_amount: U256 = 1000.into();
    let activation_day: U256 = 2.into();
    let current_wise_day: U256 = 5.into();
    let start_day: U256 = 30.into(); // must be more than current wise day

    // init critical mass dict
    let critical_mass: CriticalMass = CriticalMass {
        total_amount,
        activation_day,
    };
    // set struct to dict
    let value: Vec<u8> = critical_mass.clone().into_bytes().unwrap();
    test.call_contract(
        Sender(owner),
        "set_struct_from_key",
        runtime_args! {
            "key" => referrer.to_formatted_string(),
            "value" => Bytes::from(value),
            "struct_name" => DECLARATION_CRITICAL_MASS_DICT.to_string()
        },
    );
    //
    test.call_contract(
        Sender(owner),
        "remove_critical_mass",
        runtime_args! {
            "referrer" => referrer,
            "dai_equivalent" => dai_equivalent,
            "start_day" => start_day
        },
    );

    // check updated total_amount on critical mass struct
    test.call_contract(
        Sender(owner),
        "get_struct_from_key",
        runtime_args! {
            "key"=>referrer.to_formatted_string(),
            "struct_name"=>DECLARATION_CRITICAL_MASS_DICT.to_string(),
        },
    );
    let ret: Bytes = test.query_named_key("result".to_string());
    let ret: Vec<u8> = Vec::from(ret);
    let ret: CriticalMass = CriticalMass::from_bytes(&ret).unwrap().0;
    assert_eq!(ret.total_amount, total_amount - dai_equivalent);
    assert_eq!(ret.activation_day, critical_mass.activation_day);
}

// Cannot test till StableUSD contract is tested
// // #[test]
// // fn test_referral_token_get_stable_usd_equivalent() {
// //     let (
//         env,
//         owner,
//         test,
//         uniswap_factory,
//         wcspr,
//         flash_swapper,
//         uniswap_pair,
//         uniswap_library,
//         uniswap_router,
//         liquidity_guard,
//         wcspr,
//         erc20,
//         scspr,
//     ) = deploy();
// //     test.call_contract(Sender(owner), "get_stable_usd_equivalent", runtime_args! {});
// //     let ret: U256 = TestInstance::instance(test).result();
// // }

#[test]
fn test_referrer_interest() {
    let (
        env,
        owner,
        test,
        uniswap_factory,
        wcspr,
        flash_swapper,
        uniswap_pair,
        uniswap_library,
        uniswap_router,
        liquidity_guard,
        erc20,
        scspr,
    ) = deploy();
    // // init referral id
    // let referral_count: U256 = 0.into();
    // test.call_contract(
    //     Sender(owner),
    //     "set_referral_count",
    //     runtime_args! {
    //         "referrer" =>referrer,
    //         "value"=>referral_count
    //     },
    // );

    // create stake from staker
    let staker = env.next_user();
    let staker = Key::Account(staker);
    let referrer: Key = Key::Account(owner);

    // create structs and keys
    let stake: Stake = Stake {
        stakes_shares: U256::from(10),
        staked_amount: U256::from(10),
        reward_amount: U256::from(10),
        start_day: 10,
        lock_days: 10,
        final_day: 12,
        close_day: 13,
        scrape_day: U256::from(10),
        dai_equivalent: U256::from(10),
        referrer_shares: U256::from(10),
        referrer,
        is_active: true,
    };
    // gnerate stake id
    test.call_contract(
        Sender(owner),
        "generate_stake_id",
        runtime_args! {
            "staker"=>staker
        },
    );
    let stake_id: Vec<u32> = test.query_named_key("result".to_string());

    let referrer_link: ReferrerLink = ReferrerLink {
        staker,
        stake_id: stake_id.clone(),
        reward_amount: U256::from(10),
        processed_days: U256::from(10),
        is_active: true,
    };
    // generate referrer link id
    test.call_contract(
        Sender(owner),
        "generate_referral_id",
        runtime_args! {
            "referrer"=> referrer
        },
    );
    let referral_id: Vec<u32> = test.query_named_key("result".to_string());
    // crate dictionary keyys
    let stake_key: String = key_gen::generate_key_for_dictionary(&staker, &stake_id);
    let referrer_link_key: String = key_gen::generate_key_for_dictionary(&referrer, &referral_id);
    // structs to dictionary
    test.call_contract(
        Sender(owner),
        "set_struct_from_key",
        runtime_args! {
            "key"=>stake_key.clone(),
            "struct_name"=> DECLARATION_STAKES_DICT,
            "value"=>Bytes::from(stake.clone().into_bytes().unwrap())
        },
    );
    test.call_contract(
        Sender(owner),
        "set_struct_from_key",
        runtime_args! {
            "key"=>referrer_link_key.clone(),
            "struct_name"=> DECLARATION_REFERRER_LINK_DICT,
            "value"=>Bytes::from(referrer_link.clone().into_bytes().unwrap())
        },
    );
    // init critical mass dict
    let critical_mass: CriticalMass = CriticalMass {
        total_amount: U256::from(10),
        activation_day: U256::from(0),
    };
    // set struct to dict
    let value: Vec<u8> = critical_mass.clone().into_bytes().unwrap();
    test.call_contract(
        Sender(owner),
        "set_struct_from_key",
        runtime_args! {
            "key" => referrer.to_formatted_string(),
            "value" => Bytes::from(value),
            "struct_name" => DECLARATION_CRITICAL_MASS_DICT.to_string()
        },
    );
    // call method to test
    let scrape_days: U256 = 0.into();
    test.call_contract(
        Sender(owner),
        "referrer_interest",
        runtime_args! {
            "referral_id" => referral_id.clone(),
            "scrape_days" => scrape_days
        },
    );

    // read referral link, it should now be inactive
    // check updated total_amount on critical mass struct
    test.call_contract(
        Sender(owner),
        "get_struct_from_key",
        runtime_args! {
            "key"=>referrer_link_key.clone(),
            "struct_name"=>DECLARATION_REFERRER_LINK_DICT.to_string(),
        },
    );
    let ret: Bytes = test.query_named_key("result".to_string());
    let ret: Vec<u8> = Vec::from(ret);
    let ret: ReferrerLink = ReferrerLink::from_bytes(&ret).unwrap().0;
    assert_eq!(ret.is_active, false);
}

// // #[test]
// // fn test_referrer_interest_bulk() {
// //     let (
//         env,
//         owner,
//         test,
//         uniswap_factory,
//         wcspr,
//         flash_swapper,
//         uniswap_pair,
//         uniswap_library,
//         uniswap_router,
//         liquidity_guard,
//         wcspr,
//         erc20,
//         scspr,
//     ) = deploy();
// //     let referral_id: Vec<Vec<u32>> = Vec::new();
// //     let scrape_days: Vec<U256> = Vec::new();
// //     test.call_contract(
// //         Sender(owner),
// //         "referrer_interest_bulk",
// //         runtime_args! {
// //             "referral_id" => referral_id,
// //             "scrape_days" => scrape_days
// //         },
// //     );
// // }

// tested above using a wrapper functio
// // #[test]
// // fn test__referrer_interest() {
// //     let (
//         env,
//         owner,
//         test,
//         uniswap_factory,
//         wcspr,
//         flash_swapper,
//         uniswap_pair,
//         uniswap_library,
//         uniswap_router,
//         liquidity_guard,
//         wcspr,
//         erc20,
//         scspr,
//     ) = deploy();
// //     let referrer: Key = Key::Account(owner);
// //     let referral_id: Vec<u32> = Vec::new();
// //     let scrape_days: U256 = 10.into();
// //     test.call_contract(
// //         Sender(owner),
// //         "_referrer_interest",
// //         runtime_args! {
// //             "referrer" => referrer,
// //             "referral_id" => referral_id,
// //             "scrape_days" => scrape_days
// //         },
// //     );
// // }

#[test]
fn test_check_referrals_by_id() {
    let (
        env,
        owner,
        test,
        uniswap_factory,
        wcspr,
        flash_swapper,
        uniswap_pair,
        uniswap_library,
        uniswap_router,
        liquidity_guard,
        erc20,
        scspr,
    ) = deploy();
    // create stake from staker
    let staker = env.next_user();
    let staker = Key::Account(staker);
    let referrer: Key = Key::Account(owner);

    // create structs and keys
    let stake: Stake = Stake {
        stakes_shares: U256::from(10),
        staked_amount: U256::from(10),
        reward_amount: U256::from(10),
        start_day: 10,
        lock_days: 10,
        final_day: 12,
        close_day: 13,
        scrape_day: U256::from(10),
        dai_equivalent: U256::from(10),
        referrer_shares: U256::from(10),
        referrer,
        is_active: true,
    };
    // gnerate stake id
    test.call_contract(
        Sender(owner),
        "generate_stake_id",
        runtime_args! {
            "staker"=>staker
        },
    );
    let stake_id: Vec<u32> = test.query_named_key("result".to_string());

    let referrer_link: ReferrerLink = ReferrerLink {
        staker,
        stake_id: stake_id.clone(),
        reward_amount: U256::from(10),
        processed_days: U256::from(10),
        is_active: true,
    };
    // generate referrer link id
    test.call_contract(
        Sender(owner),
        "generate_referral_id",
        runtime_args! {
            "referrer"=> referrer
        },
    );
    let referral_id: Vec<u32> = test.query_named_key("result".to_string());
    // crate dictionary keyys
    let stake_key: String = key_gen::generate_key_for_dictionary(&staker, &stake_id);
    let referrer_link_key: String = key_gen::generate_key_for_dictionary(&referrer, &referral_id);
    // structs to dictionary
    test.call_contract(
        Sender(owner),
        "set_struct_from_key",
        runtime_args! {
            "key"=>stake_key.clone(),
            "struct_name"=> DECLARATION_STAKES_DICT,
            "value"=>Bytes::from(stake.clone().into_bytes().unwrap())
        },
    );
    test.call_contract(
        Sender(owner),
        "set_struct_from_key",
        runtime_args! {
            "key"=>referrer_link_key.clone(),
            "struct_name"=> DECLARATION_REFERRER_LINK_DICT,
            "value"=>Bytes::from(referrer_link.clone().into_bytes().unwrap())
        },
    );

    // init critical mass dict
    let critical_mass: CriticalMass = CriticalMass {
        total_amount: U256::from(10),
        activation_day: U256::from(0),
    };
    // set struct to dict
    let value: Vec<u8> = critical_mass.clone().into_bytes().unwrap();
    test.call_contract(
        Sender(owner),
        "set_struct_from_key",
        runtime_args! {
            "key" => referrer.to_formatted_string(),
            "value" => Bytes::from(value),
            "struct_name" => DECLARATION_CRITICAL_MASS_DICT.to_string()
        },
    );
    // call method to test
    test.call_contract(
        Sender(owner),
        "check_referrals_by_id",
        runtime_args! {
            "referrer" => referrer,
            "referral_id" => referral_id,
        },
    );

    let _staker: Key = test.query_named_key("staker".to_string());
    let _stake_id: Vec<u32> = test.query_named_key("stake_id".to_string());
    let _is_active_stake: bool = test.query_named_key("is_active_stake".to_string());
    let _is_mature_stake: bool = test.query_named_key("is_mature_stake".to_string());

    assert_eq!(_staker, staker);
    assert_eq!(_stake_id, stake_id);
    assert_eq!(_is_active_stake, true);
    assert_eq!(_is_mature_stake, true);
}

// //////////////
// // SNAPSHOT //
// //////////////

#[test]
fn testmanual_daily_snapshot_point() {
    let (
        env,
        owner,
        test,
        uniswap_factory,
        wcspr,
        flash_swapper,
        uniswap_pair,
        uniswap_library,
        uniswap_router,
        liquidity_guard,
        erc20,
        scspr,
    ) = deploy();
    let update_day: u64 = 3;
    let globals_current_wise_day: U256 = 1.into();
    // setup for liquidity guard trigger
    // initialize liquidity guard status
    test.call_contract(
        Sender(owner),
        "set_liquidity_guard_status",
        runtime_args! {
            "value"=>false
        },
    );
    let status: bool = test.query_named_key(DECLARATION_LIQUIDITY_GUARD_STATUS.to_string());
    assert_eq!(status, false);
    // initialize pair
    let token0 = deploy_wcspr(&env, owner);
    let token1 = deploy_wcspr(&env, owner);
    let token0 = Key::Hash(token0.contract_hash());
    let token1 = Key::Hash(token1.contract_hash());
    let factory_hash = Key::Hash(uniswap_factory.contract_hash());
    uniswap_pair.call_contract(
        Sender(owner),
        "initialize",
        runtime_args! {
            "token0"=>token0,
            "token1"=>token1,
            "factory_hash"=> factory_hash
        },
    );

    // init globals
    test.call_contract(
        Sender(owner),
        "set_globals",
        runtime_args! {
            "field"=>GLOBALS_CURRENT_WISE_DAY.to_string(),
            "value"=>globals_current_wise_day.clone()
        },
    );
    test.call_contract(
        Sender(owner),
        "set_globals",
        runtime_args! {
            "field"=>GLOBALS_TOTAL_STAKED.to_string(),
            "value"=>U256::from(608351533)
        },
    );
    // setup liquidity rate
    test.call_contract(
        Sender(owner),
        "set_liquidity_rate",
        runtime_args! {
            "value"=>U256::from(100000) // amount  = 60835153328
        },
    );
    // now call manual daily snapshot point
    // _current_wise_day() == 5
    test.call_contract(
        Sender(owner),
        "manual_daily_snapshot_point",
        runtime_args! {
            "update_day" => update_day,
        },
    );

    // globals.currentWiseDay will now be globals_current_wise_day +update_day-1
    let new_current_wise_day_globals: U256 = test
        .query_dictionary(GLOBALS_GLOBALS_STRUCT, GLOBALS_CURRENT_WISE_DAY.to_string())
        .unwrap();
    assert_eq!(
        new_current_wise_day_globals,
        globals_current_wise_day + U256::from(update_day - 1)
    );
}

#[test]
fn test_liquidity_guard_trigger() {
    let (
        env,
        owner,
        test,
        uniswap_factory,
        wcspr,
        flash_swapper,
        uniswap_pair,
        uniswap_library,
        uniswap_router,
        liquidity_guard,
        erc20,
        scspr,
    ) = deploy();
    // initialize liquidity guard status
    test.call_contract(
        Sender(owner),
        "set_liquidity_guard_status",
        runtime_args! {
            "value"=>false
        },
    );
    let status: bool = test.query_named_key(DECLARATION_LIQUIDITY_GUARD_STATUS.to_string());
    assert_eq!(status, false);
    // initialize pair
    let token0 = deploy_wcspr(&env, owner);
    let token1 = deploy_wcspr(&env, owner);
    let token0 = Key::Hash(token0.contract_hash());
    let token1 = Key::Hash(token1.contract_hash());
    let factory_hash = Key::Hash(uniswap_factory.contract_hash());
    uniswap_pair.call_contract(
        Sender(owner),
        "initialize",
        runtime_args! {
            "token0"=>token0,
            "token1"=>token1,
            "factory_hash"=> factory_hash
        },
    );
    test.call_contract(Sender(owner), "liquidity_guard_trigger", runtime_args! {});
    // liquidity guard will now be true
    let status: bool = test.query_named_key(DECLARATION_LIQUIDITY_GUARD_STATUS.to_string());
    assert_eq!(status, true);
}

#[test]
fn test_manual_daily_snapshot() {
    let (
        env,
        owner,
        test,
        uniswap_factory,
        wcspr,
        flash_swapper,
        uniswap_pair,
        uniswap_library,
        uniswap_router,
        liquidity_guard,
        erc20,
        scspr,
    ) = deploy();
    let update_day: u64 = 3;
    let globals_current_wise_day: U256 = 1.into();
    // setup for liquidity guard trigger
    // initialize liquidity guard status
    test.call_contract(
        Sender(owner),
        "set_liquidity_guard_status",
        runtime_args! {
            "value"=>false
        },
    );
    let status: bool = test.query_named_key(DECLARATION_LIQUIDITY_GUARD_STATUS.to_string());
    assert_eq!(status, false);
    // initialize pair
    let token0 = deploy_wcspr(&env, owner);
    let token1 = deploy_wcspr(&env, owner);
    let token0 = Key::Hash(token0.contract_hash());
    let token1 = Key::Hash(token1.contract_hash());
    let factory_hash = Key::Hash(uniswap_factory.contract_hash());
    uniswap_pair.call_contract(
        Sender(owner),
        "initialize",
        runtime_args! {
            "token0"=>token0,
            "token1"=>token1,
            "factory_hash"=> factory_hash
        },
    );

    // init globals
    test.call_contract(
        Sender(owner),
        "set_globals",
        runtime_args! {
            "field"=>GLOBALS_CURRENT_WISE_DAY.to_string(),
            "value"=>globals_current_wise_day.clone()
        },
    );
    test.call_contract(
        Sender(owner),
        "set_globals",
        runtime_args! {
            "field"=>GLOBALS_TOTAL_STAKED.to_string(),
            "value"=>U256::from(608351533)
        },
    );
    // setup liquidity rate
    test.call_contract(
        Sender(owner),
        "set_liquidity_rate",
        runtime_args! {
            "value"=>U256::from(100000) // amount  = 60835153328
        },
    );
    // now call manual daily snapshot point
    // _current_wise_day() == 5
    test.call_contract(Sender(owner), "manual_daily_snapshot", runtime_args! {});

    // globals.currentWiseDay will now be globals_current_wise_day +CURRENT_WISE_DAY-1
    let new_current_wise_day_globals: U256 = test
        .query_dictionary(GLOBALS_GLOBALS_STRUCT, GLOBALS_CURRENT_WISE_DAY.to_string())
        .unwrap();
    assert_eq!(
        new_current_wise_day_globals,
        globals_current_wise_day + U256::from(CURRENT_WISE_DAY - 1)
    );
}

#[test]
fn test_snapshot_get_struct_from_key() {
    let (
        env,
        owner,
        test,
        uniswap_factory,
        wcspr,
        flash_swapper,
        uniswap_pair,
        uniswap_library,
        uniswap_router,
        liquidity_guard,
        erc20,
        scspr,
    ) = deploy();
    // create dummy struct
    let lsnapshot_struct: LSnapShot = LSnapShot {
        total_shares: U256::from(10),
        inflation_amount: U256::from(10),
    };
    let value: Vec<u8> = lsnapshot_struct.into_bytes().unwrap();
    let key: U256 = 10.into();
    let struct_name: String = SNAPSHOT_LSNAPSHOTS_DICT.into();

    // set struct by key
    test.call_contract(
        Sender(owner),
        "snapshot_set_struct_from_key",
        runtime_args! {
            "key" => key,
            "struct_name" => struct_name.clone(),
            "value" => Bytes::from(value.clone())
        },
    );

    test.call_contract(
        Sender(owner),
        "snapshot_get_struct_from_key",
        runtime_args! {
            "key" => key,
            "struct_name" => struct_name.clone()
        },
    );
    let ret: Bytes = TestInstance::instance(test).result();
    let ret: Vec<u8> = Vec::from(ret);
    assert_eq!(ret, value);
}

#[test]
fn test_snapshot_set_struct_from_key() {
    let (
        env,
        owner,
        test,
        uniswap_factory,
        wcspr,
        flash_swapper,
        uniswap_pair,
        uniswap_library,
        uniswap_router,
        liquidity_guard,
        erc20,
        scspr,
    ) = deploy();
    // create dummy struct
    let lsnapshot_struct: LSnapShot = LSnapShot {
        total_shares: U256::from(10),
        inflation_amount: U256::from(10),
    };
    let value: Vec<u8> = lsnapshot_struct.into_bytes().unwrap();
    let key: U256 = 10.into();
    let struct_name: String = SNAPSHOT_LSNAPSHOTS_DICT.into();

    // set struct by key
    test.call_contract(
        Sender(owner),
        "snapshot_set_struct_from_key",
        runtime_args! {
            "key" => key,
            "struct_name" => struct_name.clone(),
            "value" => Bytes::from(value.clone())
        },
    );

    // read struct from dictionary
    let ret: Bytes = test
        .query_dictionary(&struct_name, key.to_string())
        .unwrap();
    let ret: Vec<u8> = Vec::from(ret);
    assert_eq!(ret, value);
}

// // ///////////////////
// // // STAKING_TOKEN //
// // ///////////////////

// // #[test]
// // fn test_create_stake_bulk() {
// //     let (
//         env,
//         owner,
//         test,
//         uniswap_factory,
//         wcspr,
//         flash_swapper,
//         uniswap_pair,
//         uniswap_library,
//         uniswap_router,
//         liquidity_guard,
//         wcspr,
//         erc20,
//         scspr,
//     ) = deploy();
// //     let staked_amount: Vec<U256> = Vec::new();
// //     let lock_days: Vec<u64> = Vec::new();
// //     let referrer: Vec<Key> = Vec::new();
// //     test.call_contract(
// //         Sender(owner),
// //         "create_stake_bulk",
// //         runtime_args! {
// //             "staked_amount" => staked_amount,
// //             "lock_days" => lock_days,
// //             "referrer" => referrer
// //         },
// //     );
// // }

// // NOTE Requires hardcoding stable_usd_equivalent in create_stake as stable_usd_equivalent is not setup to execute
// // #[test]
// fn test_create_stake() {
//     let (
//         env,
//         owner,
//         test,
//         uniswap_factory,
//         wcspr,
//         flash_swapper,
//         uniswap_pair,
//         uniswap_library,
//         uniswap_router,
//         liquidity_guard,
//         wcspr,
//         erc20,
//         scspr,
//     ) = deploy();
//     // init stable_usd_equivalent
//     let stable_usd_equivalent = deploy_stable_usd_equivalent(&env, owner, &wcspr, &scspr, &wcspr, &erc20, &uniswap_router);
//     test.call_contract(
//         Sender(owner),
//         "set_stable_usd_equivalent",
//         runtime_args! {
//             "stable_usd_equivalent" => Key::Hash(stable_usd_equivalent.contract_hash())
//         },
//     );
//     // need to create pair and add liquidity for this test

//     //
//     let staked_amount: U256 = 1000001.into();
//     let lock_days: u64 = 2;

//     let staker: Key = Key::Account(owner);
//     let referrer = env.next_user();
//     let referrer = Key::Account(referrer);

//     // need to ass a critical mass entry for referrer
//     let critical_mass: CriticalMass = CriticalMass {
//         total_amount: U256::from(10),
//         activation_day: U256::from(10),
//     };
//     let critical_mass: Vec<u8> = critical_mass.into_bytes().unwrap();
//     test.call_contract(
//         Sender(owner),
//         "set_struct_from_key",
//         runtime_args! {
//             "struct_name"=>DECLARATION_CRITICAL_MASS_DICT.to_string(),
//             "key"=>referrer.to_formatted_string(),
//             "value"=>Bytes::from(critical_mass)
//         },
//     );

//     // mint staked_amount to staker
//     test.call_contract(
//         Sender(owner),
//         "erc20_mint",
//         runtime_args! {
//             "account"=> staker,
//             "amount"=> staked_amount
//         },
//     );

//     //
//     test.call_contract(
//         Sender(owner),
//         "create_stake",
//         runtime_args! {
//             "staked_amount" => staked_amount,
//             "lock_days" => lock_days,
//             "referrer" => referrer
//         },
//     );

//     let (stake_id, start_day, referral_id): (Vec<u32>, U256, Vec<u32>) =
//         test.query_named_key("result".to_string());

//     // read stake and referal struct from dictionary and verify attributes
// }

#[test]
fn test_end_stake() {
    let (
        env,
        owner,
        test,
        uniswap_factory,
        wcspr,
        flash_swapper,
        uniswap_pair,
        uniswap_library,
        uniswap_router,
        liquidity_guard,
        erc20,
        scspr,
    ) = deploy();
    let staker = Key::Account(owner);
    let referrer = env.next_user();
    let referrer = Key::Account(referrer);
    // creata struct
    let stake: Stake = Stake {
        stakes_shares: U256::from(10),
        staked_amount: U256::from(10),
        reward_amount: U256::from(10),
        start_day: 10,
        lock_days: 11,
        final_day: 12,
        close_day: 13,
        scrape_day: U256::from(10),
        dai_equivalent: U256::from(10),
        referrer_shares: U256::from(10),
        referrer: referrer,
        is_active: true,
    };
    let stake_bytes: Vec<u8> = stake.clone().into_bytes().unwrap();
    // generate its itd
    test.call_contract(
        Sender(owner),
        "generate_stake_id",
        runtime_args! {
            "staker"=>staker
        },
    );
    let stake_id: Vec<u32> = test.query_named_key("result".to_string());

    // generate key and set to dictionary
    let stake_dictionary_key: String = generate_key_for_dictionary(&staker, &stake_id);
    test.call_contract(
        Sender(owner),
        "set_struct_from_key",
        runtime_args! {
            "key"=>stake_dictionary_key.clone(),
            "struct_name"=>DECLARATION_STAKES_DICT.to_string(),
            "value"=>Bytes::from(stake_bytes)
        },
    );

    // init critical mass dict
    let critical_mass: CriticalMass = CriticalMass {
        total_amount: 10.into(),
        activation_day: 10.into(), // must be more thhan current wise day
    };
    // set struct to dict
    let value: Vec<u8> = critical_mass.clone().into_bytes().unwrap();
    test.call_contract(
        Sender(owner),
        "set_struct_from_key",
        runtime_args! {
            "key" => referrer.to_formatted_string(),
            "value" => Bytes::from(value),
            "struct_name" => DECLARATION_CRITICAL_MASS_DICT.to_string()
        },
    );

    // call method to test
    test.call_contract(
        Sender(owner),
        "end_stake",
        runtime_args! {
            "stake_id" => stake_id
        },
    );
    // stake status will now be inactive
    let stake: Bytes = test
        .query_dictionary(DECLARATION_STAKES_DICT, stake_dictionary_key.clone())
        .unwrap();
    let stake: Stake = Stake::from_bytes(&Vec::from(stake)).unwrap().0;

    assert_eq!(stake.is_active, false);
}

// wrapper for helper's check mature stake
// // #[test]
// // fn test_check_mature_stake() {
// //     let (
//         env,
//         owner,
//         test,
//         uniswap_factory,
//         wcspr,
//         flash_swapper,
//         uniswap_pair,
//         uniswap_library,
//         uniswap_router,
//         liquidity_guard,
//         wcspr,
//         erc20,
//         scspr,
//     ) = deploy();
// //     let staker: Key = Key::Account(owner);
// //     let stake_id: Vec<u32> = Vec::new();
// //     test.call_contract(
// //         Sender(owner),
// //         "check_mature_stake",
// //         runtime_args! {
// //             "staker" => staker,
// //             "stake_id" => stake_id
// //         },
// //     );
// //     let ret: bool = TestInstance::instance(test).result();
// // }

#[test]
fn test_check_stake_by_id() {
    let (
        env,
        owner,
        test,
        uniswap_factory,
        wcspr,
        flash_swapper,
        uniswap_pair,
        uniswap_library,
        uniswap_router,
        liquidity_guard,
        erc20,
        scspr,
    ) = deploy();
    let staker = Key::Account(owner);
    let referrer = env.next_user();
    let referrer = Key::Account(referrer);
    // creata struct
    let stake: Stake = Stake {
        stakes_shares: U256::from(10),
        staked_amount: U256::from(10),
        reward_amount: U256::from(10),
        start_day: 10,
        lock_days: 11,
        final_day: 12,
        close_day: 13,
        scrape_day: U256::from(10),
        dai_equivalent: U256::from(10),
        referrer_shares: U256::from(10),
        referrer: referrer,
        is_active: true,
    };
    let stake_bytes: Vec<u8> = stake.clone().into_bytes().unwrap();
    // generate its itd
    test.call_contract(
        Sender(owner),
        "generate_stake_id",
        runtime_args! {
            "staker"=>staker.clone()
        },
    );
    let stake_id: Vec<u32> = test.query_named_key("result".to_string());

    // generate key and set to dictionary
    let stake_dictionary_key: String = generate_key_for_dictionary(&staker, &stake_id);
    test.call_contract(
        Sender(owner),
        "set_struct_from_key",
        runtime_args! {
            "key"=>stake_dictionary_key.clone(),
            "struct_name"=>DECLARATION_STAKES_DICT.to_string(),
            "value"=>Bytes::from(stake_bytes)
        },
    );

    test.call_contract(
        Sender(owner),
        "check_stake_by_id",
        runtime_args! {
            "staker" => staker,
            "stake_id" => stake_id
        },
    );
    let (stake, penalty_amount, is_mature): (Bytes, U256, bool) =
        TestInstance::instance(test).result();

    assert_eq!(is_mature, true);
    assert_eq!(penalty_amount, 0.into());
}

////////////
// TIMING //
////////////

#[test]
fn test_current_wise_day() {
    let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
    test.call_contract(Sender(owner), "current_wise_day", runtime_args! {});
    let ret: u64 = TestInstance::instance(test).result();
    assert_eq!(CURRENT_WISE_DAY, ret);
}

#[test]
fn test__current_wise_day() {
    let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
    test.call_contract(Sender(owner), "_current_wise_day", runtime_args! {});
    let ret: u64 = TestInstance::instance(test).result();
    assert_eq!(CURRENT_WISE_DAY, ret);
}

#[test]
fn test__previous_wise_day() {
    let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
    test.call_contract(Sender(owner), "_previous_wise_day", runtime_args! {});
    let ret: u64 = TestInstance::instance(test).result();
    assert_eq!(CURRENT_WISE_DAY - 1, ret);
}

#[test]
fn test__next_wise_day() {
    let (env, owner, test, _, _, _, _, _, _, _, _, _) = deploy();
    test.call_contract(Sender(owner), "_next_wise_day", runtime_args! {});
    let ret: u64 = TestInstance::instance(test).result();
    assert_eq!(CURRENT_WISE_DAY + 1, ret);
}
