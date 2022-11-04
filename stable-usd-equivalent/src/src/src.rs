use crate::data::set_stakeable;
use casper_types::Key;
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use common::functions::{set_contract_hash, set_package_hash};
use referral_token::{set_latest_stable_usd_equivalent, src::REFERRALTOKEN};

pub trait STABLEUSDEQUIVALENT<Storage: ContractStorage>:
    ContractContext<Storage> + REFERRALTOKEN<Storage>
{
    fn init(&mut self, contract_hash: Key, package_hash: Key, stakeable: Key) {
        set_stakeable(stakeable);
        set_contract_hash(contract_hash);
        set_package_hash(package_hash);
    }

    fn update_stable_usd_equivalent(&self) {
        set_latest_stable_usd_equivalent(self._get_stable_usd_equivalent());
    }
}
