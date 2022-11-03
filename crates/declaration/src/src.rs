use crate::data::*;
use casper_contract::contract_api::runtime;
use casper_types::{runtime_args, RuntimeArgs, U256};
use casperlabs_contract_utils::{set_key, ContractContext, ContractStorage};
use common::{
    errors::Errors,
    functions::{block_timestamp, key_to_hash, package_hash},
    keys::LAUNCH_TIME,
};
use global::src::Globals;

pub trait Declaration<Storage: ContractStorage>:
    ContractContext<Storage> + Globals<Storage>
{
    fn init(&self) {
        Globals::init(self);
        set_key(LAUNCH_TIME, U256::from(block_timestamp()));
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
