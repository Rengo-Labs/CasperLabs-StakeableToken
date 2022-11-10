use crate::data::*;
use casper_contract::contract_api::runtime;
use casper_types::{runtime_args, RuntimeArgs, U256};
use casperlabs_contract_utils::{set_key, ContractContext, ContractStorage};
use global::{
    errors::Errors,
    functions::{block_timestamp, key_to_hash, package_hash},
    keys::LAUNCH_TIME,
    src::GLOBAL,
};

pub trait DECLARATION<Storage: ContractStorage>:
    ContractContext<Storage> + GLOBAL<Storage>
{
    fn init(&self) {
        GLOBAL::init(self);
        set_key(LAUNCH_TIME, U256::from(block_timestamp()));
        // const MY_UNIX_TIME: u64 = 1668055857679;
        // set_key(
        //     LAUNCH_TIME,
        //     U256::from(MY_UNIX_TIME - (15 * 86400000)) - 84600000,
        // );
        StakeCount::init();
        ReferralCount::init();
        LiquidityStakeCount::init();
        CriticalMass::init();
        Scrapes::init();
        Stakes::init();
        ReferrerLinks::init();
        LiquidityStakes::init();
        ScheduledToEnd::init();
        ReferralSharesToEnd::init();
        TotalPenalties::init();
    }

    fn create_pair(&self) {
        let () = runtime::call_versioned_contract(
            key_to_hash(uniswap_factory(), Errors::InvalidHash4),
            None,
            "create_pair",
            runtime_args! {
                "token_a" => scspr(),
                "token_b" => package_hash(),
                "pair_hash" => uniswap_pair()
            },
        );
    }
}
