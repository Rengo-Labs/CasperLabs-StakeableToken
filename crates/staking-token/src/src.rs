use casper_contract::contract_api::runtime;
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{Key, U256};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use common::{
    errors::Errors,
    events::{emit, Events},
    functions::account_zero_address,
};
use referral_token::{src::ReferralToken, *};

pub trait StakingToken<Storage: ContractStorage>:
    ContractContext<Storage> + ReferralToken<Storage>
{
    fn init(&self) {
        ReferralToken::init(self);
    }

    /// @notice A method for a staker to create multiple stakes
    /// @param _stakedAmount amount of WISE staked.
    /// @param _lockDays amount of days it is locked for.
    /// @param _referrer address of the referrer
    fn create_stake_bulk(
        &mut self,
        staked_amount: Vec<U256>,
        lock_days: Vec<u64>,
        referrer: Vec<Key>,
    ) {
        for i in 0..staked_amount.len() {
            let _ = self.create_stake(staked_amount[i], lock_days[i], referrer[i]);
        }
    }

    /// @notice A method for a staker to create a stake
    /// @param _stakedAmount amount of WISE staked.
    /// @param _lockDays amount of days it is locked for.
    /// @param _referrer address of the referrer
    fn create_stake(
        &mut self,
        staked_amount: U256,
        lock_days: u64,
        referrer: Key,
    ) -> (Vec<u32>, U256, Vec<u32>) {
        self.snapshot_trigger();
        if self.get_caller() == referrer || self._not_contract(referrer) {
            runtime::revert(Errors::InvalidReferrer);
        }
        if lock_days < MIN_LOCK_DAYS.into() || lock_days > MAX_LOCK_DAYS.into() {
            runtime::revert(Errors::StakeIsNotInRange);
        }
        if staked_amount < MIN_STAKE_AMOUNT.into() {
            runtime::revert(Errors::StakeIsNotLargeEnough);
        }
        let (new_stake, stake_id, start_day) =
            self._create_stake(self.get_caller(), staked_amount, lock_days, referrer);
        let mut referral_id: Vec<u32> = Default::default();
        if new_stake.referrer_shares > 0.into() {
            let referrer_link: ReferrerLink = ReferrerLink {
                staker: self.get_caller(),
                stake_id: stake_id.clone(),
                reward_amount: Default::default(),
                processed_days: Default::default(),
                is_active: true,
            };
            referral_id = self._generate_referral_id(referrer);
            ReferrerLinks::instance().set(&referrer, &referral_id, referrer_link);
            self._increase_referral_count(referrer);
            self._add_referrer_shares_to_end(new_stake.final_day.into(), new_stake.referrer_shares);
        }
        Stakes::instance().set(&self.get_caller(), &stake_id, new_stake);
        self._increase_stake_count(self.get_caller());
        self._increase_globals(
            new_stake.staked_amount,
            new_stake.stakes_shares,
            new_stake.referrer_shares,
        );
        self._add_scheduled_shares(new_stake.final_day.into(), new_stake.stakes_shares);
        emit(&Events::StakeStart {
            stake_id: stake_id.clone(),
            staker_address: self.get_caller(),
            referral_address: referrer,
            staked_amount: new_stake.staked_amount,
            stakes_shares: new_stake.stakes_shares,
            referral_shares: new_stake.referrer_shares,
            start_day: new_stake.start_day.into(),
            lock_days: new_stake.lock_days.into(),
            dai_equivalent: new_stake.dai_equivalent,
        });
        (stake_id, start_day.into(), referral_id)
    }

    /// @notice A method for a staker to start a stake
    /// @param staker
    /// @param stakedAmount
    /// @param lockDays
    fn _create_stake(
        &mut self,
        staker: Key,
        staked_amount: U256,
        lock_days: u64,
        referrer: Key,
    ) -> (Stake, Vec<u32>, u64) {
        self.burn(staker, staked_amount);
        let start_day = self._next_stakeable_day();
        let stake_id = self._generate_stake_id(staker);
        let mut new_stake: Stake = Stake {
            stakes_shares: Default::default(),
            staked_amount,
            reward_amount: Default::default(),
            start_day,
            lock_days,
            final_day: start_day + lock_days,
            close_day: Default::default(),
            scrape_day: Default::default(),
            dai_equivalent: self
                ._get_stable_usd_equivalent()
                .checked_mul(staked_amount)
                .unwrap_or_revert_with(Errors::MultiplicationOverflow5)
                .checked_div(YODAS_PER_STAKEABLE)
                .unwrap_or_revert_with(Errors::DivisionByZero5),
            referrer_shares: Default::default(),
            referrer: account_zero_address(),
            is_active: true,
        };
        self._stakes_shares(
            staked_amount,
            lock_days.into(),
            referrer,
            globals().share_price,
        );
        if self._non_zero_address(referrer) {
            new_stake.referrer = referrer;
            self._add_critical_mass(new_stake.referrer, new_stake.dai_equivalent);
            new_stake.referrer_shares =
                self._referrer_shares(staked_amount, lock_days.into(), referrer);
        }
        (new_stake, stake_id, start_day)
    }

    /// @notice A method for a staker to remove a stake belonging to his address by providing ID of a stake.
    /// @param stake_id unique bytes sequence reference to the stake
    fn end_stake(&mut self, stake_id: Vec<u32>) -> U256 {
        self.snapshot_trigger();
        let (ended_stake, penalty_amount) = self._end_stake(self.get_caller(), stake_id.clone());
        self._decrease_globals(
            ended_stake.staked_amount,
            ended_stake.stakes_shares,
            ended_stake.referrer_shares,
        );
        self._remove_scheduled_shares(ended_stake.final_day.into(), ended_stake.stakes_shares);
        self._remove_referrer_shares_to_end(
            ended_stake.final_day.into(),
            ended_stake.referrer_shares,
        );
        self._remove_critical_mass(
            ended_stake.referrer,
            ended_stake.dai_equivalent,
            ended_stake.start_day.into(),
        );
        self._store_penalty(ended_stake.close_day, penalty_amount);
        self._share_price_update(
            if ended_stake.staked_amount > penalty_amount {
                ended_stake.staked_amount - penalty_amount
            } else {
                0.into()
            },
            ended_stake.reward_amount + Scrapes::instance().get(&self.get_caller(), &stake_id),
            ended_stake.referrer,
            ended_stake.lock_days.into(),
            ended_stake.stakes_shares,
        );
        emit(&Events::StakeEnd {
            stake_id,
            staker_address: self.get_caller(),
            referral_address: ended_stake.referrer,
            staked_amount: ended_stake.staked_amount,
            stakes_shares: ended_stake.stakes_shares,
            referral_shares: ended_stake.referrer_shares,
            reward_amount: ended_stake.reward_amount,
            close_day: ended_stake.close_day.into(),
            penalty_amount,
        });
        ended_stake.reward_amount
    }

    fn _end_stake(&mut self, staker: Key, stake_id: Vec<u32>) -> (Stake, U256) {
        if !Stakes::instance().get(&staker, &stake_id).is_active {
            runtime::revert(Errors::NotAnActiveStake1);
        }
        let mut stake = Stakes::instance().get(&staker, &stake_id);
        stake.close_day = self._current_stakeable_day();
        stake.reward_amount = self._calculate_reward_amount(stake);
        let penalty = self._calculate_penalty_amount(stake);
        stake.is_active = false;
        self.mint(
            staker,
            if stake.staked_amount > penalty {
                stake.staked_amount - penalty
            } else {
                0.into()
            },
        );
        self.mint(staker, stake.reward_amount);
        (stake, penalty)
    }

    /// @notice alloes to scrape interest from active stake
    /// @param _stakeID unique bytes sequence reference to the stake
    /// @param _scrapeDays amount of days to proccess, 0 = all
    fn scrape_interest(
        &mut self,
        stake_id: Vec<u32>,
        scrape_days: u64,
    ) -> (U256, U256, U256, U256, U256) {
        self.snapshot_trigger();
        if Stakes::instance()
            .get(&self.get_caller(), &stake_id)
            .is_active
        {
            runtime::revert(Errors::NotAnActiveStake2);
        }
        let mut stake = Stakes::instance().get(&self.get_caller(), &stake_id);
        let mut scrape_day = if scrape_days > 0 {
            self._starting_day(stake)
                .checked_add(scrape_days.into())
                .unwrap_or_revert_with(Errors::AdditionOverflow13)
        } else {
            self._calculation_day(stake)
        };
        scrape_day = if scrape_day > stake.final_day.into() {
            self._calculation_day(stake)
        } else {
            scrape_day
        };
        let scrape_amount =
            self._loop_reward_amount(stake.stakes_shares, self._starting_day(stake), scrape_day);
        let mut referrer_penalty: U256 = 0.into();
        let mut stakers_penalty: U256 = 0.into();
        let mut remaining_days: U256 = 0.into();
        if !self._is_mature_stake(stake) {
            remaining_days = self._days_left(stake);
            stakers_penalty = self._stakes_shares(
                scrape_amount,
                remaining_days,
                self.get_caller(),
                globals().share_price,
            );
            stake.stakes_shares = stake
                .stakes_shares
                .checked_sub(stakers_penalty)
                .unwrap_or_revert_with(Errors::SubtractionUnderflow1);
            self._remove_scheduled_shares(stake.final_day.into(), stakers_penalty);
            if stake.referrer_shares > 0.into() {
                referrer_penalty = self._stakes_shares(
                    scrape_amount,
                    remaining_days,
                    account_zero_address(),
                    globals().share_price,
                );
                stake.referrer_shares = stake
                    .referrer_shares
                    .checked_sub(referrer_penalty)
                    .unwrap_or_revert_with(Errors::SubtractionUnderflow2);
                self._remove_referrer_shares_to_end(stake.final_day.into(), referrer_penalty);
            }
            self._decrease_globals(0.into(), stakers_penalty, referrer_penalty);
            self._share_price_update(
                stake.staked_amount,
                scrape_amount,
                stake.referrer,
                stake.lock_days.into(),
                stake.stakes_shares,
            );
        } else {
            Scrapes::instance().set(
                &self.get_caller(),
                &stake_id,
                Scrapes::instance()
                    .get(&self.get_caller(), &stake_id)
                    .checked_add(scrape_amount)
                    .unwrap_or_revert_with(Errors::AdditionOverflow14),
            );
            self._share_price_update(
                stake.staked_amount,
                Scrapes::instance().get(&self.get_caller(), &stake_id),
                stake.referrer,
                stake.lock_days.into(),
                stake.stakes_shares,
            );
        }
        stake.scrape_day = scrape_day;
        Stakes::instance().set(&self.get_caller(), &stake_id, stake);
        self.mint(self.get_caller(), scrape_amount);
        emit(&Events::InterestScraped {
            stake_id,
            staker_address: self.get_caller(),
            scrape_amount,
            scrape_day,
            stakers_penalty,
            referrer_penalty,
            current_stakeable_day: self._current_stakeable_day().into(),
        });
        (
            scrape_day,
            scrape_amount,
            remaining_days,
            stakers_penalty,
            referrer_penalty,
        )
    }

    fn _add_scheduled_shares(&self, final_day: U256, shares: U256) {
        ScheduledToEnd::instance().set(
            &final_day,
            ScheduledToEnd::instance()
                .get(&final_day)
                .checked_add(shares)
                .unwrap_or_revert_with(Errors::AdditionOverflow15),
        );
    }

    fn _remove_scheduled_shares(&self, final_day: U256, shares: U256) {
        if self._not_past(final_day) {
            ScheduledToEnd::instance().set(
                &final_day,
                if ScheduledToEnd::instance().get(&final_day) > shares {
                    ScheduledToEnd::instance().get(&final_day) - shares
                } else {
                    0.into()
                },
            );
        } else {
            let day: U256 = self._previous_stakeable_day().into();
            Snapshots::instance().set(&day, {
                let mut snapshots = Snapshots::instance().get(&day);
                snapshots.scheduled_to_end = if snapshots.scheduled_to_end > shares {
                    snapshots.scheduled_to_end - shares
                } else {
                    0.into()
                };
                snapshots
            });
        }
    }

    fn _share_price_update(
        &self,
        staked_amount: U256,
        reward_amount: U256,
        referrer: Key,
        lock_days: U256,
        stake_shares: U256,
    ) {
        if stake_shares > 0.into() && self._current_stakeable_day() > FORMULA_DAY.into() {
            let mut new_share_price: U256 = self._get_new_share_price(
                staked_amount,
                reward_amount,
                stake_shares,
                lock_days,
                referrer,
            );
            if new_share_price > globals().share_price {
                new_share_price = if new_share_price
                    < globals()
                        .share_price
                        .checked_mul(110.into())
                        .unwrap_or_revert_with(Errors::MultiplicationOverflow11)
                        .checked_div(100.into())
                        .unwrap_or_revert_with(Errors::DivisionByZero9)
                {
                    new_share_price
                } else {
                    globals()
                        .share_price
                        .checked_mul(110.into())
                        .unwrap_or_revert_with(Errors::MultiplicationOverflow12)
                        .checked_div(100.into())
                        .unwrap_or_revert_with(Errors::DivisionByZero10)
                };
                emit(&Events::NewSharePrice {
                    new_share_price,
                    old_share_price: globals().share_price,
                    current_stakeable_day: self._current_stakeable_day(),
                });
                set_globals({
                    let mut globals = globals();
                    globals.share_price = new_share_price;
                    globals
                });
            }
            return;
        }
        if self._current_stakeable_day() == FORMULA_DAY as u64 {
            set_globals({
                let mut globals = globals();
                globals.share_price = 1100000000.into(); // 110E15 <=> 110E7
                globals
            });
        }
    }

    fn _get_new_share_price(
        &self,
        staked_amount: U256,
        reward_amount: U256,
        stake_shares: U256,
        lock_days: U256,
        referrer: Key,
    ) -> U256 {
        let bonus_amount = self._get_bonus(
            lock_days,
            if self._non_zero_address(referrer) {
                110000.into() // 11E9 <=> 11E4
            } else {
                100000.into() // 10E9 <=> 10E4
            },
        );
        staked_amount
            .checked_add(reward_amount)
            .unwrap_or_revert_with(Errors::AdditionOverflow17)
            .checked_mul(bonus_amount)
            .unwrap_or_revert_with(Errors::MultiplicationOverflow9)
            .checked_mul(10000.into()) // 1E8 <=> 1E4
            .unwrap_or_revert_with(Errors::MultiplicationOverflow10)
            .checked_div(stake_shares)
            .unwrap_or_revert_with(Errors::DivisionByZero8)
    }

    fn check_mature_stake(&self, staker: Key, stake_id: Vec<u32>) -> bool {
        let stake: Stake = Stakes::instance().get(&staker, &stake_id);
        self._is_mature_stake(stake)
    }

    fn check_stake_by_id(
        &self,
        staker: Key,
        stake_id: Vec<u32>,
    ) -> (
        U256,
        U256,
        U256,
        U256,
        U256,
        U256,
        U256,
        U256,
        U256,
        bool,
        bool,
    ) {
        let stake: Stake = Stakes::instance().get(&staker, &stake_id);
        (
            stake.start_day.into(),
            stake.lock_days.into(),
            stake.final_day.into(),
            stake.close_day.into(),
            stake.scrape_day,
            stake.staked_amount,
            stake.stakes_shares,
            self._check_reward_amount(stake),
            self._calculate_penalty_amount(stake),
            stake.is_active,
            self._is_mature_stake(stake),
        )
    }

    fn _stakes_shares(
        &self,
        staked_amount: U256,
        lock_days: U256,
        referrer: Key,
        share_price: U256,
    ) -> U256 {
        if self._non_zero_address(referrer) {
            // 11E9 <=> 11E4
            self._shares_amount(staked_amount, lock_days, share_price, 110000.into())
        } else {
            // 10E9 <=> 10E4
            self._shares_amount(staked_amount, lock_days, share_price, 100000.into())
        }
    }

    fn _shares_amount(
        &self,
        staked_amount: U256,
        lock_days: U256,
        share_price: U256,
        extra_bonus: U256,
    ) -> U256 {
        self._base_amount(staked_amount, share_price)
            .checked_mul(self._get_bonus(lock_days, extra_bonus))
            .unwrap_or_revert_with(Errors::MultiplicationOverflow6)
            .checked_div(100000.into()) // 10E9 <=> 10E4
            .unwrap_or_revert_with(Errors::MultiplicationOverflow6)
    }

    fn _get_bonus(&self, lock_days: U256, extra_bonus: U256) -> U256 {
        self._regular_bonus(lock_days, DAILY_BONUS_A.into(), MAX_BONUS_DAYS_A.into())
            + self._regular_bonus(
                if lock_days > MAX_BONUS_DAYS_A.into() {
                    lock_days - MAX_BONUS_DAYS_A
                } else {
                    0.into()
                },
                DAILY_BONUS_B.into(),
                MAX_BONUS_DAYS_B.into(),
            )
            + extra_bonus
    }

    fn _regular_bonus(&self, lock_days: U256, daily: U256, max_days: U256) -> U256 {
        (if lock_days > max_days {
            max_days
                .checked_mul(daily)
                .unwrap_or_revert_with(Errors::MultiplicationOverflow7)
        } else {
            lock_days
                .checked_mul(daily)
                .unwrap_or_revert_with(Errors::MultiplicationOverflow8)
        })
        .checked_div(100000.into()) // 10E9 <=> 10E4
        .unwrap_or_revert_with(Errors::DivisionByZero7)
    }

    fn _base_amount(&self, staked_amount: U256, share_price: U256) -> U256 {
        staked_amount
            .checked_mul(PRECISION_RATE.into())
            .unwrap_or_revert_with(Errors::MultiplicationOverflow6)
            .checked_div(share_price)
            .unwrap_or_revert_with(Errors::DivisionByZero6)
    }

    fn _referrer_shares(&self, staked_amount: U256, lock_days: U256, referrer: Key) -> U256 {
        if self._not_critical_mass_referrer(referrer) || lock_days < MIN_REFERRAL_DAYS.into() {
            0.into()
        } else {
            self._shares_amount(
                staked_amount,
                lock_days,
                globals().share_price,
                100000.into(), // 10E9 <=> 10E4
            )
        }
    }

    fn _check_reward_amount(&self, stake: Stake) -> U256 {
        if stake.is_active {
            self._detect_reward(stake)
        } else {
            stake.reward_amount
        }
    }

    fn _detect_reward(&self, stake: Stake) -> U256 {
        if self._stake_not_started(stake) {
            0.into()
        } else {
            self._calculate_reward_amount(stake)
        }
    }

    fn _store_penalty(&self, store_day: u64, penalty: U256) {
        if penalty > 0.into() {
            TotalPenalties::instance().set(
                &store_day.into(),
                TotalPenalties::instance()
                    .get(&store_day.into())
                    .checked_add(penalty)
                    .unwrap_or_revert_with(Errors::AdditionOverflow16),
            );
        }
    }

    fn _calculate_penalty_amount(&self, stake: Stake) -> U256 {
        if self._stake_not_started(stake) || self._is_mature_stake(stake) {
            0.into()
        } else {
            self._get_penalties(stake)
        }
    }

    fn _get_penalties(&self, stake: Stake) -> U256 {
        stake.staked_amount
            * (U256::from(100)
                + (U256::from(800) * (self._days_left(stake) - U256::from(1))
                    / (self._get_lock_days(stake))))
            / 1000
    }

    fn _calculate_reward_amount(&self, stake: Stake) -> U256 {
        self._loop_reward_amount(
            stake.stakes_shares,
            self._starting_day(stake),
            self._calculation_day(stake),
        )
    }

    fn _loop_reward_amount(&self, stake_shares: U256, start_day: U256, final_day: U256) -> U256 {
        let mut reward_amount: U256 = 0.into();
        let mut day: U256 = start_day;
        while day < final_day {
            reward_amount +=
                stake_shares * PRECISION_RATE / Snapshots::instance().get(&day).inflation_amount;
            day = day + 1;
        }
        reward_amount
    }
}
