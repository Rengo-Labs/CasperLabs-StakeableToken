use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{contracts::{ContractPackageHash}, Key, U256};
use contract_utils::{get_key, set_key, Dict};

pub const SELF_HASH: &str = "self_hash";
pub const PACKAGE_HASH: &str = "package_hash";
pub const GLOBALS_STRUCT: &str = "globals_struct";

// Keys for global struct
pub const TOTAL_STAKED: &str = "total_staked";
pub const TOTAL_SHARES: &str = "total_shares";
pub const SHARE_PRICE: &str = "share_price";
pub const CURRENT_WISE_DAY: &str = "current_wise_day";
pub const REFERRAL_SHARES: &str = "referral_shares";
pub const LIQUIDITY_SHARES: &str = "liquidity_shares";


pub struct GlobalsStruct {
    globals: Dict,
}

impl GlobalsStruct 
{
    pub fn instance() -> GlobalsStruct 
    {
        GlobalsStruct 
        {
            globals: Dict::instance(GLOBALS_STRUCT),
        }
    }

    pub fn init() {
        Dict::init(GLOBALS_STRUCT)
    }

    pub fn get(&self, key: &str) -> U256 {
        self.globals.get(key).unwrap_or_default()
    }

    pub fn set(&self, key: &str, value: U256) {
        self.globals.set(key, value);
    }
}


pub fn self_hash() -> Key { get_key(SELF_HASH).unwrap_or_revert()}
pub fn set_self_hash(contract_hash: Key) { set_key(SELF_HASH, contract_hash);}

pub fn package_hash() -> ContractPackageHash { get_key(PACKAGE_HASH).unwrap_or_revert()}
pub fn set_package_hash(package_hash: ContractPackageHash) { set_key(PACKAGE_HASH, package_hash);}