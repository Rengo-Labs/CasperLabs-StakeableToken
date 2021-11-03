use crate::config::*;
use crate::data::{self};
use alloc::{format, string::String};
use casper_contract::contract_api::runtime;
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},
    runtime_args, ApiError, Key, RuntimeArgs, U128, U256,
};
use contract_utils::{ContractContext, ContractStorage};

pub trait Snapshot<Storage: ContractStorage>: ContractContext<Storage> {
    // Will be called by constructor
    fn init(
        &mut self,
        contract_hash: Key,
        package_hash: ContractPackageHash,
        timing_contract_hash: Key,
        declaration_contract_hash: Key,
        globals_contract_hash: Key,
        helper_contract_hash: Key,
        sbnb_contract_hash: Key,
        pair_contract_hash: Key,
        bep20_contract_hash: Key,
        guard_contract_hash: Key,
    ) {
        data::set_package_hash(package_hash);
        data::set_self_hash(contract_hash);
        data::set_helper_hash(helper_contract_hash);
        data::set_timing_hash(timing_contract_hash);
        data::set_declaration_hash(declaration_contract_hash);
        data::set_globals_hash(globals_contract_hash);
        data::set_sbnb_hash(sbnb_contract_hash);
        data::set_pair_hash(pair_contract_hash);
        data::set_bep20_hash(bep20_contract_hash);
        data::set_guard_hash(guard_contract_hash);
        data::SnapshotsDict::init();
        data::RSnapshotsDict::init();
        data::LSnapshotsDict::init();
    }

    fn _liquidity_guard_trigger(&self) {
        let pair_hash = data::pair_hash();
        let sbnb_hash = data::sbnb_hash();
        let bep20_hash = data::bep20_hash();
        let guard_hash = data::guard_hash();

        let total_supply: U256 = runtime::call_contract(
            Self::_create_hash_from_key(bep20_hash),
            "total_supply",
            runtime_args! {},
        );
        let liquidity_guard_status: bool = runtime::call_contract(
            Self::_create_hash_from_key(guard_hash),
            "get_liquidity_guard_status",
            runtime_args! {},
        );
        // third return value is block_timestamp_last
        let (reserve_a, reserve_b, _): (U128, U128, u64) = runtime::call_contract(
            Self::_create_hash_from_key(pair_hash),
            "get_reserves",
            runtime_args! {},
        );

        let token1: Key = runtime::call_contract(
            Self::_create_hash_from_key(pair_hash),
            "token1",
            runtime_args! {},
        );

        let on_pancake: U256 = if token1.eq(&sbnb_hash) {
            U256::from(reserve_a.as_u128())
        } else {
            U256::from(reserve_b.as_u128())
        };

        let ratio: U256 = if total_supply == 0.into() {
            0.into()
        } else {
            (on_pancake * 200) / total_supply
        };

        if ratio < 40.into() && liquidity_guard_status == false {
            self._enable_liquidity_guard();
        }

        if ratio > 60.into() && liquidity_guard_status == true {
            self._disable_liquidity_guard();
        }
    }

    fn _daily_snapshot_point(&self, _update_day: u64) {
        self._liquidity_guard_trigger();

        let globals_hash: Key = data::globals_hash();
        let declaration_hash: Key = data::declaration_hash();
        let bep20_hash: Key = data::bep20_hash();
        let guard_hash = data::guard_hash();

        let mut total_penalties: U256;
        let mut temp: U256;

        let total_staked_today: U256 = runtime::call_contract(
            Self::_create_hash_from_key(globals_hash),
            "get_globals",
            runtime_args! {
                "field"=>data::TOTAL_STAKED
            },
        );

        let liquidity_rate: U256 = runtime::call_contract(
            Self::_create_hash_from_key(declaration_hash),
            "get_liquidity_rate",
            runtime_args! {},
        );
        let liquidity_shares: U256 = runtime::call_contract(
            Self::_create_hash_from_key(globals_hash),
            "get_globals",
            runtime_args! {
                "field"=>data::LIQUIDITY_SHARES
            },
        );

        let total_shares: U256 = runtime::call_contract(
            Self::_create_hash_from_key(globals_hash),
            "get_globals",
            runtime_args! {
                "field"=>data::TOTAL_SHARES
            },
        );

        let referral_shares: U256 = runtime::call_contract(
            Self::_create_hash_from_key(globals_hash),
            "get_globals",
            runtime_args! {
                "field"=>data::REFERRAL_SHARES
            },
        );

        let mut day: U256 = runtime::call_contract(
            Self::_create_hash_from_key(globals_hash),
            "get_globals",
            runtime_args! {
                "field"=>data::CURRENT_WISE_DAY
            },
        );
        let mut temp_str: String = runtime::call_contract(
            Self::_create_hash_from_key(declaration_hash),
            "get_declaration_constants",
            runtime_args! {},
        );
        let total_supply: U256 = runtime::call_contract(
            Self::_create_hash_from_key(bep20_hash),
            "total_supply",
            runtime_args! {},
        );

        let parameters: Structs::ConstantParameters = serde_json::from_str(&temp_str).unwrap();
        let snapshots = data::SnapshotsDict::instance();
        while day < _update_day.into() {
            // ------------------------------------
            // prepare snapshot for regular shares
            // reusing scheduledToEndToday variable
            total_penalties = runtime::call_contract(
                Self::_create_hash_from_key(declaration_hash),
                "get_total_penalties",
                runtime_args! {"key"=>day},
            );

            let prev_snapshot: Structs::Snapshot =
                serde_json::from_str(&snapshots.get(&(day - U256::from(1)))).unwrap();

            temp = runtime::call_contract(
                Self::_create_hash_from_key(declaration_hash),
                "get_scheduled_to_end",
                runtime_args! {"key"=>day},
            );
            let mut scheduled_to_end_today: U256 = temp - prev_snapshot.scheduled_to_end;

            let mut todays_snapshot: Structs::Snapshot =
                serde_json::from_str(&snapshots.get(&day)).unwrap();
            todays_snapshot.scheduled_to_end = scheduled_to_end_today;

            todays_snapshot.total_shares = if total_shares > scheduled_to_end_today {
                total_shares - scheduled_to_end_today
            } else {
                U256::from(0)
            };
            let inflation_rate_guard: U256 = runtime::call_contract(
                Self::_create_hash_from_key(guard_hash),
                "get_inflation",
                runtime_args! {
                    "liquidity_rate"=>liquidity_rate
                },
            );

            todays_snapshot.inflation_amount = (todays_snapshot.total_shares
                * parameters.precision_rate)
                / Self::_inflation_amount(
                    total_staked_today,
                    total_supply,
                    total_penalties,
                    inflation_rate_guard,
                ); //(Self::_inflation_amount(total_staked_today, ));

            temp_str = serde_json::to_string(&todays_snapshot).unwrap();
            snapshots.set(&day, temp_str);

            // ------------------------------------
            // prepare snapshot for referrer shares
            // reusing scheduledToEndToday variable

            temp = runtime::call_contract(
                Self::_create_hash_from_key(declaration_hash),
                "get_referral_shares_to_end",
                runtime_args! {"key"=>day},
            );

            let r_snapshots = data::RSnapshotsDict::instance();
            temp_str = r_snapshots.get(&(day - U256::from(1)));
            let mut r_snapshot: Structs::RSnapshot = serde_json::from_str(&temp_str).unwrap();
            scheduled_to_end_today = temp + r_snapshot.scheduled_to_end;

            temp_str = r_snapshots.get(&(day));
            r_snapshot = serde_json::from_str(&temp_str).unwrap();

            r_snapshot.scheduled_to_end = scheduled_to_end_today;

            r_snapshot.total_shares = if referral_shares > scheduled_to_end_today {
                referral_shares - scheduled_to_end_today
            } else {
                U256::from(0)
            };

            r_snapshot.inflation_amount = (r_snapshot.total_shares * parameters.precision_rate)
                / Self::_referral_inflation(total_staked_today, total_supply);

            temp_str = serde_json::to_string(&r_snapshot).unwrap();
            r_snapshots.set(&day, temp_str);

            // ------------------------------------
            // prepare snapshot for liquidity shares
            // reusing scheduledToEndToday variable
            let l_snapshots = data::LSnapshotsDict::instance();

            temp_str = l_snapshots.get(&day);
            let mut l_snapshot: Structs::LSnapShot = serde_json::from_str(&temp_str).unwrap();

            l_snapshot.total_shares = liquidity_shares;

            // VERIFY that this endpoint exists
            let inflation: U256 = runtime::call_contract(
                Self::_create_hash_from_key(guard_hash),
                "get_inflation",
                runtime_args! {
                    "liquidity_rate"=>liquidity_rate
                },
            );

            l_snapshot.inflation_amount = (l_snapshot.total_shares * parameters.precision_rate)
                / Self::_liquidity_inflation(total_staked_today, total_supply, inflation);
            temp_str = serde_json::to_string(&l_snapshot).unwrap();
            l_snapshots.set(&day, temp_str);

            self._adjust_liquidity_rates();

            // VERIFY the day=currentWiseDay mechanic of the loop
            // loop invariant
            day = day + 1;
            let () = runtime::call_contract(
                Self::_create_hash_from_key(globals_hash),
                "set_globals",
                runtime_args! {
                    data::CURRENT_WISE_DAY=>day
                },
            );
        }
    }

    fn _manual_daily_snapshot(&self) {
        let timing_hash: Key = data::timing_hash();
        let current_wise_day: u64 = runtime::call_contract(
            ContractHash::from(timing_hash.into_hash().unwrap_or_default()),
            "current_wise_day",
            runtime_args! {},
        );

        self._daily_snapshot_point(current_wise_day);
    }

    fn _manual_daily_snapshot_point(&self, _update_day: u64) {
        let timing_hash: Key = data::timing_hash();
        let current_wise_day: u64 = runtime::call_contract(
            ContractHash::from(timing_hash.into_hash().unwrap_or_default()),
            "current_wise_day",
            runtime_args! {},
        );

        if _update_day > 0 && _update_day < current_wise_day {
            let globals_hash: Key = data::globals_hash();
            let current_wise_day_globals: u64 = runtime::call_contract(
                Self::_create_hash_from_key(globals_hash),
                "get_globals",
                runtime_args! {
                    "field"=>"current_wise_day"
                },
            );

            if _update_day > current_wise_day_globals {
                self._daily_snapshot_point(_update_day);
            } else {
                runtime::revert(ApiError::InvalidArgument);
            }
        } else {
            runtime::revert(ApiError::InvalidArgument);
        }
    }

    fn _enable_liquidity_guard(&self) {
        Self::_set_liquidity_guard_status(true);
    }

    fn _disable_liquidity_guard(&self) {
        Self::_set_liquidity_guard_status(false);
    }

    fn _adjust_liquidity_rates(&self) {
        let declaration_hash: Key = data::declaration_hash();

        let mut liquidity_rate: U256 = runtime::call_contract(
            ContractHash::from(declaration_hash.into_hash().unwrap_or_default()),
            "get_liquidity_rate",
            runtime_args! {},
        );

        let parameters_string: String = runtime::call_contract(
            Self::_create_hash_from_key(declaration_hash),
            "get_declaration_constants",
            runtime_args! {},
        );
        let parameters_type: Structs::ConstantParameters =
            serde_json::from_str(&parameters_string).unwrap();
        let inflation_rate_max: U256 = parameters_type.inflation_rate_max;

        let mut inflation_rate: U256 = runtime::call_contract(
            ContractHash::from(declaration_hash.into_hash().unwrap_or_default()),
            "get_inflation_rate",
            runtime_args! {},
        );

        let is_liquidity_guard_active: bool = runtime::call_contract(
            ContractHash::from(declaration_hash.into_hash().unwrap_or_default()),
            "get_liquidity_guard_status",
            runtime_args! {},
        );

        if is_liquidity_guard_active == true && liquidity_rate < inflation_rate_max {
            liquidity_rate = liquidity_rate + 6;
            inflation_rate = inflation_rate - 6;

            Self::_set_constant_in_contract(declaration_hash, &"liquidity_rate", liquidity_rate);
            Self::_set_constant_in_contract(declaration_hash, &"inflation_rate", inflation_rate);
        }
        if is_liquidity_guard_active == false && inflation_rate < inflation_rate_max {
            inflation_rate = inflation_rate + 6;
            liquidity_rate = liquidity_rate - 6;

            Self::_set_constant_in_contract(declaration_hash, &"liquidity_rate", liquidity_rate);
            Self::_set_constant_in_contract(declaration_hash, &"inflation_rate", inflation_rate);
        }
    }

    // DONE
    fn _inflation_amount(
        _total_staked: U256,
        _total_supply: U256,
        _total_penalties: U256,
        _inflation_rate: U256,
    ) -> U256 {
        (_total_staked + _total_supply) * 10000 / _inflation_rate + _total_penalties
    }

    fn _referral_inflation(_total_staked: U256, _total_supply: U256) -> U256 {
        let declaration_hash: Key = data::declaration_hash();
        let parameters_string: String = runtime::call_contract(
            Self::_create_hash_from_key(declaration_hash),
            "get_declaration_constants",
            runtime_args! {},
        );
        let parameters_type: Structs::ConstantParameters =
            serde_json::from_str(&parameters_string).unwrap();

        (_total_staked + _total_supply) * 10000 / parameters_type.referrals_rate
    }

    // DONE
    fn _liquidity_inflation(
        _total_staked: U256,
        _total_supply: U256,
        _liquidity_rate: U256,
    ) -> U256 {
        (_total_staked + _total_supply) * 10000 / _liquidity_rate
    }

    fn _get_struct_from_key(&self, key: &U256, struct_name: String) -> String {
        if struct_name.eq(data::RSNAPSHOTS_DICT) {
            let r_snapshots = data::RSnapshotsDict::instance();
            return r_snapshots.get(&key);
        } else if struct_name.eq(data::LSNAPSHOTS_DICT) {
            let l_snapshots = data::LSnapshotsDict::instance();
            return l_snapshots.get(&key);
        } else if struct_name.eq(data::SNAPSHOTS_DICT) {
            let snapshots = data::SnapshotsDict::instance();
            return snapshots.get(&key);
        } else {
            String::from("")
        }
    }

    fn _set_struct_from_key(&self, key: &U256, value: String, struct_name: String) {
        if struct_name.eq(data::RSNAPSHOTS_DICT) {
            let r_snapshots = data::RSnapshotsDict::instance();
            return r_snapshots.set(&key, value);
        } else if struct_name.eq(data::LSNAPSHOTS_DICT) {
            let l_snapshots = data::LSnapshotsDict::instance();
            return l_snapshots.set(&key, value);
        } else if struct_name.eq(data::SNAPSHOTS_DICT) {
            let snapshots = data::SnapshotsDict::instance();
            return snapshots.set(&key, value);
        } else {
            runtime::revert(ApiError::UnexpectedKeyVariant);
        }
    }

    // ============== Helper functions ==============================//

    // verify against muzahir's implementation
    fn _set_constant_in_contract(contract_hash: Key, constant_name: &str, constant_value: U256) {
        let args: RuntimeArgs = runtime_args! {
            constant_name => constant_value
        };
        let () = runtime::call_contract(
            Self::_create_hash_from_key(contract_hash),
            &format!("set_{}", constant_name),
            args,
        );
    }

    fn _set_liquidity_guard_status(status: bool) {
        let declaration_hash: Key = data::declaration_hash();
        let args: RuntimeArgs = runtime_args! {
            "status"=>status
        };
        let () = runtime::call_contract(
            Self::_create_hash_from_key(declaration_hash),
            "set_liquidity_guard_status",
            args,
        );
    }

    fn _create_hash_from_key(key: Key) -> ContractHash {
        ContractHash::from(key.into_hash().unwrap_or_default())
    }
}
