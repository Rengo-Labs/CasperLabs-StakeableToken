use casper_types::{Key, U256};
use casper_types_derive::{CLTyped, FromBytes, ToBytes};
use casperlabs_contract_utils::{get_key, set_key, Dict};
use common::{
    functions::{account_zero_address, package_hash, zero_address},
    keys::*,
};

pub const DECIMALS: U256 = U256([9, 0, 0, 0]);
pub const YODAS_PER_STAKEABLE: U256 = U256([1_000_000_000, 0, 0, 0]); // 10 ** DECIMALS

pub const MILLI_SECONDS_IN_DAY: u32 = 86400000;
pub const MIN_LOCK_DAYS: u16 = 1;
pub const FORMULA_DAY: u16 = 25;
pub const MAX_LOCK_DAYS: u16 = 15330;
pub const MAX_BONUS_DAYS_A: u16 = 1825;
pub const MAX_BONUS_DAYS_B: u16 = 13505;
pub const MIN_REFERRAL_DAYS: u16 = 365;

pub const MIN_STAKE_AMOUNT: u32 = 1_000_000;
pub const REFERRALS_RATE: u32 = 366_816_973;
pub const INFLATION_RATE_MAX: u32 = 103000;

pub const PRECISION_RATE: u64 = 1_000_000_000; // 1E9
pub const THRESHOLD_LIMIT: u128 = 10_000_000_000_000; // 10000E9 // $10,000

pub const DAILY_BONUS_A: u128 = 13698630136986302; // 25%:1825 = 0.01369863013 per day;
pub const DAILY_BONUS_B: u128 = 370233246945575; // 5%:13505 = 0.00037023324 per day;

#[derive(Debug, Clone, Copy, CLTyped, ToBytes, FromBytes)]
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
impl Default for Stake {
    fn default() -> Self {
        Self {
            stakes_shares: Default::default(),
            staked_amount: Default::default(),
            reward_amount: Default::default(),
            start_day: Default::default(),
            lock_days: Default::default(),
            final_day: Default::default(),
            close_day: Default::default(),
            scrape_day: Default::default(),
            dai_equivalent: Default::default(),
            referrer_shares: Default::default(),
            referrer: account_zero_address(),
            is_active: Default::default(),
        }
    }
}

#[derive(Debug, Clone, CLTyped, ToBytes, FromBytes)]
pub struct ReferrerLink {
    pub staker: Key,
    pub stake_id: Vec<u32>,
    pub reward_amount: U256,
    pub processed_days: U256,
    pub is_active: bool,
}
impl Default for ReferrerLink {
    fn default() -> Self {
        Self {
            staker: account_zero_address(),
            stake_id: Default::default(),
            reward_amount: Default::default(),
            processed_days: Default::default(),
            is_active: Default::default(),
        }
    }
}

#[derive(Default, Debug, Clone, Copy, CLTyped, ToBytes, FromBytes)]
pub struct LiquidityStake {
    pub staked_amount: U256,
    pub reward_amount: U256,
    pub start_day: u64,
    pub close_day: u64,
    pub is_active: bool,
}

#[derive(Default, Debug, Clone, Copy, CLTyped, ToBytes, FromBytes)]
pub struct CriticalMass_ {
    pub total_amount: U256,
    pub activation_day: U256,
}

pub struct StakeCount {
    dict: Dict,
}
impl StakeCount {
    pub fn instance() -> StakeCount {
        StakeCount {
            dict: Dict::instance(STAKE_COUNT_DICT),
        }
    }
    pub fn init() {
        Dict::init(STAKE_COUNT_DICT)
    }
    pub fn get(&self, key: &Key) -> U256 {
        self.dict.get_by_key(key).unwrap_or_default()
    }
    pub fn set(&self, key: &Key, value: U256) {
        self.dict.set_by_key(key, value);
    }
}

pub struct ReferralCount {
    dict: Dict,
}
impl ReferralCount {
    pub fn instance() -> ReferralCount {
        ReferralCount {
            dict: Dict::instance(REFERRAL_COUNT_DICT),
        }
    }
    pub fn init() {
        Dict::init(REFERRAL_COUNT_DICT)
    }
    pub fn get(&self, key: &Key) -> U256 {
        self.dict.get_by_key(key).unwrap_or_default()
    }
    pub fn set(&self, key: &Key, value: U256) {
        self.dict.set_by_key(key, value);
    }
}

pub struct LiquidityStakeCount {
    dict: Dict,
}
impl LiquidityStakeCount {
    pub fn instance() -> LiquidityStakeCount {
        LiquidityStakeCount {
            dict: Dict::instance(LIQUIDITY_STAKE_COUNT_DICT),
        }
    }
    pub fn init() {
        Dict::init(LIQUIDITY_STAKE_COUNT_DICT)
    }
    pub fn get(&self, key: &Key) -> U256 {
        self.dict.get_by_key(key).unwrap_or_default()
    }
    pub fn set(&self, key: &Key, value: U256) {
        self.dict.set_by_key(key, value);
    }
}

pub struct CriticalMass {
    dict: Dict,
}
impl CriticalMass {
    pub fn instance() -> CriticalMass {
        CriticalMass {
            dict: Dict::instance(CRITICAL_MASS_DICT),
        }
    }
    pub fn init() {
        Dict::init(CRITICAL_MASS_DICT)
    }
    pub fn get(&self, key: &Key) -> CriticalMass_ {
        self.dict.get_by_key(key).unwrap_or_default()
    }
    pub fn set(&self, key: &Key, value: CriticalMass_) {
        self.dict.set_by_key(key, value);
    }
}

pub struct Scrapes {
    dict: Dict,
}
impl Scrapes {
    pub fn instance() -> Scrapes {
        Scrapes {
            dict: Dict::instance(SCRAPES_DICT),
        }
    }
    pub fn init() {
        Dict::init(SCRAPES_DICT)
    }
    pub fn get(&self, key0: &Key, key1: &Vec<u32>) -> U256 {
        self.dict.get_by_values((key0, key1)).unwrap_or_default()
    }
    pub fn set(&self, key0: &Key, key1: &Vec<u32>, value: U256) {
        self.dict.set_by_values((key0, key1), value);
    }
}

pub struct Stakes {
    dict: Dict,
}
impl Stakes {
    pub fn instance() -> Stakes {
        Stakes {
            dict: Dict::instance(STAKES_DICT),
        }
    }
    pub fn init() {
        Dict::init(STAKES_DICT)
    }
    pub fn get(&self, key0: &Key, key1: &Vec<u32>) -> Stake {
        self.dict.get_by_values((key0, key1)).unwrap_or_default()
    }
    pub fn set(&self, key0: &Key, key1: &Vec<u32>, value: Stake) {
        self.dict.set_by_values((key0, key1), value);
    }
}

pub struct ReferrerLinks {
    dict: Dict,
}
impl ReferrerLinks {
    pub fn instance() -> ReferrerLinks {
        ReferrerLinks {
            dict: Dict::instance(REFERRER_LINKS_DICT),
        }
    }
    pub fn init() {
        Dict::init(REFERRER_LINKS_DICT)
    }
    pub fn get(&self, key0: &Key, key1: &Vec<u32>) -> ReferrerLink {
        self.dict.get_by_values((key0, key1)).unwrap_or_default()
    }
    pub fn set(&self, key0: &Key, key1: &Vec<u32>, value: ReferrerLink) {
        self.dict.set_by_values((key0, key1), value);
    }
}

pub struct LiquidityStakes {
    dict: Dict,
}
impl LiquidityStakes {
    pub fn instance() -> LiquidityStakes {
        LiquidityStakes {
            dict: Dict::instance(LIQUIDITY_STAKES_DICT),
        }
    }
    pub fn init() {
        Dict::init(LIQUIDITY_STAKES_DICT)
    }
    pub fn get(&self, key0: &Key, key1: &Vec<u32>) -> LiquidityStake {
        self.dict.get_by_values((key0, key1)).unwrap_or_default()
    }
    pub fn set(&self, key0: &Key, key1: &Vec<u32>, value: LiquidityStake) {
        self.dict.set_by_values((key0, key1), value);
    }
}

pub struct ScheduledToEnd {
    dict: Dict,
}
impl ScheduledToEnd {
    pub fn instance() -> ScheduledToEnd {
        ScheduledToEnd {
            dict: Dict::instance(SCHEDULED_TO_END_DICT),
        }
    }
    pub fn init() {
        Dict::init(SCHEDULED_TO_END_DICT)
    }
    pub fn get(&self, key: &U256) -> U256 {
        self.dict.get(&key.to_string()).unwrap_or_default()
    }
    pub fn set(&self, key: &U256, value: U256) {
        self.dict.set(&key.to_string(), value);
    }
}

pub struct ReferralSharesToEnd {
    dict: Dict,
}
impl ReferralSharesToEnd {
    pub fn instance() -> ReferralSharesToEnd {
        ReferralSharesToEnd {
            dict: Dict::instance(REFERRAL_SHARES_TO_END_DICT),
        }
    }
    pub fn init() {
        Dict::init(REFERRAL_SHARES_TO_END_DICT)
    }
    pub fn get(&self, key: &U256) -> U256 {
        self.dict.get(&key.to_string()).unwrap_or_default()
    }
    pub fn set(&self, key: &U256, value: U256) {
        self.dict.set(&key.to_string(), value);
    }
}

pub struct TotalPenalties {
    dict: Dict,
}
impl TotalPenalties {
    pub fn instance() -> TotalPenalties {
        TotalPenalties {
            dict: Dict::instance(TOTAL_PENALTIES_DICT),
        }
    }
    pub fn init() {
        Dict::init(TOTAL_PENALTIES_DICT)
    }
    pub fn get(&self, key: &U256) -> U256 {
        self.dict.get(&key.to_string()).unwrap_or_default()
    }
    pub fn set(&self, key: &U256, value: U256) {
        self.dict.set(&key.to_string(), value);
    }
}

pub fn launch_time() -> U256 {
    get_key(LAUNCH_TIME).unwrap_or_default()
}

pub fn set_inflation_rate(inflation_rate: u32) {
    set_key(INFLATION_RATE, inflation_rate);
}
pub fn inflation_rate() -> u32 {
    get_key(INFLATION_RATE).unwrap_or(103000) // 3.000% (indirect -> checks throgh LiquidityGuard)
}

pub fn set_liquidity_rate(liquidity_rate: u32) {
    set_key(LIQUIDITY_RATE, liquidity_rate);
}
pub fn liquidity_rate() -> u32 {
    get_key(LIQUIDITY_RATE).unwrap_or(100006) // 3.000% (indirect -> checks throgh LiquidityGuard)
}

pub fn set_stable_usd(stable_usd: Key) {
    set_key(STABLE_USD, stable_usd);
}
pub fn stable_usd() -> Key {
    get_key(STABLE_USD).unwrap_or_else(zero_address)
}

pub fn set_wcspr(wcspr: Key) {
    set_key(WCSPR, wcspr);
}
pub fn wcspr() -> Key {
    get_key(WCSPR).unwrap_or_else(zero_address)
}

pub fn set_scspr(scspr: Key) {
    set_key(SCSPR, scspr);
}
pub fn scspr() -> Key {
    get_key(SCSPR).unwrap_or_else(zero_address)
}

pub fn set_uniswap_router(uniswap_router: Key) {
    set_key(UNISWAP_ROUTER, uniswap_router);
}
pub fn uniswap_router() -> Key {
    get_key(UNISWAP_ROUTER).unwrap_or_else(zero_address)
}

pub fn set_uniswap_factory(uniswap_factory: Key) {
    set_key(UNISWAP_FACTORY, uniswap_factory);
}
pub fn uniswap_factory() -> Key {
    get_key(UNISWAP_FACTORY).unwrap_or_else(zero_address)
}

pub fn set_liquidity_guard(liquidity_guard: Key) {
    set_key(LIQUIDITY_GUARD, liquidity_guard);
}
pub fn liquidity_guard() -> Key {
    get_key(LIQUIDITY_GUARD).unwrap_or_else(zero_address)
}

pub fn set_is_liquidity_guard_active(is_liquidity_guard_active: bool) {
    set_key(IS_LIQUIDITY_GUARD_ACTIVE, is_liquidity_guard_active);
}
pub fn is_liquidity_guard_active() -> bool {
    get_key(IS_LIQUIDITY_GUARD_ACTIVE).unwrap_or_default()
}

pub fn set_uniswap_pair(uniswap_pair: Key) {
    set_key(UNISWAP_PAIR, uniswap_pair);
}
pub fn uniswap_pair() -> Key {
    get_key(UNISWAP_PAIR).unwrap_or_else(zero_address)
}

pub fn set_latest_stable_usd_equivalent(latest_stable_usd_equivalent: U256) {
    set_key(LATEST_STABLE_USD_EQUIVALENT, latest_stable_usd_equivalent);
}
pub fn latest_stable_usd_equivalent() -> U256 {
    get_key(LATEST_STABLE_USD_EQUIVALENT).unwrap_or_default()
}

pub fn path() -> Vec<Key> {
    vec![package_hash(), scspr(), wcspr(), stable_usd()]
}
