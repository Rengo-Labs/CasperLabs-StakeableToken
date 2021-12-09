pub mod parameters {
    use casper_types::{U256, Key, bytesrepr::{ToBytes, FromBytes}};
    use casper_types_derive::{CLTyped, FromBytes, ToBytes};
    extern crate alloc;
    use alloc::{vec::Vec};

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
        pub inflation_rate: U256, // 3.000% (indirect -> checks through LiquidityGuard)
        pub liquidity_rate: U256, // 0.006% (indirect -> checks through LiquidityGuard)
        pub precision_rate: U256,
        pub threshold_limit: U256, // $10,000 $BUSD
        pub daily_bonus_a: U256,   // 25%:1825 = 0.01369863013 per day;
        pub daily_bonus_b: U256,   // 5%:13505 = 0.00037023324 per day;
    }
}

pub mod structs {
    use casper_types::{U256, Key, bytesrepr::{ToBytes, FromBytes}};
    use casper_types_derive::{CLTyped, FromBytes, ToBytes};
    extern crate alloc;
    use alloc::{vec::Vec};

    pub const SCRAPES: &str = "scrapes";
    pub const STAKES: &str = "stakes";
    pub const REFERRER_LINK: &str = "referrer_link";
    pub const LIQUIDITY_STAKES: &str = "liquidity_stakes";
    pub const TOTAL_PENALTIES: &str = "total_penalties";
    pub const CRITICAL_MASS: &str = "critical_mass";
    pub const RSNAPSHOT: &str = "rsnapshot";

    // Keys for global struct
    pub const TOTAL_STAKED: &str = "total_staked";
    pub const TOTAL_SHARES: &str = "total_shares";
    pub const SHARE_PRICE: &str = "share_price";
    pub const CURRENT_WISE_DAY: &str = "current_wise_day";
    pub const REFERRAL_SHARES: &str = "referral_shares";
    pub const LIQUIDITY_SHARES: &str = "liquidity_shares";

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
        pub stake_id: u32,
        pub reward_amount: U256,
        pub processed_days: U256,
        pub is_active: bool,
    }

    pub struct LiquidityStake {
        pub staked_amount: U256,
        pub reward_amount: U256,
        pub start_day: u64,
        pub close_day: u64,
        pub is_active: bool,
    }

    #[derive(Clone, CLTyped, ToBytes, FromBytes)]
    pub struct CriticalMass {
        pub total_amount: U256,
        pub activation_day: U256,
    }

    // referral shares
    #[derive(Clone, CLTyped, ToBytes, FromBytes)]
    pub struct RSnapshot {
        pub total_shares: U256,
        pub inflation_amount: U256,
        pub scheduled_to_end: U256,
    }
}
