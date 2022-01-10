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

use test_env::{Sender, TestContract, TestEnv};
use crate::wise_instance::WiseTestInstance;
use std::time::{SystemTime, UNIX_EPOCH};

const EXAMPLE_ERC20_TOKEN: &str = "erc20_token.wasm";
const CONTRACT_ERC20_TEST: &str = "erc20_test.wasm";
const CONTRACT_ERC20_TEST_CALL: &str = "erc20_test_call.wasm";
const ERC20_TOKEN_CONTRACT_KEY: &str = "erc20_token_contract";

const ARG_NAME: &str = "name";
const ARG_SYMBOL: &str = "symbol";
const ARG_DECIMALS: &str = "decimals";
const ARG_TOTAL_SUPPLY: &str = "total_supply";

const TEST_CONTRACT_KEY: &str = "test_contract";

const _ERROR_INVALID_CONTEXT: u16 = u16::MAX;

const TOKEN_NAME: &str = "CasperTest";
const TOKEN_SYMBOL: &str = "CSPRT";
const TOKEN_DECIMALS: u8 = 100;
const TOKEN_TOTAL_SUPPLY: u64 = 1_000_000_000;
const ERC20_TEST_CALL_KEY: &str = "erc20_test_call";

static ACCOUNT_1_SECRET_KEY: Lazy<SecretKey> =
    Lazy::new(|| SecretKey::secp256k1_from_bytes(&[221u8; 32]).unwrap());
static ACCOUNT_1_PUBLIC_KEY: Lazy<PublicKey> =
    Lazy::new(|| PublicKey::from(&*ACCOUNT_1_SECRET_KEY));
static ACCOUNT_1_ADDR: Lazy<AccountHash> = Lazy::new(|| ACCOUNT_1_PUBLIC_KEY.to_account_hash());

static ACCOUNT_2_SECRET_KEY: Lazy<SecretKey> =
    Lazy::new(|| SecretKey::secp256k1_from_bytes(&[212u8; 32]).unwrap());
static ACCOUNT_2_PUBLIC_KEY: Lazy<PublicKey> =
    Lazy::new(|| PublicKey::from(&*ACCOUNT_2_SECRET_KEY));
static ACCOUNT_2_ADDR: Lazy<AccountHash> = Lazy::new(|| ACCOUNT_2_PUBLIC_KEY.to_account_hash());

#[derive(Copy, Clone)]
struct TestContext {
    erc20_token: ContractHash
}

fn erc20_setup() -> (InMemoryWasmTestBuilder, TestContext) {
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&*DEFAULT_RUN_GENESIS_REQUEST);

    let id: Option<u64> = None;
    let transfer_1_args = runtime_args! {
        mint::ARG_TARGET => *ACCOUNT_1_ADDR,
        mint::ARG_AMOUNT => MINIMUM_ACCOUNT_CREATION_BALANCE,
        mint::ARG_ID => id,
    };
    let transfer_2_args = runtime_args! {
        mint::ARG_TARGET => *ACCOUNT_2_ADDR,
        mint::ARG_AMOUNT => MINIMUM_ACCOUNT_CREATION_BALANCE,
        mint::ARG_ID => id,
    };

    let transfer_request_1 =
        ExecuteRequestBuilder::transfer(*DEFAULT_ACCOUNT_ADDR, transfer_1_args).build();
    let transfer_request_2 =
        ExecuteRequestBuilder::transfer(*DEFAULT_ACCOUNT_ADDR, transfer_2_args).build();

    let install_request_1 = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        EXAMPLE_ERC20_TOKEN,
        runtime_args! {
            ARG_NAME => TOKEN_NAME,
            ARG_SYMBOL => TOKEN_SYMBOL,
            ARG_DECIMALS => TOKEN_DECIMALS,
            ARG_TOTAL_SUPPLY => U256::from(TOKEN_TOTAL_SUPPLY),
        },
    )
    .build();
    let install_request_2 = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        CONTRACT_ERC20_TEST,
        RuntimeArgs::default(),
    )
    .build();
    let install_request_3 = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        CONTRACT_ERC20_TEST_CALL,
        RuntimeArgs::default(),
    )
    .build();

    builder.exec(transfer_request_1).expect_success().commit();
    builder.exec(transfer_request_2).expect_success().commit();
    builder.exec(install_request_1).expect_success().commit();
    builder.exec(install_request_2).expect_success().commit();
    builder.exec(install_request_3).expect_success().commit();

    let account = builder
        .get_account(*DEFAULT_ACCOUNT_ADDR)
        .expect("should have account");

    let erc20_token = account
        .named_keys()
        .get(ERC20_TOKEN_CONTRACT_KEY)
        .and_then(|key| key.into_hash())
        .map(ContractHash::new)
        .expect("should have contract hash");

    let test_context = TestContext {
        erc20_token
    };

    (builder, test_context)
}

fn deploy_bep20(env: &TestEnv, owner: AccountHash) -> TestContract 
{
    let initial_supply: U256 = 1000.into();
    // deploy wcspr contract
    let bep20 = TestContract::new(
        &env,
        "bep20-token.wasm",
        "bep20",
        Sender(owner),
        runtime_args! {
            "name" => "Wise Token",
            "symbol" => "WISB",
            "initial_supply" => initial_supply
        },
    );

    bep20
}
fn deploy_busd_eq(env: TestEnv, owner: AccountHash, wise: TestContract) -> TestContract 
{
    let router: Key = wise.query_named_key("router_contract_hash".to_string());
    let sbnb: Key = wise.query_named_key("sbnb_contract_hash".to_string());
    let wbnb: Key = wise.query_named_key("wbnb_contract_hash".to_string());
    let (_, busd) = erc20_setup();                  // since busd is an ERC20 token, using casper's erc20 as busd

 
    // deploy busd eq. contract
    let busd_contract = TestContract::new(
        &env,
        "busd_equivalent.wasm",
        "Busd_Eq",
        Sender(owner),
        runtime_args! {
            "wise" => Key::Hash(wise.contract_hash()),
            "sbnb" => sbnb,
            "wbnb" => wbnb,
            "busd" => Key::from(busd.erc20_token),
            "router" => router
        },
    );

    busd_contract
}

fn deploy_pair_contract( env: &TestEnv, owner: AccountHash, factory_contract: Key, flash_swapper: Key) -> TestContract
{
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
    wbnb: &TestContract,
    synthetic_helper: &TestContract,
    uniswap_pair: &TestContract,
    uniswap_router: &TestContract,
    bep20: &TestContract,
) -> TestContract {
    TestContract::new(
        &env,
        "synthetic_token.wasm",
        "synthetic_token",
        Sender(owner),
        runtime_args! {
            "wbnb" => Key::Hash(wbnb.contract_hash()),
            "synthetic_helper" => Key::Hash(synthetic_helper.contract_hash()),
            "uniswap_pair" => Key::Hash(uniswap_pair.contract_hash()),
            "uniswap_router" => Key::Hash(uniswap_router.contract_hash()),
            "bep20" => Key::Hash(bep20.contract_hash()),
        },
    )
}

fn deploy_sbnb(
    env: &TestEnv,
    owner: AccountHash,
    bep20: &TestContract,
    uniswap_factory: &TestContract,
    synthetic_helper: &TestContract,
    synthetic_token: &TestContract,
) -> TestContract {
    TestContract::new(
        &env,
        "sbnb.wasm",
        "sbnb",
        Sender(owner),
        runtime_args! {
            "bep20" => Key::Hash(bep20.contract_hash()),
            "uniswap_factory" => Key::Hash(uniswap_factory.contract_hash()),
            "synthetic_helper" => Key::Hash(synthetic_helper.contract_hash()),
            "synthetic_token" => Key::Hash(synthetic_token.contract_hash())
        },
    )
}

fn deploy_wise() -> (
    TestEnv,                // env
    AccountHash,            // owner
    TestContract,           // wise contract
    WiseTestInstance,       // WiseTestInstance
    TestContract,           // bep20
    TestContract,           // flash_swapper
    TestContract,           // factory
    TestContract,           // Router
    TestContract,           // WCSPR
    TestContract,           // SBNB
) {
    let env = TestEnv::new();
    let owner = env.next_user();

    let (_, erc20_token) = erc20_setup();


    // deploy factory contract
    let factory_contract = TestContract::new(
        &env,
        "factory.wasm",
        "factory",
        Sender(owner),
        runtime_args! {
            "fee_to_setter" => Key::from(erc20_token.erc20_token)
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
    
    // deploy bep20
    let bep20: TestContract = deploy_bep20(&env, owner);

    // Deploy synthetic token
    let synthetic_token =  deploy_synthetic_token(
        &env,
        owner,
        &wcspr,
        &synthetic_helper,
        &pair_contract,
        &router_contract,
        &bep20,
    );

    //Deploy Sbnb
    let sbnb_contract = deploy_sbnb(
        &env,
        owner,
        &bep20,
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
            "sbnb" => Key::Hash(sbnb_contract.contract_hash()),
            "router" => Key::Hash(router_contract.contract_hash()),
            "factory" => Key::Hash(factory_contract.contract_hash()),
            "pair" => Key::Hash(pair_contract.contract_hash()),
            "liquidity_guard" => Key::Hash(liquidity_guard_contract.contract_hash()),
            "wbnb" => Key::Hash(wcspr.contract_hash()),
            "launch_time" => launch_time
        },
    );

    // deploy Test contract
    let test_contract = WiseTestInstance::new(
        &env,
        Key::Hash(wise_contract.contract_hash()),
        Key::Hash(bep20.contract_hash()),
        Sender(owner),
    );

    // change keeper to test contract
    wise_contract.call_contract(Sender(owner), "change_keeper", runtime_args!{"keeper" => test_contract.test_contract_package_hash()});

    // insert router to the factory's white-list
    let router_package_hash: ContractPackageHash = router_contract.query_named_key("package_hash".to_string());
    factory_contract.call_contract(Sender(owner), "set_white_list" ,runtime_args! {"white_list" => Key::from(router_package_hash)});  

    (env, owner, wise_contract, test_contract, bep20, flash_swapper, factory_contract, router_contract, wcspr, sbnb_contract)
}


#[test]
fn test_erc20_deploy() {

    let (_, test_context) = erc20_setup();
    assert_ne!(Key::from(test_context.erc20_token), Key::Hash([0u8;32]));
}


#[test]
fn test_wise_deploy() {

    let (env, owner, wise_contract, test_contract, _, _, _, _, _, _) = deploy_wise();
    assert_ne!(Key::from(test_contract.test_contract_hash()), Key::Hash([0u8;32]));
}

#[test]
fn test_busd_deploy() {
    let (env, owner, wise_contract, _, _, _, _, _, _, _) = deploy_wise();
    let busd_contract = deploy_busd_eq(env, owner, wise_contract);

    assert_ne!(Key::Hash(busd_contract.contract_hash()), Key::Hash([0u8;32]));
}

#[test]
fn set_liquidity_transfomer() {

    let (_, owner, _, test_contract, _, _, _, _, _, _) = deploy_wise();
    test_contract.set_liquidity_transfomer(Sender(owner), Key::from(owner));
}

#[test]
fn set_busd() {
    let (env, owner, wise_contract, test_contract, _, _, _, _, _, _) = deploy_wise();
    let busd_contract = deploy_busd_eq(env, owner, wise_contract);


    test_contract.set_busd(Sender(owner), Key::Hash(busd_contract.contract_hash()));
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

    test_contract.set_liquidity_transfomer(Sender(owner), test_contract.test_contract_package_hash());      // make test contract as liquidity_transformer

    test_contract.mint_supply(Sender(owner), Key::from(user), 50.into());
    let amount: U256 = test_contract.balance_of(&wise, Key::from(user));
    
    assert_eq!(amount, 50.into())             // 1000 + 50 = 1050
}


//#[test]
fn create_stake_with_bnb() {

    let (env, owner, wise_contract, test_contract, _, flash_swapper, factory_contract, router_contract, wcspr, sbnb) = deploy_wise();
    let user = env.next_user();

    // let router_library: ContractHash = router_contract.query_named_key("library_hash".to_string());
    // println!("LIbrary: {}", router_library);

    // let router_sbnb: ContractHash = router_contract.query_named_key("wcspr".to_string());
    // let wise_sbnb: Key = wise_contract.query_named_key("wbnb_contract_hash".to_string());
    // assert_eq!(Key::from(router_sbnb), wise_sbnb);

    // let router_hash: Key = Key::Hash(wcspr.contract_hash());
    // let router_hash: ContractHash = ContractHash::from(router_hash.into_hash().unwrap_or_default());

    // let wise_hash: Key = Key::Hash(wcspr.contract_hash());

    // let zero: Key = Key::Hash([0u8;32]);
    // assert_eq!(zero, Key::from(router_hash));

    
    // mint sbnb, and wise to test_contract
    let _:() = sbnb.call_contract(Sender(owner), "mint", runtime_args!{"account" => test_contract.test_contract_package_hash(), "amount" => U256::from("900000000000000000000")});
    let _:() = wise_contract.call_contract(Sender(owner), "mint", runtime_args!{"account" => test_contract.test_contract_package_hash(), "amount" => U256::from("900000000000000000000")});    
        
    // create pair of wcspr and sbnb
    let pair: TestContract = deploy_pair_contract(&env, owner, Key::Hash(factory_contract.contract_hash()), Key::Hash(flash_swapper.contract_hash()));
    add_liquidity_cspr(&test_contract, &owner, &sbnb, &Key::from(user), &Key::Hash(router_contract.contract_hash()),  &Key::Hash(pair.contract_hash()));
    
    // create pair of sbnb and wise contract
    let pair: TestContract = deploy_pair_contract(&env, owner, Key::Hash(factory_contract.contract_hash()), Key::Hash(flash_swapper.contract_hash()));
    add_liquidity(&test_contract, &owner, &sbnb, &wise_contract, &Key::from(user), &Key::Hash(router_contract.contract_hash()),  &Key::Hash(pair.contract_hash()));

    
    let test_hash: Key = test_contract.test_contract_hash();
    let lock_days: u64 = 15;
    let referrer: Key = Key::from(owner);
    let amount: U256 = 40.into();
    

    test_contract.create_stake_with_bnb(Sender(owner), test_hash, lock_days, referrer, amount);
}

#[test]
fn extend_lt_auction() {
    
    let (env, owner, wise_contract, test_contract, _, flash_swapper, factory_contract, router_contract, wcspr, sbnb) = deploy_wise();
    let user = env.next_user();

    test_contract.set_liquidity_transfomer(Sender(owner), test_contract.test_contract_package_hash());      // make test contract as liquidity_transformer
    test_contract.extend_lt_auction(Sender(owner));
}


fn add_liquidity_cspr(test_contract: &WiseTestInstance, owner: &AccountHash, token: &TestContract, to: &Key, router_hash: &Key, pair: &Key) {

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
        self_hash
    )
}

fn add_liquidity(
    test_contract: &WiseTestInstance, 
    owner: &AccountHash, 
    token_a: &TestContract, 
    token_b: &TestContract,
    to: &Key, 
    router_hash: &Key, 
    pair: &Key) {

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
        Some(*pair)
    )
}