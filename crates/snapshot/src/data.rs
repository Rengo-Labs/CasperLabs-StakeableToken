use casper_types::U256;
use casper_types_derive::{CLTyped, FromBytes, ToBytes};
use casperlabs_contract_utils::Dict;
extern crate alloc;
use alloc::{string::ToString, vec::Vec};
use helper::keys::{LSNAPSHOTS_DICT, RSNAPSHOTS_DICT, SNAPSHOTS_DICT};

#[derive(Default, Debug, Clone, Copy, CLTyped, ToBytes, FromBytes)]
pub struct SnapShot {
    pub total_shares: U256,
    pub inflation_amount: U256,
    pub scheduled_to_end: U256,
}

#[derive(Default, Debug, Clone, Copy, CLTyped, ToBytes, FromBytes)]
pub struct RSnapShot {
    pub total_shares: U256,
    pub inflation_amount: U256,
    pub scheduled_to_end: U256,
}

#[derive(Default, Debug, Clone, Copy, CLTyped, ToBytes, FromBytes)]
pub struct LSnapShot {
    pub total_shares: U256,
    pub inflation_amount: U256,
}

pub struct Snapshots {
    dict: Dict,
}
impl Snapshots {
    pub fn instance() -> Snapshots {
        Snapshots {
            dict: Dict::instance(SNAPSHOTS_DICT),
        }
    }
    pub fn init() {
        Dict::init(SNAPSHOTS_DICT)
    }
    pub fn get(&self, key: &U256) -> SnapShot {
        self.dict.get(&key.to_string()).unwrap_or_default()
    }
    pub fn set(&self, key: &U256, value: SnapShot) {
        self.dict.set(&key.to_string(), value);
    }
}

pub struct RSnapshots {
    dict: Dict,
}
impl RSnapshots {
    pub fn instance() -> RSnapshots {
        RSnapshots {
            dict: Dict::instance(RSNAPSHOTS_DICT),
        }
    }
    pub fn init() {
        Dict::init(RSNAPSHOTS_DICT)
    }
    pub fn get(&self, key: &U256) -> RSnapShot {
        self.dict.get(&key.to_string()).unwrap_or_default()
    }
    pub fn set(&self, key: &U256, value: RSnapShot) {
        self.dict.set(&key.to_string(), value);
    }
}

pub struct LSnapshots {
    dict: Dict,
}
impl LSnapshots {
    pub fn instance() -> LSnapshots {
        LSnapshots {
            dict: Dict::instance(LSNAPSHOTS_DICT),
        }
    }
    pub fn init() {
        Dict::init(LSNAPSHOTS_DICT)
    }
    pub fn get(&self, key: &U256) -> LSnapShot {
        self.dict.get(&key.to_string()).unwrap_or_default()
    }
    pub fn set(&self, key: &U256, value: LSnapShot) {
        self.dict.set(&key.to_string(), value);
    }
}
