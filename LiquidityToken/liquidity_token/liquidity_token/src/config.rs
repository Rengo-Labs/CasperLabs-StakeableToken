#[allow(non_snake_case)]

pub mod Structs {
    use casper_types::U256;
    extern crate serde;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct ConstantParameters {
        pub _decimals: u32,
        pub yodas_per_wise: U256,
        pub seconds_in_day: u32,
        pub min_lock_days: u32,
        pub formula_day: u32,
        pub max_lock_days: u32,
        pub max_bonus_days_a: u32,
        pub max_bonus_days_b: u32,
        pub min_referral_days: u32,
        pub min_stake_amount: U256,
        pub referrals_rate: U256, // 1.000% (direct value, can be used right away)
        pub inflation_rate_max: U256, // 3.000% (indirect -> checks through LiquidityGuard)
        pub precision_rate: U256,
        pub threshold_limit: U256, // $10,000 $BUSD
        pub daily_bonus_a: U256,   // 25%:1825 = 0.01369863013 per day;
        pub daily_bonus_b: U256,   // 5%:13505 = 0.00037023324 per day;
    }
    // liquidity shares
    #[derive(Serialize, Deserialize)]
    pub struct LSnapShot {
        pub total_shares: U256,
        pub inflation_amount: U256,
    }
    #[derive(Serialize, Deserialize)]
    pub struct LiquidityStake {
        pub staked_amount: U256,
        pub reward_amount: U256,
        pub start_day: u64,
        pub close_day: u64,
        pub is_active: bool,
    }
    impl LiquidityStake {
        pub fn new() -> LiquidityStake {
            LiquidityStake {
                start_day: 0 as u64,
                staked_amount: 0.into(),
                is_active: false,
                close_day: 0 as u64,
                reward_amount: 0.into(),
            }
        }
    }
}
