use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{runtime_args, Key, RuntimeArgs, U256};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use snapshot::{
    data::*,
    errors::Errors,
    events::{emit, Events},
    functions::key_to_hash,
    latest_stable_usd_equivalent, path,
    src::Snapshot,
    uniswap_router, CriticalMass, ReferralSharesToEnd, ReferrerLink, ReferrerLinks, Stake, Stakes,
    PRECISION_RATE, THRESHOLD_LIMIT, YODAS_PER_STAKEABLE,
};

pub trait ReferralToken<Storage: ContractStorage>:
    ContractContext<Storage> + Snapshot<Storage>
{
    fn init(&self) {
        Snapshot::init(self);
    }

    fn _add_referrer_shares_to_end(&self, final_day: U256, shares: U256) {
        ReferralSharesToEnd::instance().set(
            &final_day,
            ReferralSharesToEnd::instance()
                .get(&final_day)
                .checked_add(shares)
                .unwrap_or_revert_with(Errors::AdditionOverflow4),
        );
    }

    fn _remove_referrer_shares_to_end(&self, final_day: U256, shares: U256) {
        if self._not_past(final_day) {
            ReferralSharesToEnd::instance().set(
                &final_day,
                if ReferralSharesToEnd::instance().get(&final_day) > shares {
                    ReferralSharesToEnd::instance().get(&final_day) - shares
                } else {
                    0.into()
                },
            );
        } else {
            let day: U256 = self._previous_stakeable_day().into();
            RSnapshots::instance().set(&day, {
                let mut r_snapshots = RSnapshots::instance().get(&day);
                r_snapshots.scheduled_to_end =
                    if RSnapshots::instance().get(&day).scheduled_to_end > shares {
                        RSnapshots::instance().get(&day).scheduled_to_end - shares
                    } else {
                        0.into()
                    };
                r_snapshots
            });
        }
    }

    fn _below_threshold_level(&self, referrer: Key) -> bool {
        CriticalMass::instance().get(&referrer).total_amount < THRESHOLD_LIMIT.into()
    }

    fn _add_critical_mass(&self, referrer: Key, dai_equivalent: U256) {
        CriticalMass::instance().set(&referrer, {
            let mut critical_mass = CriticalMass::instance().get(&referrer);
            critical_mass.total_amount = critical_mass
                .total_amount
                .checked_add(dai_equivalent)
                .unwrap_or_revert_with(Errors::AdditionOverflow5);
            critical_mass.activation_day = self._determine_activation_day(referrer);
            critical_mass
        });
    }

    fn _remove_critical_mass(&self, referrer: Key, dai_equivalent: U256, start_day: U256) {
        if !self._not_future(start_day) && self._non_zero_address(referrer) {
            CriticalMass::instance().set(&referrer, {
                let mut critical_mass = CriticalMass::instance().get(&referrer);
                critical_mass.total_amount = if critical_mass.total_amount > dai_equivalent {
                    critical_mass.total_amount - dai_equivalent
                } else {
                    0.into()
                };
                critical_mass.activation_day = self._determine_activation_day(referrer);
                critical_mass
            });
        }
    }

    fn _determine_activation_day(&self, referrer: Key) -> U256 {
        if self._below_threshold_level(referrer) {
            0.into()
        } else {
            self._activation_day(referrer)
        }
    }

    fn _activation_day(&self, referrer: Key) -> U256 {
        if CriticalMass::instance().get(&referrer).activation_day > 0.into() {
            CriticalMass::instance().get(&referrer).activation_day
        } else {
            self._current_stakeable_day().into()
        }
    }

    fn get_stable_usd_equivalent(&self) -> U256 {
        self._get_stable_usd_equivalent()
    }

    fn _get_stable_usd_equivalent(&self) -> U256 {
        let results: Vec<U256> = runtime::call_versioned_contract(
            key_to_hash(uniswap_router(), Errors::InvalidHash9),
            None,
            "get_amounts_out",
            runtime_args! {
                "amount_in" => YODAS_PER_STAKEABLE,
                "path" => path()
            },
        );
        if !results.is_empty() {
            results[3]
        } else {
            latest_stable_usd_equivalent()
        }
    }

    fn referrer_interest(&mut self, referral_id: Vec<u32>, scrape_days: U256) {
        self.snapshot_trigger();
        self._referrer_interest(self.get_caller(), referral_id, scrape_days);
    }

    fn referrer_interest_bulk(&mut self, referral_ids: Vec<Vec<u32>>, scrape_days: Vec<U256>) {
        self.snapshot_trigger();
        for i in 0..referral_ids.len() {
            self._referrer_interest(self.get_caller(), referral_ids[i].clone(), scrape_days[i]);
        }
    }

    fn _referrer_interest(&mut self, referrer: Key, referral_id: Vec<u32>, mut process_days: U256) {
        let mut link: ReferrerLink = ReferrerLinks::instance().get(&referrer, &referral_id);
        if !link.is_active {
            runtime::revert(Errors::NotActive);
        }
        let staker: Key = link.staker;
        let stake_id: Vec<u32> = link.stake_id.clone();
        let stake: Stake = Stakes::instance().get(&staker, &stake_id);
        let start_day: U256 = self._determine_start_day(stake, link.clone());
        let mut final_day: U256 = self._determine_final_day(stake);
        if self._stake_ended(stake) {
            if process_days > 0.into() && process_days < self._days_diff(start_day, final_day) {
                link.processed_days = link
                    .processed_days
                    .checked_add(process_days)
                    .unwrap_or_revert_with(Errors::AdditionOverflow7);
                final_day = start_day
                    .checked_add(process_days)
                    .unwrap_or_revert_with(Errors::AdditionOverflow1);
            } else {
                link.is_active = false;
            }
        } else {
            process_days = self._days_diff(start_day, self._current_stakeable_day().into());
            link.processed_days = link
                .processed_days
                .checked_add(process_days)
                .unwrap_or_revert_with(Errors::AdditionOverflow9);
            final_day = start_day
                .checked_add(process_days)
                .unwrap_or_revert_with(Errors::AdditionOverflow10);
        }
        let referral_interest = self._check_referral_interest(stake, start_day, final_day);
        link.reward_amount = link
            .reward_amount
            .checked_add(referral_interest)
            .unwrap_or_revert_with(Errors::AdditionOverflow12);
        ReferrerLinks::instance().set(&referrer, &referral_id, link);
        self.mint(referrer, referral_interest);
        emit(&Events::ReferralCollected {
            staker,
            stake_id,
            referrer,
            referrer_id: referral_id,
            reward_amount: referral_interest,
        });
    }

    fn check_referrals_by_id(
        &self,
        referrer: Key,
        referral_id: Vec<u32>,
    ) -> (Key, Vec<u32>, U256, U256, bool, bool, bool, bool) {
        let link: ReferrerLink = ReferrerLinks::instance().get(&referrer, &referral_id);
        let stake: Stake = Stakes::instance().get(&link.staker, &link.stake_id);
        (
            link.staker,
            link.stake_id.clone(),
            stake.referrer_shares,
            self._check_referral_interest(
                stake,
                self._determine_start_day(stake, link.clone()),
                self._determine_final_day(stake),
            ),
            link.is_active,
            stake.is_active,
            self._is_mature_stake(stake),
            self._stake_ended(stake),
        )
    }

    fn _check_referral_interest(&self, stake: Stake, start_day: U256, final_day: U256) -> U256 {
        if self._not_critical_mass_referrer(stake.referrer) {
            0.into()
        } else {
            self._get_referral_interest(stake, start_day, final_day)
        }
    }

    fn _get_referral_interest(&self, stake: Stake, start_day: U256, final_day: U256) -> U256 {
        let mut referral_interest: U256 = 0.into();
        let mut day: U256 = start_day;
        while day < final_day {
            referral_interest += stake.stakes_shares * PRECISION_RATE
                / RSnapshots::instance().get(&day).inflation_amount;
            day = day + 1;
        }
        referral_interest
    }

    fn _determine_start_day(&self, stake: Stake, link: ReferrerLink) -> U256 {
        (if CriticalMass::instance().get(&stake.referrer).activation_day > stake.start_day.into() {
            CriticalMass::instance().get(&stake.referrer).activation_day
        } else {
            stake.start_day.into()
        })
        .checked_add(link.processed_days)
        .unwrap_or_revert_with(Errors::AdditionOverflow6)
    }

    fn _determine_final_day(&self, stake: Stake) -> U256 {
        if stake.close_day > 0 {
            stake.close_day.into()
        } else {
            self._calculation_day(stake)
        }
    }
}
