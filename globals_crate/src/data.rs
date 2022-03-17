use casper_types::{U256};
use contract_utils::{Dict};

use stakeable_token_utils::commons::key_names::*;


pub struct GlobalsStruct {
    globals: Dict,
}

impl GlobalsStruct 
{
    pub fn instance() -> GlobalsStruct 
    {
        GlobalsStruct 
        {
            globals: Dict::instance(GLOBALS_GLOBALS_STRUCT),
        }
    }

    pub fn init() {
        Dict::init(GLOBALS_GLOBALS_STRUCT)
    }

    pub fn get(&self, key: &str) -> U256 {
        self.globals.get(key).unwrap_or_default()
    }

    pub fn set(&self, key: &str, value: U256) {
        self.globals.set(key, value);
    }
}