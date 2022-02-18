pub mod structs {
    use casper_types::{Key, U256};
    use casper_types_derive::{CLTyped, FromBytes, ToBytes};
    extern crate alloc;
    use alloc::vec::Vec;

    #[derive(Clone, CLTyped, ToBytes, FromBytes)]
    pub struct Stake {
        pub stakes_shares: U256,
        pub staked_amount: U256,
        pub reward_amount: U256,
        pub start_day: u64,
        pub lock_days: u64,
        pub final_day: u64,
        pub close_day: u64,
        pub scrape_day: U256,
        pub dai_equivalent: U256,
        pub referrer_shares: U256,
        pub referrer: Key,
        pub is_active: bool,
    }

    #[derive(Clone, CLTyped, ToBytes, FromBytes)]
    pub struct ReferrerLink {
        pub staker: Key,
        pub stake_id: Vec<u32>,
        pub reward_amount: U256,
        pub processed_days: U256,
        pub is_active: bool,
    }

    #[derive(Clone, CLTyped, ToBytes, FromBytes)]
    pub struct CriticalMass {
        pub total_amount: U256,
        pub activation_day: U256,
    }

    #[derive(Clone, CLTyped, ToBytes, FromBytes)]
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

pub mod parameters {
    use casper_types::U256;
    use casper_types_derive::{CLTyped, FromBytes, ToBytes};
    extern crate alloc;
    use alloc::vec::Vec;

    #[derive(Clone, CLTyped, ToBytes, FromBytes)]
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
        pub threshold_limit: U256, // $10,000 $STABLE_USD
        pub daily_bonus_a: U256,   // 25%:1825 = 0.01369863013 per day;
        pub daily_bonus_b: U256,   // 5%:13505 = 0.00037023324 per day;
    }

    impl ConstantParameters {
        pub fn instance() -> ConstantParameters {
            let precision_rate: u128 = 1000000000000000000; // 1E18
            let threshold_limit: u128 = 10000000000000000000000; // 10000E18
            let daily_bonus_a: u128 = 13698630136986302;
            let daily_bonus_b: u128 = 370233246945575;

            let mut p = ConstantParameters {
                _decimals: 18,
                yodas_per_wise: U256::from(0),
                //yodas_per_wise: U256::from(10).pow(_decimals.into()),             // cannot access struct's other methods here
                seconds_in_day: 86400,
                min_lock_days: 1,
                formula_day: 25,
                max_lock_days: 15330,
                max_bonus_days_a: 1825,
                max_bonus_days_b: 13505,
                min_referral_days: 365,
                min_stake_amount: 1000000.into(),
                referrals_rate: 366816973.into(),
                inflation_rate_max: 103000.into(),
                precision_rate: precision_rate.into(),
                threshold_limit: threshold_limit.into(),
                daily_bonus_a: daily_bonus_a.into(),
                daily_bonus_b: daily_bonus_b.into(),
            };
            p.yodas_per_wise = U256::from(10).pow(p._decimals.into());
            p
        }
    }
}