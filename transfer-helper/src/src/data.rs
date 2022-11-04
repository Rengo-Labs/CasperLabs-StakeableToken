use casper_types::Key;
use casperlabs_contract_utils::{get_key, set_key};
use common::{functions::zero_address, keys::TRANSFER_INVOKER};

pub fn set_transfer_invoker(transfer_invoker: Key) {
    set_key(TRANSFER_INVOKER, transfer_invoker);
}
pub fn transfer_invoker() -> Key {
    get_key(TRANSFER_INVOKER).unwrap_or_else(zero_address)
}
