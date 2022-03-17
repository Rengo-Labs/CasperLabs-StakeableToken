extern crate alloc;
use alloc::string::String;

use casper_types::U256;
use contract_utils::{ContractContext, ContractStorage};

// use crate::GLOBALS_{self, GlobalsStruct};
use crate::data::GlobalsStruct;
use stakeable_token_utils::{commons::key_names::*, events::*};

pub trait Globals<Storage: ContractStorage>: ContractContext<Storage> {
    // Will be called by constructor
    fn init(&mut self) {
        let mut number: U256 = U256::from(10).pow(15.into()); // 10 ^ 15
        number = U256::from(100) * number; // 100 * (10 ^ 15)  =  100E15;

        GlobalsStruct::init();
        GlobalsStruct::instance().set(GLOBALS_SHARE_PRICE, number);
    }

    fn increase_globals(&self, _staked: U256, _shares: U256, _rshares: U256) {
        let globals: GlobalsStruct = GlobalsStruct::instance();

        globals.set(
            GLOBALS_TOTAL_STAKED,
            globals
                .get(GLOBALS_TOTAL_STAKED)
                .checked_add(_staked)
                .unwrap(),
        ); // globals.totalStaked = globals.totalStaked.add(_staked)
        globals.set(
            GLOBALS_TOTAL_SHARES,
            globals
                .get(GLOBALS_TOTAL_SHARES)
                .checked_add(_shares)
                .unwrap(),
        ); // globals.total_shares = globals.totalShares.add(_shares)

        if _rshares > 0.into() {
            globals.set(
                GLOBALS_REFERRAL_SHARES,
                globals
                    .get(GLOBALS_REFERRAL_SHARES)
                    .checked_add(_rshares)
                    .unwrap(),
            );
        }

        Self::_log_globals(self);
    }

    fn decrease_globals(&self, _staked: U256, _shares: U256, _rshares: U256) {
        let globals: GlobalsStruct = GlobalsStruct::instance();

        globals.set(
            GLOBALS_TOTAL_STAKED,
            if globals.get(GLOBALS_TOTAL_STAKED) > _staked {
                globals.get(GLOBALS_TOTAL_STAKED) - _staked
            } else {
                0.into()
            },
        );
        globals.set(
            GLOBALS_TOTAL_SHARES,
            if globals.get(GLOBALS_TOTAL_SHARES) > _shares {
                globals.get(GLOBALS_TOTAL_SHARES) - _shares
            } else {
                0.into()
            },
        );

        if _rshares > 0.into() {
            globals.set(
                GLOBALS_REFERRAL_SHARES,
                if globals.get(GLOBALS_REFERRAL_SHARES) > _rshares {
                    globals.get(GLOBALS_REFERRAL_SHARES) - _rshares
                } else {
                    0.into()
                },
            );
        }

        Self::_log_globals(self);
    }

    fn get_globals(&self, field: String) -> U256 {
        let globals: GlobalsStruct = GlobalsStruct::instance();
        globals.get(&field)
    }

    fn set_globals(&self, field: String, value: U256) {
        let globals: GlobalsStruct = GlobalsStruct::instance();
        globals.set(&field, value);
    }

    fn _log_globals(&self) {
        let globals: GlobalsStruct = GlobalsStruct::instance();
        emit(&StakeableEvents::NewGlobals {
            total_shares: globals.get(GLOBALS_TOTAL_SHARES),
            total_staked: globals.get(GLOBALS_TOTAL_STAKED),
            share_rate: globals.get(GLOBALS_SHARE_PRICE),
            referral_shares: globals.get(GLOBALS_REFERRAL_SHARES),
            current_stakeable_day: globals.get(GLOBALS_CURRENT_WISE_DAY).as_u64(),
        });
    }
}
