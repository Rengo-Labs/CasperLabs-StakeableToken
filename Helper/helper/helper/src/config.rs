pub mod parameters
{  
    pub const SECONDS_IN_DAY: u32 = 86400;
}

pub mod Structs
{
    use casper_types::{U256, Key, bytesrepr::{ToBytes, FromBytes}, CLTyped, CLType};
    extern crate serde;
    use serde::{Serialize, Deserialize};


    pub const SCRAPES: &str = "scrapes";
    pub const STAKES: &str = "stakes";
    pub const REFERRER_LINK: &str = "referrer_link";
    pub const LIQUIDITY_STAKES: &str = "liquidity_stakes";

    #[derive(Serialize, Deserialize)]
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
    
    #[derive(Serialize, Deserialize)]
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

    #[derive(Serialize, Deserialize)]
    pub struct CriticalMass 
    {
        total_amount: U256,
        activation_day: U256
    }
}