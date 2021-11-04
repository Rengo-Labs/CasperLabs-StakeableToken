use crate::alloc::{string::String, string::ToString, vec::Vec};

use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{contracts::{ContractPackageHash}, Key, U256};
use contract_utils::{get_key, set_key, Dict};

use crate::config;

pub const SELF_HASH: &str = "self_hash";
pub const PACKAGE_HASH: &str = "package_hash";
pub const ROUTER_HASH: &str = "router_hash";
pub const FACTORY_HASH: &str = "factory_hash";
pub const PAIR_HASH: &str = "pair_hash";
pub const LIQUIDITY_GUARD_HASH: &str = "liquidity_guard_hash";
pub const LTBALANCE: &str = "lt_balance";
pub const LAUNCH_TIME: &str = "launch_time";
pub const SYNTHETIC_BNB: &str = "synthetic_bnb";
pub const INFLATION_RATE: &str = "inflation_rate";
pub const LIQUIDITY_RATE: &str = "liquidity_rate";
pub const LIQUIDITY_GUARD_STATUS: &str = "liquidity_guard_status";
pub const WBNB: &str = "wbnb";
pub const SBNB: &str = "sbnb";


// mappings
pub const STAKE_COUNT: &str = "stake_count";
pub const REFERRAL_COUNT: &str = "referral_count";
pub const LIQUIDITY_STAKE_COUNT: &str = "liquidity_stake_count";
pub const SCHEDULED_TO_END: &str = "scheduled_to_end";
pub const REFERRAL_SHARES_TO_END: &str = "referral_shares_to_end";
pub const TOTAL_PENALTIES: &str = "total_penalties";
pub const CRITICAL_MASS: &str = "critical_mass";
pub const SCRAPES: &str = "scrapes";
pub const STAKES: &str = "stakes";
pub const REFERRER_LINK: &str = "referrer_link";
pub const LIQUIDITY_STAKES: &str = "liquidity_stakes";



pub fn self_hash() -> Key { get_key(SELF_HASH).unwrap_or_revert()}
pub fn set_self_hash(hash: Key) { set_key(SELF_HASH, hash);}

pub fn package_hash() -> ContractPackageHash { get_key(PACKAGE_HASH).unwrap_or_revert()}
pub fn set_package_hash(hash: ContractPackageHash) { set_key(PACKAGE_HASH, hash);}

pub fn router_hash() -> Key { get_key(ROUTER_HASH).unwrap_or_revert()}
pub fn set_router_hash(hash: Key) { set_key(ROUTER_HASH, hash);}

pub fn factory_hash() -> Key { get_key(FACTORY_HASH).unwrap_or_revert()}
pub fn set_factory_hash(hash: Key) { set_key(FACTORY_HASH, hash);}

pub fn pair_hash() -> Key {  get_key(PAIR_HASH).unwrap_or_revert() }
pub fn set_pair_hash(hash: Key) { set_key(PAIR_HASH, hash); }

pub fn liquidity_guard_hash() -> Key { get_key(LIQUIDITY_GUARD_HASH).unwrap_or_revert()}
pub fn set_liquidity_guard_hash(hash: Key) { set_key(LIQUIDITY_GUARD_HASH, hash);}

pub fn synthetic_bnb_hash() -> Key {  get_key(SYNTHETIC_BNB).unwrap_or_revert() }
pub fn set_synthetic_bnb_hash(hash: Key) { set_key(SYNTHETIC_BNB, hash); }

pub fn lt_balance() -> U256 {get_key(LTBALANCE).unwrap_or_default()}
pub fn set_lt_balance(balance: U256) { set_key(LTBALANCE, balance);}

pub fn launch_time() -> U256 {get_key(LAUNCH_TIME).unwrap_or_default()}
pub fn set_launch_time(launch_time: U256) { set_key(LAUNCH_TIME, launch_time);}

pub fn inflation_rate() -> U256 {get_key(INFLATION_RATE).unwrap_or_default()}
pub fn set_inflation_rate(inflation_rate: U256) { set_key(INFLATION_RATE, inflation_rate);}

pub fn liquidity_rate() -> U256 {get_key(LIQUIDITY_RATE).unwrap_or_default()}
pub fn set_liquidity_rate(liquidity_rate: U256) { set_key(LIQUIDITY_RATE, liquidity_rate);}

pub fn liquidity_guard_status() -> bool {get_key(LIQUIDITY_GUARD_STATUS).unwrap_or_default()}
pub fn set_liquidity_guard_status(status: bool) { set_key(LIQUIDITY_GUARD_STATUS, status);}

pub fn wbnb() -> Key {get_key(WBNB).unwrap_or_revert()}
pub fn set_wbnb(wbnb: Key) { set_key(WBNB, wbnb);}

pub fn sbnb() -> Key {get_key(SBNB).unwrap_or_revert()}
pub fn set_sbnb(sbnb: Key) { set_key(SBNB, sbnb);}


// mappings
pub struct StakeCount {
    dict: Dict,
}

impl StakeCount {
    pub fn instance() -> StakeCount {
        StakeCount {
            dict: Dict::instance(STAKE_COUNT),
        }
    }

    pub fn init() {
        Dict::init(STAKE_COUNT)
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
            dict: Dict::instance(REFERRAL_COUNT),
        }
    }

    pub fn init() {
        Dict::init(REFERRAL_COUNT)
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
            dict: Dict::instance(LIQUIDITY_STAKE_COUNT),
        }
    }

    pub fn init() {
        Dict::init(LIQUIDITY_STAKE_COUNT)
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
            dict: Dict::instance(SCHEDULED_TO_END),
        }
    }

    pub fn init() {
        Dict::init(SCHEDULED_TO_END)
    }

    pub fn get(&self, owner: &U256) ->U256 {
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
            dict: Dict::instance(REFERRAL_SHARES_TO_END),
        }
    }

    pub fn init() {
        Dict::init(REFERRAL_SHARES_TO_END)
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
            dict: Dict::instance(TOTAL_PENALTIES),
        }
    }

    pub fn init() {
        Dict::init(TOTAL_PENALTIES)
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
            dict: Dict::instance(CRITICAL_MASS),
        }
    }

    pub fn init() {
        Dict::init(CRITICAL_MASS)
    }

    // key is the string representation of the Key type
    pub fn get(&self, key: &str) -> String
    {
        let result: String = self.dict.get(&key).unwrap_or_default();
        result
    }
    
    // value is the json string representation of the 'CriticalMass' structure
    // key should the string representation of the Key type
    pub fn set(&self, key : &str, value: String ) 
    {
        self.dict.set(&key, value);
    }

    /*
    pub fn get(&self, key: &str) -> String {
        let result: U256 = self.dict.get(&key).unwrap_or_default();
        result

        // let json_string = serde_json::to_string(&value).unwrap();                    // convert structure to json string and save
        //let ret: config::CriticalMass = serde_json::from_str(&json_string).unwrap();
        //let json_string = serde_json::to_string(&value).unwrap();                             // convert structure to json string and save
        //ret
    }

    // pass in the key and the json representation of the struct here
    pub fn set(&self, owner: &Key, value: &str) {
        self.dict.set_by_key(owner, value);
    }
    */
}


// both the id(vec<u16>) and struct should be passed in as string
pub struct Scrapes {
    dict: Dict,
}

impl Scrapes {
    pub fn instance() -> Scrapes {
        Scrapes {
            dict: Dict::instance(SCRAPES),
        }
    }

    pub fn init() {
        Dict::init(SCRAPES)
    }

    pub fn get(&self, key: &str) -> U256
    {
        let result: U256 = self.dict.get(&key).unwrap_or_default();
        result
    }

    // value is the json string representation of the 'Scrapes' structure
    // key should the string representation of the Key and Vec<u16> concatinated
    pub fn set(&self, key : &str, value: U256 ) 
    {
        self.dict.set(&key, value);
    }
}


pub struct Stakes {
    dict: Dict,
}

impl Stakes {
    pub fn instance() -> Stakes {
        Stakes {
            dict: Dict::instance(STAKES),
        }
    }

    pub fn init() {
        Dict::init(STAKES)
    }

    // key is the string representation of the Key and ID concatinated
    pub fn get(&self, key: &str) -> String
    {
        let result: String = self.dict.get(&key).unwrap_or_default();
        result
    }

    // value is the json string representation of the 'Stakes' structure
    // key should the string representation of the Key and Vec<u16> concatinated
    pub fn set(&self, key : &str, value: String ) 
    {
        self.dict.set(&key, value);
    }
}


pub struct ReferrerLink {
    dict: Dict,
}

impl ReferrerLink {
    pub fn instance() -> ReferrerLink {
        ReferrerLink {
            dict: Dict::instance(REFERRER_LINK),
        }
    }

    pub fn init() {
        Dict::init(REFERRER_LINK)
    }

    // key is the string representation of the Key and ID concatinated
    pub fn get(&self, key: &str) -> String
    {
        let result: String = self.dict.get(&key).unwrap_or_default();
        result
    }

    // value is the json string representation of the 'Stakes' structure
    // key should the string representation of the Key and Vec<u16> concatinated
    pub fn set(&self, key : &str, value: String ) 
    {
        self.dict.set(&key, value);
    }
}


pub struct LiquidityStakes {
    dict: Dict,
}

impl LiquidityStakes {
    pub fn instance() -> LiquidityStakes {
        LiquidityStakes {
            dict: Dict::instance(LIQUIDITY_STAKES),
        }
    }

    pub fn init() {
        Dict::init(LIQUIDITY_STAKES)
    }

    // key is the string representation of the Key and ID concatinated
    pub fn get(&self, key: &str) -> String
    {
        let result: String = self.dict.get(&key).unwrap_or_default();
        result
    }

    // value is the json string representation of the 'Stakes' structure
    // key is the string representation of the Key and ID concatinated
    pub fn set(&self, key : &str, value: String ) 
    {
        self.dict.set(&key, value);
    }
}