
pub mod parameters
{
    use casper_types::{U256, Key, bytesrepr::{ToBytes, FromBytes}};
    use casper_types_derive::{CLTyped, FromBytes, ToBytes};
    extern crate alloc;
    use alloc::{vec::Vec};

    #[derive(Clone, CLTyped, ToBytes, FromBytes)]
    pub struct ConstantParameters
    {
        _decimals : u32,
        yodas_per_wise: U256,
        seconds_in_day: u32,
        min_lock_days: u32,
        formula_day: u32,
        max_lock_days: u32,
        max_bonus_days_a: u32,
        max_bonus_days_b: u32,
        min_referral_days: u32,
        min_stake_amount: U256,
        referrals_rate: U256,                       // 1.000% (direct value, can be used right away)
        inflation_rate_max: U256,                   // 3.000% (indirect -> checks through LiquidityGuard)
        precision_rate: U256, 
        threshold_limit: U256,                      // $10,000 $BUSD
        daily_bonus_a: U256,                        // 25%:1825 = 0.01369863013 per day;
        daily_bonus_b: U256                         // 5%:13505 = 0.00037023324 per day;
    }

    impl ConstantParameters 
    {
        pub fn instance() -> ConstantParameters
        {
            let precision_rate: u128 = 1000000000000000000;         // 1E18
            let threshold_limit: u128 = 10000000000000000000000;    // 10000E18
            let daily_bonus_a: u128 = 13698630136986302;
            let daily_bonus_b: u128 = 370233246945575;
            
            let mut p = ConstantParameters 
            {
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
                daily_bonus_b: daily_bonus_b.into()
            };
            p.yodas_per_wise = U256::from(10).pow(p._decimals.into());
            p
        }
    }
}

pub mod structs
{
    extern crate alloc;
    use alloc::{vec, vec::Vec};
    use casper_types::{U256, Key, bytesrepr::{ToBytes, FromBytes}, CLTyped, CLType};
    use casper_types_derive::{CLTyped, FromBytes, ToBytes};


    #[derive(Clone, CLTyped, ToBytes, FromBytes)]
    pub struct Stake 
    {
        stakes_shares: U256,
        staked_amount: U256,
        reward_amount: U256,
        start_day: u64,
        lock_days: u64,
        final_day: u64,
        close_day: u64,
        scrape_day: U256,
        dai_equivalent: U256,
        referrer_shares: U256,
        referrer: Key,
        is_active: bool
    }

    pub struct ReferrerLink 
    {
        staker: Key,
        stake_id: Vec<u32>,
        reward_amount: U256,
        processed_days: U256,
        is_active: bool
    }

    pub struct LiquidityStake 
    {
        staked_amount: U256,
        reward_amount: U256,
        start_day: u64,
        close_day: u64,
        is_active: bool
    }

    #[derive(Clone, CLTyped, ToBytes, FromBytes)]
    pub struct CriticalMass 
    {
        total_amount: U256,
        activation_day: U256
    }
}