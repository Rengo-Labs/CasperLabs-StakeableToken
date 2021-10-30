
pub mod parameters
{
    use casper_types::{U256, Key};
    
    struct Parameters
    {
        _decimals : u32,
        YODAS_PER_WISE: U256,
        SECONDS_IN_DAY: u32,
        MIN_LOCK_DAYS: u32,
        FORMULA_DAY: u32,
        MAX_LOCK_DAYS: u32,
        MAX_BONUS_DAYS_A: u32,
        MAX_BONUS_DAYS_B: u32,
        MIN_REFERRAL_DAYS: u32,
        MIN_STAKE_AMOUNT: U256,
        REFERRALS_RATE: U256,                       // 1.000% (direct value, can be used right away)
        INFLATION_RATE_MAX: U256,                   // 3.000% (indirect -> checks throgh LiquidityGuard)
        INFLATION_RATE: U256,                       // 3.000% (indirect -> checks throgh LiquidityGuard)
        LIQUIDITY_RATE: U256,                       // 0.006% (indirect -> checks throgh LiquidityGuard)
        PRECISION_RATE: U256, 
        THRESHOLD_LIMIT: U256,                      // $10,000 $BUSD
        DAILY_BONUS_A: U256,                        // 25%:1825 = 0.01369863013 per day;
        DAILY_BONUS_B: U256                         // 5%:13505 = 0.00037023324 per day;
    }

    impl Parameters 
    {
        pub fn instance(&self) -> Parameters
        {
            let precision_rate: u128 = 1000000000000000000;         // 1E18
            let threshold_limit: u128 = 10000000000000000000000;    // 10000E18
            let daily_bonus_a: u128 = 13698630136986302;
            let daily_bonus_b: u128 = 370233246945575;
            
            Parameters 
            {
                _decimals: 18,
                YODAS_PER_WISE: U256::from(10).pow(self._decimals.into()),
                SECONDS_IN_DAY: 86400,
                MIN_LOCK_DAYS: 1,
                FORMULA_DAY: 25,
                MAX_LOCK_DAYS: 15330,
                MAX_BONUS_DAYS_A: 1825,
                MAX_BONUS_DAYS_B: 13505,
                MIN_REFERRAL_DAYS: 365,
                MIN_STAKE_AMOUNT: 1000000.into(),
                REFERRALS_RATE: 366816973.into(),
                INFLATION_RATE_MAX: 103000.into(),
                INFLATION_RATE: 103000.into(),
                LIQUIDITY_RATE: 100006.into(),
                PRECISION_RATE: precision_rate.into(),         
                THRESHOLD_LIMIT: threshold_limit.into(),    
                DAILY_BONUS_A: daily_bonus_a.into(),
                DAILY_BONUS_B: daily_bonus_b.into()
            }
        }
    }
}

pub mod Structs
{
    use casper_types::{U256, Key, bytesrepr::{ToBytes, FromBytes}, CLTyped, CLType};
    extern crate serde;
    use serde::{Serialize, Deserialize};


    #[derive(Serialize, Deserialize)]
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
        stake_id: u32,
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

    #[derive(Serialize, Deserialize)]
    pub struct CriticalMass 
    {
        total_amount: U256,
        activation_day: U256
    }
}