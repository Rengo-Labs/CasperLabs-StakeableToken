use casper_types::{Key, URef};
use casperlabs_contract_utils::{get_key, set_key};
use common::{
    functions::zero_address,
    keys::{LIQUIDITY_TRANSFORMER, LIQUIDITY_TRANSFORMER_PURSE, TRANSFORMER_GATE_KEEPER},
};

pub fn set_liquidity_transformer(liquidity_transformer: Key, transformer_purse: URef) {
    set_key(LIQUIDITY_TRANSFORMER, liquidity_transformer);
    set_key(LIQUIDITY_TRANSFORMER_PURSE, transformer_purse);
}
pub fn liquidity_transformer() -> (Key, URef) {
    (
        get_key(LIQUIDITY_TRANSFORMER).unwrap_or_else(zero_address),
        get_key(LIQUIDITY_TRANSFORMER_PURSE).unwrap_or_default(),
    )
}

pub fn set_transformer_gate_keeper(transformer_gate_keeper: Key) {
    set_key(TRANSFORMER_GATE_KEEPER, transformer_gate_keeper);
}
pub fn transformer_gate_keeper() -> Key {
    get_key(TRANSFORMER_GATE_KEEPER).unwrap_or_else(zero_address)
}
