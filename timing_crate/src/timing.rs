extern crate alloc;

use crate::data;
use casper_contract::contract_api::runtime;
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{ApiError, U256};
use contract_utils::{ContractContext, ContractStorage};
use declaration_crate::Declaration;
use globals_crate::Globals;
use stakeable_token_utils::{error_codes::ErrorCodes, timing::parameters::*};

pub trait Timing<Storage: ContractStorage>:
    ContractContext<Storage> + Declaration<Storage> + Globals<Storage>
{
    // Will be called by constructor
    fn init(&mut self) {}

    fn current_stakeable_day(&self) -> u64 {
        let launch_time = data::launch_time();
        if Self::_get_now() >= launch_time {
            Self::_current_stakeable_day(self)
        } else {
            0
        }
    }

    fn current_stakeable_day_only(&self) -> u64 {
        Self::_current_stakeable_day(self)
    }

    fn previous_stakeable_day(&self) -> u64 {
        Self::_previous_stakeable_day(self)
    }

    fn next_stakeable_day(&self) -> u64 {
        Self::_next_stakeable_day(self)
    }

    // Helper methods

    fn _current_stakeable_day(&self) -> u64 {
        Self::_stakeable_day_from_stamp(Self::_get_now())
    }

    fn _next_stakeable_day(&self) -> u64 {
        Self::_current_stakeable_day(self) + 1
    }

    fn _previous_stakeable_day(&self) -> u64 {
        Self::_current_stakeable_day(self) - 1
    }

    fn _stakeable_day_from_stamp(_timestamp: U256) -> u64 {
        let launch_time = data::launch_time();
        let seconds_in_day = U256::from(SECONDS_IN_DAY);

        // ((_timestamp - launch_time) / seconds_in_day).as_u64()
        (_timestamp
            .checked_sub(launch_time)
            .ok_or(ApiError::User(ErrorCodes::Underflow as u16))
            .unwrap_or_revert()
            / seconds_in_day)
            .as_u64()
    }

    fn _get_now() -> U256 {
        let block_time: u64 = runtime::get_blocktime().into(); // returns milliseconds since the Unix epoch
        U256::from(block_time) / U256::from(1000) // convert milliseconds to seconds
    }
}
