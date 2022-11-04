use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{Key, U256};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use staking_token::{
    errors::Errors, functions::package_hash, globals, is_liquidity_guard_active, set_globals,
    src::StakingToken, uniswap_pair, LSnapshots, LiquidityStake, LiquidityStakes,
    MIN_REFERRAL_DAYS, PRECISION_RATE,
};

pub trait LiquidityToken<Storage: ContractStorage>:
    ContractContext<Storage> + StakingToken<Storage>
{
    fn init(&self) {
        StakingToken::init(self);
    }

    /// @notice A method for a staker to create a liquidity stake
    /// @param _liquidityTokens amount of UNI-STAKEABLE staked.
    fn create_liquidity_stake(&mut self, liquidity_tokens: U256) -> Vec<u32> {
        self.snapshot_trigger();
        if !is_liquidity_guard_active() {
            runtime::revert(Errors::LiquidityGuardIsNotActive);
        }
        self._transfer_from(
            uniswap_pair(),
            self.get_caller(),
            package_hash(),
            liquidity_tokens,
        );
        let new_liquidity_stake: LiquidityStake = LiquidityStake {
            staked_amount: liquidity_tokens,
            reward_amount: Default::default(),
            start_day: self._next_stakeable_day(),
            close_day: Default::default(),
            is_active: true,
        };
        let liquidity_stake_id = self._generate_liquidity_stake_id(self.get_caller());
        set_globals({
            let mut globals = globals();
            globals.liquidity_shares = globals
                .liquidity_shares
                .checked_add(liquidity_tokens)
                .unwrap_or_revert_with(Errors::AdditionOverflow18);
            globals
        });
        LiquidityStakes::instance().set(
            &self.get_caller(),
            &liquidity_stake_id,
            new_liquidity_stake,
        );
        self._increase_liquidity_stake_count(self.get_caller());
        liquidity_stake_id
    }

    /// @notice A method for a staker to end a liquidity stake
    /// @param _liquidityStakeID - identification number
    fn end_liquidity_stake(&mut self, liquidity_stake_id: Vec<u32>) -> U256 {
        self.snapshot_trigger();
        let mut liquidity_stake =
            LiquidityStakes::instance().get(&self.get_caller(), &liquidity_stake_id);
        if !liquidity_stake.is_active {
            runtime::revert(Errors::NotAnActiveStake3);
        }
        liquidity_stake.is_active = false;
        liquidity_stake.close_day = self._current_stakeable_day();
        // liquidity_stake.reward_amount = self._calculate_reward_amount(liquidity_stake);
        self.mint(self.get_caller(), liquidity_stake.reward_amount);
        self._transfer(
            uniswap_pair(),
            self.get_caller(),
            liquidity_stake.staked_amount,
        );
        set_globals({
            let mut globals = globals();
            globals.liquidity_shares = globals
                .liquidity_shares
                .checked_sub(liquidity_stake.staked_amount)
                .unwrap_or_revert_with(Errors::SubtractionUnderflow3);
            globals
        });
        LiquidityStakes::instance().set(&self.get_caller(), &liquidity_stake_id, liquidity_stake);
        liquidity_stake.reward_amount
    }

    /// @notice returns full view and details of a liquidity stake belonging to caller
    /// @param _liquidityStakeID - stakeID
    fn check_liquidity_stake_by_id(
        &self,
        staker: Key,
        liquidity_stake_id: Vec<u32>,
    ) -> (U256, U256, U256, U256, bool) {
        let stake: LiquidityStake = LiquidityStakes::instance().get(&staker, &liquidity_stake_id);
        (
            stake.start_day.into(),
            stake.staked_amount,
            LiquidityToken::_calculate_reward_amount(self, stake),
            stake.close_day.into(),
            stake.is_active,
        )
    }

    /// @notice calculates reward when closing liquidity stake
    /// @param _liquidityStake - stake instance
    fn _calculate_reward_amount(&self, liquidity_stake: LiquidityStake) -> U256 {
        let max_calculation_day: U256 =
            (liquidity_stake.start_day + MIN_REFERRAL_DAYS as u64).into();
        let calculation_day = if globals().current_stakeable_day < max_calculation_day {
            globals().current_stakeable_day
        } else {
            max_calculation_day
        };
        let mut reward_amount: U256 = 0.into();
        let mut day: U256 = liquidity_stake.start_day.into();
        while day < calculation_day {
            reward_amount += liquidity_stake.staked_amount * PRECISION_RATE
                / LSnapshots::instance().get(&day).inflation_amount;
            day += 1.into();
        }
        reward_amount
    }
}
