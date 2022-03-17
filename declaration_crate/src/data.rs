use crate::alloc::{string::ToString, vec::Vec};
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{contracts::ContractPackageHash, Key, U256};
use contract_utils::{get_key, set_key, Dict};
use hex::encode;
use renvm_sig::keccak256;
use stakeable_token_utils::commons::key_names::*;

pub fn self_hash() -> Key {
    get_key(SELF_CONTRACT_HASH).unwrap_or_revert()
}
pub fn package_hash() -> ContractPackageHash {
    get_key(SELF_PACKAGE_HASH).unwrap_or_revert()
}

pub fn router_hash() -> Key {
    get_key(ROUTER_CONTRACT_HASH).unwrap_or_revert()
}
pub fn set_router_hash(hash: Key) {
    set_key(ROUTER_CONTRACT_HASH, hash);
}

pub fn factory_hash() -> Key {
    get_key(FACTORY_CONTRACT_HASH).unwrap_or_revert()
}
pub fn set_factory_hash(hash: Key) {
    set_key(FACTORY_CONTRACT_HASH, hash);
}

pub fn pair_hash() -> Key {
    get_key(PAIR_CONTRACT_HASH).unwrap_or_revert()
} // pair contract
pub fn set_pair_hash(hash: Key) {
    set_key(PAIR_CONTRACT_HASH, hash);
}

pub fn liquidity_guard_hash() -> Key {
    get_key(LIQUIDITY_GUARD_CONTRACT_HASH).unwrap_or_revert()
}
pub fn set_liquidity_guard_hash(hash: Key) {
    set_key(LIQUIDITY_GUARD_CONTRACT_HASH, hash);
}

pub fn lt_balance() -> U256 {
    get_key(DECLARATION_LTBALANCE).unwrap_or_default()
}
pub fn set_lt_balance(balance: U256) {
    set_key(DECLARATION_LTBALANCE, balance);
}

pub fn launch_time() -> U256 {
    get_key(DECLARATION_LAUNCH_TIME).unwrap_or_default()
}
pub fn set_launch_time(launch_time: U256) {
    set_key(DECLARATION_LAUNCH_TIME, launch_time);
}

pub fn inflation_rate() -> U256 {
    get_key(DECLARATION_INFLATION_RATE).unwrap_or_default()
}
pub fn set_inflation_rate(inflation_rate: U256) {
    set_key(DECLARATION_INFLATION_RATE, inflation_rate);
}

pub fn liquidity_rate() -> U256 {
    get_key(DECLARATION_LIQUIDITY_RATE).unwrap_or_default()
}
pub fn set_liquidity_rate(liquidity_rate: U256) {
    set_key(DECLARATION_LIQUIDITY_RATE, liquidity_rate);
}

pub fn liquidity_guard_status() -> bool {
    get_key(DECLARATION_LIQUIDITY_GUARD_STATUS).unwrap_or_default()
}
pub fn set_liquidity_guard_status(status: bool) {
    set_key(DECLARATION_LIQUIDITY_GUARD_STATUS, status);
}

pub fn wcspr() -> Key {
    get_key(WCSPR_CONTRACT_HASH).unwrap_or_revert()
}
pub fn set_wcspr(wcspr: Key) {
    set_key(WCSPR_CONTRACT_HASH, wcspr);
}

pub fn scspr() -> Key {
    get_key(SCSPR_CONTRACT_HASH).unwrap_or_revert()
}
pub fn set_scspr(scspr: Key) {
    set_key(SCSPR_CONTRACT_HASH, scspr);
}

pub fn stable_usd_equivalent() -> Key {
    get_key(STABLE_USD_EQUIVALENT_CONTRACT_HASH).unwrap_or_revert()
}
pub fn set_stable_usd_equivalent(stable_usd_equivalent: Key) {
    set_key(STABLE_USD_EQUIVALENT_CONTRACT_HASH, stable_usd_equivalent);
}

pub fn uniswap_pair() -> Key {
    get_key(UNISWAP_PAIR_CONTRACT_HASH).unwrap_or_revert()
}
pub fn set_uniswap_pair(pair: Key) {
    set_key(UNISWAP_PAIR_CONTRACT_HASH, pair);
}

// mappings
pub struct StakeCount {
    dict: Dict,
}

impl StakeCount {
    pub fn instance() -> StakeCount {
        StakeCount {
            dict: Dict::instance(DECLARATION_STAKE_COUNT_DICT),
        }
    }

    pub fn init() {
        Dict::init(DECLARATION_STAKE_COUNT_DICT)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get_by_key(owner).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set_by_key(owner, value);
    }
}

pub struct ReferralCount {
    dict: Dict,
}

impl ReferralCount {
    pub fn instance() -> ReferralCount {
        ReferralCount {
            dict: Dict::instance(DECLARATION_REFERRAL_COUNT_DICT),
        }
    }

    pub fn init() {
        Dict::init(DECLARATION_REFERRAL_COUNT_DICT)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get_by_key(owner).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set_by_key(owner, value);
    }
}

pub struct LiquidityStakeCount {
    dict: Dict,
}

impl LiquidityStakeCount {
    pub fn instance() -> LiquidityStakeCount {
        LiquidityStakeCount {
            dict: Dict::instance(DECLARATION_LIQUIDITY_STAKE_COUNT_DICT),
        }
    }

    pub fn init() {
        Dict::init(DECLARATION_LIQUIDITY_STAKE_COUNT_DICT)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get_by_key(owner).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set_by_key(owner, value);
    }
}

pub struct ScheduledToEnd {
    dict: Dict,
}

impl ScheduledToEnd {
    pub fn instance() -> ScheduledToEnd {
        ScheduledToEnd {
            dict: Dict::instance(DECLARATION_SCHEDULED_TO_END_DICT),
        }
    }

    pub fn init() {
        Dict::init(DECLARATION_SCHEDULED_TO_END_DICT)
    }

    pub fn get(&self, owner: &U256) -> U256 {
        self.dict.get(&owner.to_string()).unwrap_or_default()
    }

    pub fn set(&self, owner: &U256, value: U256) {
        self.dict.set(&owner.to_string(), value);
    }
}

pub struct ReferralSharesToEnd {
    dict: Dict,
}

impl ReferralSharesToEnd {
    pub fn instance() -> ReferralSharesToEnd {
        ReferralSharesToEnd {
            dict: Dict::instance(DECLARATION_REFERRAL_SHARES_TO_END_DICT),
        }
    }

    pub fn init() {
        Dict::init(DECLARATION_REFERRAL_SHARES_TO_END_DICT)
    }

    pub fn get(&self, owner: &U256) -> U256 {
        self.dict.get(&owner.to_string()).unwrap_or_default()
    }

    pub fn set(&self, owner: &U256, value: U256) {
        self.dict.set(&owner.to_string(), value);
    }
}

pub struct TotalPenalties {
    dict: Dict,
}

impl TotalPenalties {
    pub fn instance() -> TotalPenalties {
        TotalPenalties {
            dict: Dict::instance(DECLARATION_TOTAL_PENALTIES_DICT),
        }
    }

    pub fn init() {
        Dict::init(DECLARATION_TOTAL_PENALTIES_DICT)
    }

    pub fn get(&self, owner: &U256) -> U256 {
        self.dict.get(&owner.to_string()).unwrap_or_default()
    }

    pub fn set(&self, owner: &U256, value: U256) {
        self.dict.set(&owner.to_string(), value);
    }
}

pub struct CriticalMass {
    dict: Dict,
}

impl CriticalMass {
    pub fn instance() -> CriticalMass {
        CriticalMass {
            dict: Dict::instance(DECLARATION_CRITICAL_MASS_DICT),
        }
    }

    pub fn init() {
        Dict::init(DECLARATION_CRITICAL_MASS_DICT)
    }

    // key is the string representation of the Key type
    pub fn get(&self, key: &str) -> Vec<u8> {
        let key = encode(keccak256(key.to_string().as_bytes()));
        let result: Vec<u8> = self.dict.get(&key).unwrap_or_default();
        result
    }
    // value is the json string representation of the 'CriticalMass' structure
    // key should the string representation of the Key type
    pub fn set(&self, key: &str, value: Vec<u8>) {
        let key = encode(keccak256(key.to_string().as_bytes()));
        self.dict.set(&key, value);
    }
}

// both the id(vec<u16>) and struct should be passed in as string
pub struct Scrapes {
    dict: Dict,
}

impl Scrapes {
    pub fn instance() -> Scrapes {
        Scrapes {
            dict: Dict::instance(DECLARATION_SCRAPES_DICT),
        }
    }

    pub fn init() {
        Dict::init(DECLARATION_SCRAPES_DICT)
    }

    pub fn get(&self, key: &str) -> U256 {
        let result: U256 = self.dict.get(&key).unwrap_or_default();
        result
    }

    // value is the json string representation of the 'Scrapes' structure
    // key should the string representation of the Key and Vec<u16> concatinated
    pub fn set(&self, key: &str, value: U256) {
        self.dict.set(&key, value);
    }
}

pub struct Stakes {
    dict: Dict,
}

impl Stakes {
    pub fn instance() -> Stakes {
        Stakes {
            dict: Dict::instance(DECLARATION_STAKES_DICT),
        }
    }

    pub fn init() {
        Dict::init(DECLARATION_STAKES_DICT)
    }

    // key is the string representation of the Key and ID concatinated
    pub fn get(&self, key: &str) -> Vec<u8> {
        let result: Vec<u8> = self.dict.get(&key).unwrap_or_default();
        result
    }

    // value is the json string representation of the 'Stakes' structure
    // key should the string representation of the Key and Vec<u16> concatinated
    pub fn set(&self, key: &str, value: Vec<u8>) {
        self.dict.set(&key, value);
    }
}

pub struct ReferrerLink {
    dict: Dict,
}

impl ReferrerLink {
    pub fn instance() -> ReferrerLink {
        ReferrerLink {
            dict: Dict::instance(DECLARATION_REFERRER_LINK_DICT),
        }
    }

    pub fn init() {
        Dict::init(DECLARATION_REFERRER_LINK_DICT)
    }

    // key is the string representation of the Key and ID concatinated
    pub fn get(&self, key: &str) -> Vec<u8> {
        let result: Vec<u8> = self.dict.get(&key).unwrap_or_default();
        result
    }

    // value is the json string representation of the 'Stakes' structure
    // key should the string representation of the Key and Vec<u16> concatinated
    pub fn set(&self, key: &str, value: Vec<u8>) {
        self.dict.set(&key, value);
    }
}

pub struct LiquidityStakes {
    dict: Dict,
}

impl LiquidityStakes {
    pub fn instance() -> LiquidityStakes {
        LiquidityStakes {
            dict: Dict::instance(DECLARATION_LIQUIDITY_STAKES_DICT),
        }
    }

    pub fn init() {
        Dict::init(DECLARATION_LIQUIDITY_STAKES_DICT)
    }

    // key is the string representation of the Key and ID concatinated
    pub fn get(&self, key: &str) -> Vec<u8> {
        let result: Vec<u8> = self.dict.get(&key).unwrap_or_default();
        result
    }

    // value is the json string representation of the 'Stakes' structure
    // key is the string representation of the Key and ID concatinated
    pub fn set(&self, key: &str, value: Vec<u8>) {
        self.dict.set(&key, value);
    }
}
