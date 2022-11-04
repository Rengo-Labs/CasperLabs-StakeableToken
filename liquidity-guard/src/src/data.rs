use casper_types::U256;
use casperlabs_contract_utils::{get_key, set_key, Dict};
use common::keys::{INFLATION_LN, IS_READY};

pub struct InflationLN {
    dict: Dict,
}
impl InflationLN {
    pub fn instance() -> InflationLN {
        InflationLN {
            dict: Dict::instance(INFLATION_LN),
        }
    }
    pub fn init() {
        Dict::init(INFLATION_LN)
    }
    pub fn get(&self, key: &u32) -> U256 {
        self.dict.get(key.to_string().as_str()).unwrap_or_default()
    }
    pub fn set(&self, key: &u32, value: U256) {
        self.dict.set(key.to_string().as_str(), value);
    }
}

pub fn set_is_ready(is_ready: bool) {
    set_key(IS_READY, is_ready);
}
pub fn is_ready() -> bool {
    get_key(IS_READY).unwrap_or_default()
}
