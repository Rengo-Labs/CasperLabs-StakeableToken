use crate::alloc::string::ToString;
use casper_contract::unwrap_or_revert::UnwrapOrRevert;

use crate::data::{self};
use alloc::{string::String, vec::Vec};
use casper_contract::contract_api::runtime;
use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    contracts::ContractHash,
    runtime_args, ApiError, Key, RuntimeArgs, U128, U256,
};
use contract_utils::{ContractContext, ContractStorage};
use stakeable_token_utils::{
    commons::key_names::*, declaration, error_codes::ErrorCodes, events::*, snapshot,
};

use erc20_crate::ERC20;

use declaration_crate::Declaration;
use globals_crate::Globals;
use helper_crate::Helper;
use timing_crate::Timing;

pub trait Snapshot<Storage: ContractStorage>:
    ContractContext<Storage>
    + Declaration<Storage>
    + Globals<Storage>
    + Timing<Storage>
    + Helper<Storage>
    + ERC20<Storage>
{
    // Will be called by constructor
    fn init(&self, scspr_contract_hash: Key, pair_contract_hash: Key, guard_contract_hash: Key) {
        data::set_scspr_hash(scspr_contract_hash);
        data::set_pair_hash(pair_contract_hash);
        data::set_guard_hash(guard_contract_hash);
        data::SnapshotsDict::init();
        data::RSnapshotsDict::init();
        data::LSnapshotsDict::init();
    }

    fn liquidity_guard_trigger(&self) {
        let pair_hash = data::pair_hash();
        let scspr_hash = data::scspr_hash();

        let total_supply: U256 = ERC20::total_supply(self);

        let liquidity_guard_status: bool = Declaration::get_liquidity_guard_status(self);
        // third return value is block_timestamp_last
        let (reserve_a, reserve_b, block_timestamp_last): (U128, U128, u64) =
            runtime::call_contract(
                Self::_create_hash_from_key(pair_hash),
                "get_reserves",
                runtime_args! {},
            );

        emit(&StakeableEvents::UniswapReserves {
            reserve_a,
            reserve_b,
            block_timestamp_last,
        });

        let token1: Key = runtime::call_contract(
            Self::_create_hash_from_key(pair_hash),
            "token1",
            runtime_args! {},
        );

        let on_pancake: U256 = if token1.eq(&scspr_hash) {
            U256::from(reserve_a.as_u128())
        } else {
            U256::from(reserve_b.as_u128())
        };

        let ratio: U256 = if total_supply == 0.into() {
            0.into()
        } else {
            on_pancake
                .checked_mul(200.into())
                .ok_or(ApiError::User(ErrorCodes::Overflow as u16))
                .unwrap_or_revert()
                .checked_div(total_supply.into())
                .ok_or(ApiError::User(ErrorCodes::DivisionByZero as u16))
                .unwrap_or_revert()
        };

        if ratio < 40.into() && liquidity_guard_status == false {
            self.enable_liquidity_guard();
        }

        if ratio > 60.into() && liquidity_guard_status == true {
            self.disable_liquidity_guard();
        }

        emit(&StakeableEvents::LiquidityGuardStatus {
            liquidity_guard_status,
        });
    }

    fn _daily_snapshot_point(&self, _update_day: u64) {
        self.liquidity_guard_trigger();

        let mut scheduled_to_end_today: U256 = 0.into();
        let total_staked_today: U256 = Globals::get_globals(self, GLOBALS_TOTAL_STAKED.to_string());
        let snapshots = data::SnapshotsDict::instance();
        let lsnapshots = data::LSnapshotsDict::instance();
        let rsnapshots = data::RSnapshotsDict::instance();
        let parameters: Vec<u8> = Declaration::get_declaration_constants(self);
        let parameters: declaration::parameters::ConstantParameters =
            declaration::parameters::ConstantParameters::from_bytes(&parameters)
                .unwrap()
                .0;

        for _day in
            Globals::get_globals(self, GLOBALS_CURRENT_WISE_DAY.to_string()).as_u64().._update_day
        {
            // previouis days snapshot
            let previous_snapshot: Vec<u8> = snapshots.get(&(_day - 1).into());
            let previous_snapshot: snapshot::structs::Snapshot =
                snapshot::structs::Snapshot::from_bytes(&previous_snapshot)
                    .unwrap()
                    .0;

            //get schedules to end today and calculate for today
            scheduled_to_end_today = Declaration::get_scheduled_to_end(self, _day.into())
                .checked_add(previous_snapshot.scheduled_to_end)
                .ok_or(ApiError::User(ErrorCodes::Overflow as u16))
                .unwrap_or_revert();

            // create snapshot struct for today
            let mut snapshot = snapshot::structs::Snapshot::new();
            snapshot.scheduled_to_end = scheduled_to_end_today;

            // set total shares for snapshot
            let globals_total_shares: U256 =
                Globals::get_globals(self, GLOBALS_TOTAL_SHARES.into());
            snapshot.total_shares = if globals_total_shares > scheduled_to_end_today {
                // globals_total_shares - scheduled_to_end_today
                globals_total_shares
                    .checked_sub(scheduled_to_end_today)
                    .ok_or(ApiError::User(ErrorCodes::Underflow as u16))
                    .unwrap_or_revert()
            } else {
                0.into()
            };

            // get inflation amount from liquidity guard
            let guard_hash = data::guard_hash();
            let declaration_liquidity_rate = Declaration::get_liquidity_rate(self);
            let liquidity_guard_inflation_amount: U256 = runtime::call_contract(
                Self::_create_hash_from_key(guard_hash),
                "get_inflation",
                runtime_args! {
                    "amount"=>declaration_liquidity_rate.as_u64()
                },
            );

            // calc inflation amount for snapshop
            snapshot.inflation_amount = snapshot
                .total_shares
                .checked_mul(parameters.precision_rate)
                .ok_or(ApiError::User(ErrorCodes::Overflow as u16))
                .unwrap_or_revert()
                .checked_div(Self::_inflation_amount(
                    total_staked_today,
                    ERC20::total_supply(self),
                    Declaration::get_total_penalties(self, _day.into()),
                    liquidity_guard_inflation_amount,
                ))
                .ok_or(ApiError::User(ErrorCodes::DivisionByZero as u16))
                .unwrap_or_revert();

            // store snapshot
            snapshots.set(&_day.into(), snapshot.clone().into_bytes().unwrap());

            // ------------------------------------
            // prepare snapshot for referrer shares
            // reusing scheduled_to_end_today variable

            // get previous days rsnapshot
            let previous_rsnapshot = rsnapshots.get(&(_day - 1).into());
            let previous_rsnapshot =
                snapshot::structs::RSnapshot::from_bytes(&previous_rsnapshot.clone())
                    .unwrap()
                    .0;

            //get schedules to end today and calculate for today
            scheduled_to_end_today = Declaration::get_referral_shares_to_end(self, _day.into())
                .checked_add(previous_rsnapshot.scheduled_to_end)
                .ok_or(ApiError::User(ErrorCodes::Overflow as u16))
                .unwrap_or_revert();

            // create new rsnapshot
            let mut rsnapshot = snapshot::structs::RSnapshot::new();

            // calc total shares
            let globals_referral_shares =
                Globals::get_globals(self, GLOBALS_REFERRAL_SHARES.into());
            rsnapshot.total_shares = if globals_referral_shares > scheduled_to_end_today {
                globals_referral_shares
                    .checked_sub(scheduled_to_end_today)
                    .ok_or(ApiError::User(ErrorCodes::Underflow as u16))
                    .unwrap_or_revert()
            } else {
                0.into()
            };

            // // calculate inflation amount
            rsnapshot.inflation_amount = rsnapshot
                .total_shares
                .checked_mul(parameters.precision_rate)
                .ok_or(ApiError::User(ErrorCodes::Overflow as u16))
                .unwrap_or_revert()
                .checked_div(Self::_referral_inflation(
                    self,
                    total_staked_today,
                    ERC20::total_supply(self),
                ))
                .ok_or(ApiError::User(ErrorCodes::DivisionByZero as u16))
                .unwrap_or_revert();

            // store referral snapshot
            rsnapshots.set(&_day.into(), rsnapshot.clone().into_bytes().unwrap());

            // ------------------------------------
            // prepare snapshot for liquidity shares
            // reusing scheduled_to_end_today variable

            // create liquidity snapshot
            let mut lsnapshot = snapshot::structs::LSnapShot::new();
            lsnapshot.total_shares = Globals::get_globals(self, GLOBALS_LIQUIDITY_SHARES.into());

            // get inflation amount
            let liquidity_guard_inflation_amount: U256 = runtime::call_contract(
                Self::_create_hash_from_key(guard_hash),
                "get_inflation",
                runtime_args! {
                    "amount"=>declaration_liquidity_rate.as_u64()
                },
            );

            lsnapshot.inflation_amount = lsnapshot
                .total_shares
                .checked_mul(parameters.precision_rate)
                .ok_or(ApiError::User(ErrorCodes::Overflow as u16))
                .unwrap_or_revert()
                .checked_div(Self::_liquidity_inflation(
                    total_staked_today,
                    ERC20::total_supply(self),
                    liquidity_guard_inflation_amount,
                ))
                .ok_or(ApiError::User(ErrorCodes::DivisionByZero as u16))
                .unwrap_or_revert();

            //set liqudity snapshot to dictionary
            lsnapshots.set(&_day.into(), lsnapshot.into_bytes().unwrap());

            //wrap up snapshotting
            Self::adjust_liquidity_rates(self);

            // increment globals.current stakeable day
            let new_current_stakeable_day = Globals::get_globals(self, GLOBALS_CURRENT_WISE_DAY.into())
                .checked_add(1.into())
                .ok_or(ApiError::User(ErrorCodes::Overflow as u16))
                .unwrap_or_revert();
            Globals::set_globals(self, GLOBALS_CURRENT_WISE_DAY.into(), new_current_stakeable_day);
        }
    }

    fn _snapshot_trigger(&self) {
        let current_stakeable_day: u64 = Timing::_current_stakeable_day(self);
        self._daily_snapshot_point(current_stakeable_day);
    }
    fn manual_daily_snapshot(&self) {
        let current_stakeable_day: u64 = Timing::_current_stakeable_day(self);
        self._daily_snapshot_point(current_stakeable_day);
    }

    fn manual_daily_snapshot_point(&self, _update_day: u64) {
        let current_stakeable_day: u64 = Timing::_current_stakeable_day(self);

        if _update_day > 0 && _update_day < current_stakeable_day {
            let current_stakeable_day_globals: U256 =
                Globals::get_globals(self, GLOBALS_CURRENT_WISE_DAY.to_string());
            if _update_day > current_stakeable_day_globals.as_u64() {
                self._daily_snapshot_point(_update_day);
            } else {
                runtime::revert(ApiError::InvalidArgument);
            }
        } else {
            runtime::revert(ApiError::InvalidArgument);
        }
    }

    fn enable_liquidity_guard(&self) {
        Declaration::set_liquidity_guard_status(self, true);
    }

    fn disable_liquidity_guard(&self) {
        Declaration::set_liquidity_guard_status(self, false);
    }

    fn adjust_liquidity_rates(&self) {
        let mut liquidity_rate: U256 = Declaration::get_liquidity_rate(self);
        let parameters_string: Vec<u8> = Declaration::get_declaration_constants(self);
        let parameters_type: declaration::parameters::ConstantParameters =
            declaration::parameters::ConstantParameters::from_bytes(&parameters_string)
                .unwrap()
                .0;
        let inflation_rate_max: U256 = parameters_type.inflation_rate_max;
        let mut inflation_rate: U256 = Declaration::get_inflation_rate(self);
        let is_liquidity_guard_active: bool = Declaration::get_liquidity_guard_status(self);

        if is_liquidity_guard_active == true && liquidity_rate < inflation_rate_max {
            liquidity_rate = liquidity_rate
                .checked_add(6.into())
                .ok_or(ApiError::User(ErrorCodes::Overflow as u16))
                .unwrap_or_revert();
            inflation_rate = inflation_rate
                .checked_sub(6.into())
                .ok_or(ApiError::User(ErrorCodes::Underflow as u16))
                .unwrap_or_revert();
            Declaration::set_liquidity_rate(self, liquidity_rate);
            Declaration::set_inflation_rate(self, inflation_rate);
        }
        if is_liquidity_guard_active == false && inflation_rate < inflation_rate_max {
            inflation_rate = inflation_rate
                .checked_add(6.into())
                .ok_or(ApiError::User(ErrorCodes::Overflow as u16))
                .unwrap_or_revert();
            liquidity_rate = liquidity_rate
                .checked_sub(6.into())
                .ok_or(ApiError::User(ErrorCodes::Underflow as u16))
                .unwrap_or_revert();
            Declaration::set_liquidity_rate(self, liquidity_rate);
            Declaration::set_inflation_rate(self, inflation_rate);
        }
    }

    fn _inflation_amount(
        _total_staked: U256,
        _total_supply: U256,
        _total_penalties: U256,
        _inflation_rate: U256,
    ) -> U256 {
        // (_total_staked + _total_supply) * 10000 / _inflation_rate + _total_penalties
        _total_staked
            .checked_add(_total_supply)
            .ok_or(ApiError::User(ErrorCodes::Overflow as u16))
            .unwrap_or_revert()
            .checked_mul(10000.into())
            .ok_or(ApiError::User(ErrorCodes::Overflow as u16))
            .unwrap_or_revert()
            .checked_div(
                _inflation_rate
                    .checked_add(_total_penalties)
                    .ok_or(ApiError::User(ErrorCodes::Overflow as u16))
                    .unwrap_or_revert(),
            )
            .ok_or(ApiError::User(ErrorCodes::DivisionByZero as u16))
            .unwrap_or_revert()
    }

    fn _referral_inflation(&self, _total_staked: U256, _total_supply: U256) -> U256 {
        let parameters_string: Vec<u8> = Declaration::get_declaration_constants(self);
        let parameters_type: declaration::parameters::ConstantParameters =
            declaration::parameters::ConstantParameters::from_bytes(&parameters_string)
                .unwrap()
                .0;

        // (_total_staked + _total_supply) * 10000 / parameters_type.referrals_rate
        _total_staked
            .checked_add(_total_supply)
            .ok_or(ApiError::User(ErrorCodes::Overflow as u16))
            .unwrap_or_revert()
            .checked_mul(10000.into())
            .ok_or(ApiError::User(ErrorCodes::Overflow as u16))
            .unwrap_or_revert()
            .checked_div(parameters_type.referrals_rate)
            .ok_or(ApiError::User(ErrorCodes::DivisionByZero as u16))
            .unwrap_or_revert()
    }

    fn _liquidity_inflation(
        _total_staked: U256,
        _total_supply: U256,
        _liquidity_rate: U256,
    ) -> U256 {
        // (_total_staked + _total_supply) * 10000 / _liquidity_rate
        _total_staked
            .checked_add(_total_supply)
            .ok_or(ApiError::User(ErrorCodes::Overflow as u16))
            .unwrap_or_revert()
            .checked_mul(10000.into())
            .ok_or(ApiError::User(ErrorCodes::Overflow as u16))
            .unwrap_or_revert()
            .checked_div(_liquidity_rate)
            .ok_or(ApiError::User(ErrorCodes::DivisionByZero as u16))
            .unwrap_or_revert()
    }

    fn get_struct_from_key(&self, key: &U256, struct_name: String) -> Vec<u8> {
        if struct_name.eq(SNAPSHOT_RSNAPSHOTS_DICT) {
            let r_snapshots = data::RSnapshotsDict::instance();
            return r_snapshots.get(&key);
        } else if struct_name.eq(SNAPSHOT_LSNAPSHOTS_DICT) {
            let l_snapshots = data::LSnapshotsDict::instance();
            return l_snapshots.get(&key);
        } else if struct_name.eq(SNAPSHOT_SNAPSHOTS_DICT) {
            let snapshots = data::SnapshotsDict::instance();
            return snapshots.get(&key);
        } else {
            runtime::revert(ApiError::UnexpectedKeyVariant);
        }
    }

    fn set_struct_from_key(&self, key: &U256, value: Vec<u8>, struct_name: String) {
        if struct_name.eq(SNAPSHOT_RSNAPSHOTS_DICT) {
            let r_snapshots = data::RSnapshotsDict::instance();
            r_snapshots.set(&key, value);
        } else if struct_name.eq(SNAPSHOT_LSNAPSHOTS_DICT) {
            let l_snapshots = data::LSnapshotsDict::instance();
            l_snapshots.set(&key, value);
        } else if struct_name.eq(SNAPSHOT_SNAPSHOTS_DICT) {
            let snapshots = data::SnapshotsDict::instance();
            snapshots.set(&key, value);
        } else {
            runtime::revert(ApiError::UnexpectedKeyVariant);
        }
    }

    // ============== Helper functions ==============================//

    fn _create_hash_from_key(key: Key) -> ContractHash {
        ContractHash::from(key.into_hash().unwrap_or_default())
    }
}
