extern crate alloc;
use alloc::vec::Vec;
use casper_types::U256;
use casper_types_derive::{CLTyped, FromBytes, ToBytes};
use casperlabs_contract_utils::{get_key, set_key};
use common::keys::GLOBALS;

#[derive(Default, Debug, Clone, Copy, CLTyped, ToBytes, FromBytes)]
pub struct Globals {
    pub total_staked: U256,
    pub total_shares: U256,
    pub share_price: U256,
    pub current_stakeable_day: U256,
    pub referral_shares: U256,
    pub liquidity_shares: U256,
}

pub fn set_globals(globals: Globals) {
    set_key(GLOBALS, globals);
}
pub fn globals() -> Globals {
    get_key(GLOBALS).unwrap_or_default()
}
