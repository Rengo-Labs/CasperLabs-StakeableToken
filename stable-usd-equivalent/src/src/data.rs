use casper_types::Key;
use casperlabs_contract_utils::{get_key, set_key};
use common::{functions::zero_address, keys::STAKEABLE};

pub fn set_stakeable(stakeable: Key) {
    set_key(STAKEABLE, stakeable);
}
pub fn stakeable() -> Key {
    get_key(STAKEABLE).unwrap_or_else(zero_address)
}
