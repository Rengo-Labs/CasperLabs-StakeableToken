use crate::data::{self};
use alloc::{string::ToString, vec::Vec};
use bep20_crate::BEP20;
use casper_contract::contract_api::runtime;
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    ApiError, Key, U256,
};
use contract_utils::{ContractContext, ContractStorage};
use declaration_crate::Declaration;
use globals_crate::Globals;
use helper_crate::Helper;
use referral_token_crate::ReferralToken;
use snapshot_crate::Snapshot;
use staking_token_crate::StakingToken;
use timing_crate::Timing;
use wise_token_utils::{
    commons::key_names::*, declaration::parameters::*, declaration::structs::*,
    error_codes::ErrorCodes, snapshot,
};

pub trait LiquidityToken<Storage: ContractStorage>:
    ContractContext<Storage>
    + Declaration<Storage>
    + Globals<Storage>
    + Timing<Storage>
    + Helper<Storage>
    + Snapshot<Storage>
    + ReferralToken<Storage>
    + StakingToken<Storage>
    + BEP20<Storage>
{
    // Will be called by constructor
    fn init(&mut self, sbnb_contract_hash: Key, pair_contract_hash: Key, guard_contract_hash: Key) {
        data::set_sbnb_hash(sbnb_contract_hash);
        data::set_pair_hash(pair_contract_hash);
        data::set_guard_hash(guard_contract_hash);
    }

    fn to_bytes16(&self, x: U256) -> Vec<u16> {
        let x: Vec<u8> = x.to_bytes().unwrap_or_default();
        let result: Vec<u16> = x
            .chunks_exact(2)
            .into_iter()
            .map(|a| u16::from_ne_bytes([a[0], a[1]]))
            .collect(); // Create a native endian integer value
        result
    }

    fn _create_liquidity_stake(&self, _liquidity_tokens: U256) -> Vec<u32> {
        let pair_hash: Key = data::pair_hash();
        // calling hooks

        Snapshot::_snapshot_trigger(self); // snapshot trigger is a modifier in solidity

        let liquidity_guard_status: bool = Declaration::get_liquidity_guard_status(self);
        if liquidity_guard_status == false {
            runtime::revert(ApiError::User(ErrorCodes::LiquidityGuardDisabled as u16));
        }

        let ret: Result<(), u32> = Helper::transfer_from(
            self,
            pair_hash,
            self.get_caller(),
            Key::from(data::package_hash()),
            _liquidity_tokens,
        );
        if ret.is_err() {
            runtime::revert(ret.unwrap_err());
        }
        let liquidity_stake_id: Vec<u32> =
            Helper::generate_liquidity_stake_id(self, self.get_caller());
        let next_wise_day: u64 = Timing::_next_wise_day(self);

        let mut new_liquidity_stake: LiquidityStake = LiquidityStake::new();
        new_liquidity_stake.start_day = next_wise_day;
        new_liquidity_stake.staked_amount = _liquidity_tokens;
        new_liquidity_stake.is_active = true;

        let mut liquidity_shares: U256 =
            Globals::get_globals(self, GLOBALS_LIQUIDITY_SHARES.to_string());
        // liquidity_shares = liquidity_shares + _liquidity_tokens;
        liquidity_shares = liquidity_shares
            .checked_add(_liquidity_tokens)
            .ok_or(ApiError::User(ErrorCodes::Overflow as u16))
            .unwrap_or_revert();
        Globals::set_globals(self, GLOBALS_LIQUIDITY_SHARES.to_string(), liquidity_shares);

        let new_liquidity_stake_bytes: Vec<u8> = new_liquidity_stake.clone().into_bytes().unwrap();
        Declaration::set_liquidity_stake(
            self,
            self.get_caller(),
            liquidity_stake_id.clone(),
            new_liquidity_stake_bytes.clone(),
        );
        Helper::increase_liquidity_stake_count(self, self.get_caller());
        liquidity_stake_id
    }

    fn _check_liquidity_stake_by_id(&self, _staker: Key, _liquidity_stake_id: Vec<u32>) -> Vec<u8> {
        Declaration::get_liquidity_stake(self, _staker, _liquidity_stake_id)
    }

    fn _end_liquidity_stake(&self, _liquidity_stake_id: Vec<u32>) -> U256 {
        let pair_hash = data::pair_hash();

        let mut liquidity_stake_bytes: Vec<u8> =
            Declaration::get_liquidity_stake(self, self.get_caller(), _liquidity_stake_id.clone());
        let mut liquidity_stake_struct: LiquidityStake =
            LiquidityStake::from_bytes(&liquidity_stake_bytes)
                .unwrap()
                .0;

        if !liquidity_stake_struct.is_active {
            runtime::revert(ApiError::User(ErrorCodes::StakeInactive as u16))
        }

        let current_wise_day: u64 = Timing::_current_wise_day(self);

        liquidity_stake_struct.is_active = false;
        liquidity_stake_struct.close_day = current_wise_day;
        liquidity_stake_struct.reward_amount =
            LiquidityToken::_calculate_reward_amount(self, &liquidity_stake_struct);

        let _: () = BEP20::_mint(
            self,
            self.get_caller(),
            liquidity_stake_struct.reward_amount,
        );

        let ret: Result<(), u32> = Helper::transfer(
            self,
            pair_hash,
            self.get_caller(),
            liquidity_stake_struct.staked_amount,
        );
        if ret.is_err() {
            runtime::revert(ret.unwrap_err());
        }

        let mut liquidity_shares: U256 =
            Globals::get_globals(self, GLOBALS_LIQUIDITY_SHARES.to_string());
        // liquidity_shares = liquidity_shares - liquidity_stake_struct.staked_amount;
        liquidity_shares = liquidity_shares
            .checked_sub(liquidity_stake_struct.staked_amount)
            .ok_or(ApiError::User(ErrorCodes::Underflow as u16))
            .unwrap_or_revert();
        Globals::set_globals(
            self,
            GLOBALS_LIQUIDITY_SHARES.to_string(),
            liquidity_shares.clone(),
        );

        liquidity_stake_bytes = liquidity_stake_struct.clone().into_bytes().unwrap();
        Declaration::set_liquidity_stake(
            self,
            self.get_caller(),
            _liquidity_stake_id.clone(),
            liquidity_stake_bytes.clone(),
        );

        liquidity_stake_struct.reward_amount
    }
    fn _calculate_reward_amount(&self, _liquidity_stake: &LiquidityStake) -> U256 {
        let constant_parameters_bytes: Vec<u8> = Declaration::get_declaration_constants(self);
        let constant_parameters_struct: ConstantParameters =
            ConstantParameters::from_bytes(&constant_parameters_bytes)
                .unwrap()
                .0;

        // let max_calculation_day: U256 = U256::from(_liquidity_stake.start_day)
        //     + U256::from(constant_parameters_struct.min_referral_days);
        let max_calculation_day: U256 = U256::from(_liquidity_stake.start_day)
            .checked_add(U256::from(constant_parameters_struct.min_referral_days))
            .ok_or(ApiError::User(ErrorCodes::Overflow as u16))
            .unwrap_or_revert();
        let current_wise_day_globals: U256 =
            Globals::get_globals(self, GLOBALS_CURRENT_WISE_DAY.to_string());
        let calculation_day: U256 = if current_wise_day_globals < max_calculation_day {
            current_wise_day_globals
        } else {
            max_calculation_day
        };

        let mut reward_amount: U256 = U256::from(0);
        let mut day: U256 = U256::from(_liquidity_stake.start_day);

        let l_snapshot_string: Vec<u8> =
            Snapshot::get_struct_from_key(self, &day, SNAPSHOT_LSNAPSHOTS_DICT.to_string());
        let l_snapshot_struct: snapshot::structs::LSnapShot =
            snapshot::structs::LSnapShot::from_bytes(&l_snapshot_string)
                .unwrap()
                .0;

        while day < calculation_day {
            // reward_amount = reward_amount
            //     + ((U256::from(_liquidity_stake.staked_amount)
            //         * constant_parameters_struct.precision_rate)
            //         / l_snapshot_struct.inflation_amount);
            reward_amount = reward_amount
                .checked_add(_liquidity_stake.staked_amount.into())
                .ok_or(ApiError::User(ErrorCodes::Overflow as u16))
                .unwrap_or_revert()
                .checked_mul(constant_parameters_struct.precision_rate)
                .ok_or(ApiError::User(ErrorCodes::Overflow as u16))
                .unwrap_or_revert()
                .checked_div(l_snapshot_struct.inflation_amount)
                .ok_or(ApiError::User(ErrorCodes::DivisionByZero as u16))
                .unwrap_or_revert();
            // day++
            day = day
                .checked_add(1.into())
                .ok_or(ApiError::User(ErrorCodes::Overflow as u16))
                .unwrap_or_revert();
        }

        reward_amount
    }
}
