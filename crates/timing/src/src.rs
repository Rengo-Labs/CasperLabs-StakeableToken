use casper_types::U256;
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use declaration::{
    data::{launch_time, MILLI_SECONDS_IN_DAY},
    functions::block_timestamp,
    src::DECLARATION,
};

pub trait TIMING<Storage: ContractStorage>:
    ContractContext<Storage> + DECLARATION<Storage>
{
    fn init(&self) {
        DECLARATION::init(self);
    }

    fn current_stakeable_day(&self) -> u64 {
        if self._get_now() >= launch_time() {
            self._current_stakeable_day()
        } else {
            0
        }
    }

    fn _current_stakeable_day(&self) -> u64 {
        self._stakeable_day_from_stamp(self._get_now())
    }

    fn _next_stakeable_day(&self) -> u64 {
        self._current_stakeable_day() + 1
    }

    fn _previous_stakeable_day(&self) -> u64 {
        self._current_stakeable_day() - 1
    }

    fn _stakeable_day_from_stamp(&self, timestamp: U256) -> u64 {
        ((timestamp - launch_time()) / U256::from(MILLI_SECONDS_IN_DAY)).as_u64()
    }

    fn _get_now(&self) -> U256 {
        block_timestamp().into()
    }
}
