use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{bytesrepr::ToBytes, runtime_args, Key, RuntimeArgs, U256};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use renvm_sig::keccak256;
use timing::{
    errors::Errors,
    functions::{account_zero_address, key_to_hash, zero_address},
    globals,
    src::TIMING,
    CriticalMass, LiquidityStakeCount, ReferralCount, ReferrerLinks, Stake, StakeCount, Stakes,
};

pub trait HELPER<Storage: ContractStorage>: ContractContext<Storage> + TIMING<Storage> {
    fn init(&self) {
        TIMING::init(self);
    }

    fn _not_contract(&self, addr: Key) -> bool {
        addr.to_formatted_string().starts_with("account")
    }

    fn _to_bytes16(&self, x: U256) -> Vec<u16> {
        x.to_bytes()
            .unwrap_or_default()
            .chunks_exact(2)
            .into_iter()
            .map(|a| u16::from_ne_bytes([a[0], a[1]]))
            .collect() // Create a native endian integer value
    }

    fn generate_id(&self, x: Key, y: U256, z: u8) -> Vec<u32> {
        let mut packed: Vec<u8> = Vec::new();
        packed.append(&mut x.to_bytes().unwrap_or_default());
        packed.append(&mut y.to_bytes().unwrap_or_default());
        packed.append(&mut z.to_bytes().unwrap_or_default());
        let encode_packed: Vec<u8> = hex::encode(packed).to_bytes().unwrap_or_default();
        let bytes16: Vec<u16> = self._to_bytes16(keccak256(&encode_packed).into());
        let mut bytes32: Vec<u32> = Vec::new();
        for byte in bytes16 {
            bytes32.push(u32::from(byte));
        }
        bytes32 // Casper doesnot support u16 therefore returning u32
    }

    fn _generate_stake_id(&self, staker: Key) -> Vec<u32> {
        self.generate_id(staker, StakeCount::instance().get(&staker), 0x01)
    }

    fn _generate_referral_id(&self, referrer: Key) -> Vec<u32> {
        self.generate_id(referrer, ReferralCount::instance().get(&referrer), 0x02)
    }

    fn _generate_liquidity_stake_id(&self, staker: Key) -> Vec<u32> {
        self.generate_id(staker, LiquidityStakeCount::instance().get(&staker), 0x03)
    }

    fn stakes_pagination(&self, staker: Key, offset: U256, length: U256) -> Vec<Vec<u32>> {
        let start: U256 = if offset > 0.into() && StakeCount::instance().get(&staker) > offset {
            StakeCount::instance().get(&staker) - offset
        } else {
            StakeCount::instance().get(&staker)
        };
        let finish: U256 = if length > 0.into() && start > length {
            start - length
        } else {
            0.into()
        };
        let mut i = 0;
        let mut stakes: Vec<Vec<u32>> = Vec::with_capacity((start - finish).as_usize());
        let mut stake_index = start;
        while stake_index > finish {
            let stake_id: Vec<u32> =
                Self::generate_id(self, staker, (stake_index - 1).into(), 0x01);
            if Stakes::instance().get(&staker, &stake_id).staked_amount > 0.into() {
                stakes[i] = stake_id;
                i += 1;
            }
            stake_index -= 1.into();
        }
        stakes
    }

    fn referrals_pagination(&self, referrer: Key, offset: U256, length: U256) -> Vec<Vec<u32>> {
        let start: U256 = if offset > 0.into() && ReferralCount::instance().get(&referrer) > offset
        {
            ReferralCount::instance().get(&referrer) - offset
        } else {
            ReferralCount::instance().get(&referrer)
        };
        let finish: U256 = if length > 0.into() && start > length {
            start - length
        } else {
            0.into()
        };
        let mut i = 0;
        let mut referrals: Vec<Vec<u32>> = Vec::with_capacity((start - finish).as_usize());
        let mut r_index = start;
        while r_index > finish {
            let r_id: Vec<u32> = Self::generate_id(self, referrer, (r_index - 1).into(), 0x02);
            if self._non_zero_address(ReferrerLinks::instance().get(&referrer, &r_id).staker) {
                referrals[i] = r_id;
                i += 1;
            }
            r_index -= 1.into();
        }
        referrals
    }

    fn latest_stake_id(&self, staker: Key) -> Vec<u32> {
        if StakeCount::instance().get(&staker) == 0.into() {
            Default::default()
        } else {
            self.generate_id(
                staker,
                StakeCount::instance()
                    .get(&staker)
                    .checked_sub(1.into())
                    .unwrap_or_revert_with(Errors::SubtractionUnderflow4),
                0x01,
            )
        }
    }

    fn latest_referral_id(&self, referrer: Key) -> Vec<u32> {
        if ReferralCount::instance().get(&referrer) == 0.into() {
            Default::default()
        } else {
            self.generate_id(
                referrer,
                ReferralCount::instance()
                    .get(&referrer)
                    .checked_sub(1.into())
                    .unwrap_or_revert_with(Errors::SubtractionUnderflow5),
                0x02,
            )
        }
    }

    fn latest_liquidity_stake_id(&self, staker: Key) -> Vec<u32> {
        if LiquidityStakeCount::instance().get(&staker) == 0.into() {
            Default::default()
        } else {
            self.generate_id(
                staker,
                LiquidityStakeCount::instance()
                    .get(&staker)
                    .checked_sub(1.into())
                    .unwrap_or_revert_with(Errors::SubtractionUnderflow6),
                0x03,
            )
        }
    }

    fn _increase_stake_count(&self, staker: Key) {
        StakeCount::instance().set(&staker, StakeCount::instance().get(&staker) + 1);
    }

    fn _increase_referral_count(&self, referrer: Key) {
        ReferralCount::instance().set(&referrer, ReferralCount::instance().get(&referrer) + 1);
    }

    fn _increase_liquidity_stake_count(&self, staker: Key) {
        LiquidityStakeCount::instance()
            .set(&staker, LiquidityStakeCount::instance().get(&staker) + 1);
    }

    fn _is_mature_stake(&self, stake: Stake) -> bool {
        if stake.close_day > 0 {
            stake.final_day <= stake.close_day
        } else {
            stake.final_day <= self._current_stakeable_day()
        }
    }

    fn _not_critical_mass_referrer(&self, referrer: Key) -> bool {
        CriticalMass::instance().get(&referrer).activation_day == 0.into()
    }

    fn _stake_not_started(&self, stake: Stake) -> bool {
        if stake.close_day > 0 {
            stake.start_day > stake.close_day
        } else {
            stake.start_day > self._current_stakeable_day()
        }
    }

    fn _stake_ended(&self, stake: Stake) -> bool {
        !stake.is_active || self._is_mature_stake(stake)
    }

    fn _days_left(&self, stake: Stake) -> U256 {
        if !stake.is_active {
            self._days_diff(stake.close_day.into(), stake.final_day.into())
        } else {
            self._days_diff(self._current_stakeable_day().into(), stake.final_day.into())
        }
    }

    fn _days_diff(&self, start_date: U256, end_date: U256) -> U256 {
        if start_date > end_date {
            0.into()
        } else {
            end_date
                .checked_sub(start_date)
                .unwrap_or_revert_with(Errors::SubtractionUnderflow7)
        }
    }

    fn _calculation_day(&self, stake: Stake) -> U256 {
        if U256::from(stake.final_day) > globals().current_stakeable_day {
            globals().current_stakeable_day
        } else {
            stake.final_day.into()
        }
    }

    fn _starting_day(&self, stake: Stake) -> U256 {
        if stake.scrape_day == 0.into() {
            stake.start_day.into()
        } else {
            stake.scrape_day
        }
    }

    fn _not_future(&self, day: U256) -> bool {
        day <= self._current_stakeable_day().into()
    }

    fn _not_past(&self, day: U256) -> bool {
        day >= self._current_stakeable_day().into()
    }

    fn _non_zero_address(&self, address: Key) -> bool {
        address != account_zero_address() && address != zero_address()
    }

    fn _get_lock_days(&self, stake: Stake) -> U256 {
        if stake.lock_days > 1 {
            (stake.lock_days - 1).into()
        } else {
            1.into()
        }
    }

    fn _prepare_path(
        &self,
        token_address: Key,
        synthetic_address: Key,
        stakeable_address: Key,
    ) -> Vec<Key> {
        let mut path: Vec<Key> = Vec::with_capacity(3);
        path[0] = token_address;
        path[1] = synthetic_address;
        path[2] = stakeable_address;
        path
    }

    fn _transfer(&self, token: Key, to: Key, value: U256) {
        let ret: Result<(), u32> = runtime::call_versioned_contract(
            key_to_hash(token, Errors::InvalidHash2),
            None,
            "transfer",
            runtime_args! {
                "recipient" => to,
                "amount" => value
            },
        );
        if ret.is_err() {
            runtime::revert(Errors::TransferFailed);
        }
    }

    fn _transfer_from(&self, token: Key, from: Key, to: Key, value: U256) {
        let ret: Result<(), u32> = runtime::call_versioned_contract(
            key_to_hash(token, Errors::InvalidHash3),
            None,
            "transfer_from",
            runtime_args! {
                "owner" => from,
                "recipient" => to,
                "amount" => value
            },
        );
        if ret.is_err() {
            runtime::revert(Errors::TransferFromFailed);
        }
    }
}
