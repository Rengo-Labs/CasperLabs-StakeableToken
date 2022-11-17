use crate::data::*;
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::U256;
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use casperlabs_erc20::ERC20;
use common::{
    errors::Errors,
    events::{emit, Events},
};

pub trait GLOBAL<Storage: ContractStorage>: ContractContext<Storage> + ERC20<Storage> {
    fn init(&self) {
        set_globals({
            let mut globals = globals();
            globals.share_price = U256::from(100) * U256::from(10).pow(6.into()); // 100E15 <=> 100E6 == 0.1 WISE
            globals
        });
    }

    fn _increase_globals(&self, staked: U256, shares: U256, rshares: U256) {
        set_globals({
            let mut globals = globals();
            globals.total_staked = globals
                .total_staked
                .checked_add(staked)
                .unwrap_or_revert_with(Errors::AdditionOverflow1);
            globals.total_shares = globals
                .total_shares
                .checked_add(shares)
                .unwrap_or_revert_with(Errors::AdditionOverflow2);
            if rshares > 0.into() {
                globals.referral_shares = globals
                    .referral_shares
                    .checked_add(rshares)
                    .unwrap_or_revert_with(Errors::AdditionOverflow3);
            }
            globals
        });
        self._log_globals();
    }

    fn _decrease_globals(&self, staked: U256, shares: U256, rshares: U256) {
        set_globals({
            let mut globals = globals();
            globals.total_staked = if globals.total_staked > staked {
                globals.total_staked - staked
            } else {
                0.into()
            };
            globals.total_shares = if globals.total_shares > shares {
                globals.total_shares - shares
            } else {
                0.into()
            };
            if rshares > 0.into() {
                globals.referral_shares = if globals.referral_shares > rshares {
                    globals.referral_shares - rshares
                } else {
                    0.into()
                };
            }
            globals
        });
        self._log_globals();
    }

    fn _log_globals(&self) {
        emit(&Events::NewGlobals {
            total_shares: globals().total_shares,
            total_staked: globals().total_staked,
            share_rate: globals().share_price,
            referral_shares: globals().referral_shares,
            current_stakeable_day: globals().current_stakeable_day,
        });
    }
}
