pub mod parameters
{  
    pub const SECONDS_IN_DAY: u32 = 86400;
}

pub mod Structs
{
    use casper_types::{U256, Key};
    
    extern crate alloc;
    use alloc::{vec::Vec};

    use casper_types_derive::{CLTyped, FromBytes, ToBytes};


    pub const SCRAPES: &str = "scrapes";
    pub const STAKES: &str = "stakes";
    pub const REFERRER_LINK: &str = "referrer_link";
    pub const LIQUIDITY_STAKES: &str = "liquidity_stakes";
    pub const TOTAL_PENALTIES: &str = "total_penalties";
    pub const CRITICAL_MASS: &str = "critical_mass";


    // Keys for global struct
    pub const TOTAL_STAKED: &str = "total_staked";
    pub const TOTAL_SHARES: &str = "total_shares";
    pub const SHARE_PRICE: &str = "share_price";
    pub const CURRENT_WISE_DAY: &str = "current_wise_day";
    pub const REFERRAL_SHARES: &str = "referral_shares";
    pub const LIQUIDITY_SHARES: &str = "liquidity_shares";

    #[derive(Clone, CLTyped, ToBytes, FromBytes)]
    pub struct Stake 
    {
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
        pub is_active: bool
    }
    
    #[derive(Clone, CLTyped, ToBytes, FromBytes)]
    pub struct ReferrerLink 
    {
        pub staker: Key,
        pub stake_id: u32,
        pub reward_amount: U256,
        pub processed_days: U256,
        pub is_active: bool
    }

    pub struct LiquidityStake 
    {
        pub staked_amount: U256,
        pub reward_amount: U256,
        pub start_day: u64,
        pub close_day: u64,
        pub is_active: bool
    }

    #[derive(Clone, CLTyped, ToBytes, FromBytes)]
    pub struct CriticalMass 
    {
        pub total_amount: U256,
        pub activation_day: U256
    }
}