use crate::data::*;
use casper_contract::contract_api::runtime;
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{runtime_args, Key, RuntimeArgs, U128, U256};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use helper::{
    errors::Errors,
    events::{emit, Events},
    functions::key_to_hash,
    src::HELPER,
    *,
};

pub trait SNAPSHOT<Storage: ContractStorage>: ContractContext<Storage> + HELPER<Storage> {
    fn init(&self) {
        HELPER::init(self);
        Snapshots::init();
        RSnapshots::init();
        LSnapshots::init();
    }

    fn snapshot_trigger(&mut self) {
        self._daily_snapshot_point(self._current_stakeable_day());
    }

    fn liquidity_guard_trigger(&mut self) {
        let (reserve_a, reserve_b, block_timestamp_last): (U128, U128, u64) =
            runtime::call_versioned_contract(
                key_to_hash(uniswap_pair(), Errors::InvalidHash5),
                None,
                "get_reserves",
                runtime_args! {},
            );
        emit(&Events::UniswapReserves {
            reserve_a,
            reserve_b,
            block_timestamp_last,
        });
        let token1: Key = runtime::call_versioned_contract(
            key_to_hash(uniswap_pair(), Errors::InvalidHash6),
            None,
            "token1",
            runtime_args! {},
        );
        let on_pancake: U256 = if token1 == scspr() {
            reserve_a.as_u128().into()
        } else {
            reserve_b.as_u128().into()
        };
        let ratio: U256 = if self.total_supply() == 0.into() {
            0.into()
        } else {
            on_pancake
                .checked_mul(200.into())
                .unwrap_or_revert_with(Errors::MultiplicationOverflow1)
                .checked_div(self.total_supply())
                .unwrap_or_revert_with(Errors::DivisionByZero2)
        };
        if ratio < 40.into() && !is_liquidity_guard_active() {
            self._enable_liquidity_guard();
        }
        if ratio > 60.into() && is_liquidity_guard_active() {
            self._disable_liquidity_guard();
        }
        emit(&Events::LiquidityGuardStatus {
            is_active: is_liquidity_guard_active(),
        });
    }

    fn _enable_liquidity_guard(&self) {
        set_is_liquidity_guard_active(true);
    }

    fn _disable_liquidity_guard(&self) {
        set_is_liquidity_guard_active(false);
    }

    fn manual_daily_snapshot(&mut self) {
        self._daily_snapshot_point(self._current_stakeable_day());
    }

    fn manual_daily_snapshot_point(&mut self, update_day: u64) {
        if update_day == 0 || update_day >= self._current_stakeable_day() {
            runtime::revert(Errors::SnapshotDayDoesNotExistYet);
        }
        if U256::from(update_day) <= globals().current_stakeable_day {
            runtime::revert(Errors::SnapshotAlreadyTakenForThatDay);
        }
        self._daily_snapshot_point(update_day);
    }

    /// @notice internal function that offloads global values to daily snapshots updates globals.currentStakeableDay
    fn _daily_snapshot_point(&mut self, update_day: u64) {
        self.liquidity_guard_trigger();
        let mut scheduled_to_end_today: U256;
        let total_staked_today: U256 = globals().total_staked;
        let mut day = globals().current_stakeable_day;
        while day < update_day.into() {
            // ------------------------------------
            // prepare snapshot for regular shares
            // reusing scheduledToEndToday variable
            scheduled_to_end_today = ScheduledToEnd::instance().get(&day.into())
                + if day.checked_sub(1.into()).is_some() {
                    Snapshots::instance()
                        .get(&(day - 1).into())
                        .scheduled_to_end
                } else {
                    0.into()
                };
            let mut snapshot: SnapShot = Snapshots::instance().get(&day.into());
            snapshot.scheduled_to_end = scheduled_to_end_today;
            snapshot.total_shares = if globals().total_shares > scheduled_to_end_today {
                globals().total_shares - scheduled_to_end_today
            } else {
                0.into()
            };
            let total_supply = self.total_supply();
            snapshot.inflation_amount = snapshot
                .total_shares
                .checked_mul(PRECISION_RATE.into())
                .unwrap_or_revert_with(Errors::MultiplicationOverflow2)
                .checked_div(self._inflation_amount(
                    total_staked_today,
                    total_supply,
                    TotalPenalties::instance().get(&day.into()),
                    runtime::call_versioned_contract(
                        key_to_hash(liquidity_guard(), Errors::InvalidHash7),
                        None,
                        "get_inflation",
                        runtime_args! {
                            "amount" => inflation_rate()
                        },
                    ),
                ))
                .unwrap_or_revert_with(Errors::DivisionByZero1);
            // store regular snapshot
            Snapshots::instance().set(&day.into(), snapshot);
            // ------------------------------------
            // prepare snapshot for referrer shares
            // reusing scheduledToEndToday variable
            scheduled_to_end_today = ReferralSharesToEnd::instance().get(&day.into())
                + if day.checked_sub(1.into()).is_some() {
                    RSnapshots::instance()
                        .get(&(day - 1).into())
                        .scheduled_to_end
                } else {
                    0.into()
                };
            let mut rsnapshot: RSnapShot = RSnapshots::instance().get(&day.into());
            rsnapshot.scheduled_to_end = scheduled_to_end_today;
            rsnapshot.total_shares = if globals().referral_shares > scheduled_to_end_today {
                globals().referral_shares - scheduled_to_end_today
            } else {
                0.into()
            };
            rsnapshot.inflation_amount = rsnapshot
                .total_shares
                .checked_mul(PRECISION_RATE.into())
                .unwrap_or_revert_with(Errors::MultiplicationOverflow3)
                .checked_div(self._referral_inflation(total_staked_today, total_supply))
                .unwrap_or_revert_with(Errors::DivisionByZero3);
            // store referral snapshot
            RSnapshots::instance().set(&day.into(), rsnapshot);
            // ------------------------------------
            // prepare snapshot for liquidity shares
            // reusing scheduledToEndToday variable
            let mut lsnapshot: LSnapShot = LSnapshots::instance().get(&day.into());
            lsnapshot.total_shares = globals().liquidity_shares;
            lsnapshot.inflation_amount = lsnapshot
                .total_shares
                .checked_mul(PRECISION_RATE.into())
                .unwrap_or_revert_with(Errors::MultiplicationOverflow4)
                .checked_div(self._liquidity_inflation(
                    total_staked_today,
                    total_supply,
                    runtime::call_versioned_contract(
                        key_to_hash(liquidity_guard(), Errors::InvalidHash8),
                        None,
                        "get_inflation",
                        runtime_args! {
                            "amount" => liquidity_rate()
                        },
                    ),
                ))
                .unwrap_or_revert_with(Errors::DivisionByZero4);
            // store liquidity snapshot
            LSnapshots::instance().set(&day.into(), lsnapshot);
            self._adjust_liquidity_rates();
            set_globals({
                let mut globals = globals();
                globals.current_stakeable_day = globals.current_stakeable_day + 1;
                globals
            });
            day += 1.into();
        }
    }

    /// @notice moves inflation up and down by 0.006% from regular shares to liquidity shares if the liquidityGuard is active (visa-versa)
    fn _adjust_liquidity_rates(&self) {
        if is_liquidity_guard_active() && liquidity_rate() < INFLATION_RATE_MAX {
            set_liquidity_rate(liquidity_rate() + 6);
            set_inflation_rate(inflation_rate() - 6);
            return;
        }
        if !is_liquidity_guard_active() && inflation_rate() < INFLATION_RATE_MAX {
            set_inflation_rate(inflation_rate() + 6);
            set_liquidity_rate(liquidity_rate() - 6);
        }
    }

    fn _inflation_amount(
        &self,
        total_staked: U256,
        total_supply: U256,
        total_penalties: U256,
        inflation_rate: U256,
    ) -> U256 {
        (total_staked + total_supply) * 10000 / inflation_rate + total_penalties
    }

    fn _referral_inflation(&self, total_staked: U256, total_supply: U256) -> U256 {
        (total_staked + total_supply) * 10000 / REFERRALS_RATE
    }

    fn _liquidity_inflation(
        &self,
        total_staked: U256,
        total_supply: U256,
        liquidity_rate: U256,
    ) -> U256 {
        (total_staked + total_supply) * 10000 / liquidity_rate
    }
}
