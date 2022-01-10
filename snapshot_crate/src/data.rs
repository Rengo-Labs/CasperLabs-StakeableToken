use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    Key, U256,
};
use contract_utils::{get_key, set_key, Dict};
extern crate alloc;
use alloc::{string::ToString, vec::Vec};
use wise_token_utils::commons::key_names::*;
use wise_token_utils::snapshot::structs::*;

pub struct SnapshotsDict {
    dict: Dict,
}

impl SnapshotsDict {
    pub fn instance() -> SnapshotsDict {
        SnapshotsDict {
            dict: Dict::instance(SNAPSHOT_SNAPSHOTS_DICT),
        }
    }

    pub fn init() {
        Dict::init(SNAPSHOT_SNAPSHOTS_DICT)
    }

    pub fn get(&self, key: &U256) -> Vec<u8> {
        let mut result: Vec<u8> = self.dict.get(&key.to_string()).unwrap_or_default();
        if result.is_empty() {
            result = Snapshot::new().into_bytes().unwrap();
        }
        result
    }

    pub fn set(&self, key: &U256, value: Vec<u8>) {
        self.dict.set(&key.to_string(), value);
    }
}

pub struct LSnapshotsDict {
    dict: Dict,
}

impl LSnapshotsDict {
    pub fn instance() -> LSnapshotsDict {
        LSnapshotsDict {
            dict: Dict::instance(SNAPSHOT_LSNAPSHOTS_DICT),
        }
    }

    pub fn init() {
        Dict::init(SNAPSHOT_LSNAPSHOTS_DICT)
    }

    pub fn get(&self, key: &U256) -> Vec<u8> {
        let mut result: Vec<u8> = self.dict.get(&key.to_string()).unwrap_or_default();
        if result.is_empty() {
            result = LSnapShot::new().into_bytes().unwrap();
        }
        result
    }

    pub fn set(&self, key: &U256, value: Vec<u8>) {
        self.dict.set(&key.to_string(), value);
    }
}

pub struct RSnapshotsDict {
    dict: Dict,
}

impl RSnapshotsDict {
    pub fn instance() -> RSnapshotsDict {
        RSnapshotsDict {
            dict: Dict::instance(SNAPSHOT_RSNAPSHOTS_DICT),
        }
    }

    pub fn init() {
        Dict::init(SNAPSHOT_RSNAPSHOTS_DICT)
    }

    pub fn get(&self, key: &U256) -> Vec<u8> {
        let mut result: Vec<u8> = self.dict.get(&key.to_string()).unwrap_or_default();
        if result.is_empty() {
            result = RSnapshot::new().into_bytes().unwrap();
        }
        result
    }

    pub fn set(&self, key: &U256, value: Vec<u8>) {
        self.dict.set(&key.to_string(), value);
    }
}

pub fn self_hash() -> Key {
    get_key(SELF_CONTRACT_HASH).unwrap_or_revert()
}

pub fn sbnb_hash() -> Key {
    get_key(SBNB_CONTRACT_HASH).unwrap_or_revert()
}
pub fn set_sbnb_hash(hash: Key) {
    set_key(SBNB_CONTRACT_HASH, hash);
}

pub fn pair_hash() -> Key {
    get_key(PAIR_CONTRACT_HASH).unwrap_or_revert()
}
pub fn set_pair_hash(hash: Key) {
    set_key(PAIR_CONTRACT_HASH, hash);
}

pub fn guard_hash() -> Key {
    get_key(LIQUIDITY_GUARD_CONTRACT_HASH).unwrap_or_revert()
}

pub fn set_guard_hash(hash: Key) {
    set_key(LIQUIDITY_GUARD_CONTRACT_HASH, hash);
}
