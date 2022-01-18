use casper_types::{bytesrepr::FromBytes, runtime_args, CLTyped, Key, RuntimeArgs, U256};
use test_env::{Sender, TestContract, TestEnv};

pub struct TestInstance(TestContract);
impl TestInstance {
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: Sender,
        launch_time: U256,
        uniswap_router: Key,
        factory: Key,
        pair_hash: Key,
        liquidity_guard: Key,
        synthetic_cspr: Key,
        wcspr: Key,
        bep20: Key,
    ) -> TestContract {
        TestContract::new(
            env,
            "entry_points.wasm",
            contract_name,
            sender,
            runtime_args! {
                "launch_time" => launch_time,
                "uniswap_router" => uniswap_router,
                "factory" => factory,
                "pair_hash" => pair_hash,
                "liquidity_guard" => liquidity_guard,
                "synthetic_cspr" => synthetic_cspr,
                "wcspr" => wcspr,
                "bep20"=>bep20
            },
        )
    }

    pub fn instance(test: TestContract) -> TestInstance {
        TestInstance(test)
    }

    // Result method
    pub fn result<T: CLTyped + FromBytes>(&self) -> T {
        self.0.query_named_key("result".to_string())
    }
}
