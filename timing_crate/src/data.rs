use casper_types::U256;
use contract_utils::{get_key};

use stakeable_token_utils::commons::key_names::DECLARATION_LAUNCH_TIME;

pub fn launch_time() -> U256 {
    get_key(DECLARATION_LAUNCH_TIME).unwrap_or_default()
}