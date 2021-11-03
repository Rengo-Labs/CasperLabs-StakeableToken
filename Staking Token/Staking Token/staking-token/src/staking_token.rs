use alloc::string::ToString;
use alloc::{string::String, vec::Vec};

use crate::data::{self};

use crate::config::*;
// use crate::config::parameters::*;
use casper_contract::contract_api::runtime;
use casper_types::{runtime_args, ApiError, ContractHash, Key, RuntimeArgs, U256};
use contract_utils::{ContractContext, ContractStorage};

pub trait STAKINGTOKEN<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(
        &mut self,
        declaration_hash: Key,
        timing_hash: Key,
        helper_hash: Key,
        bep20_hash: Key,
        snapshot_hash: Key,
        contract_hash: Key,
    ) {
        data::set_hash(contract_hash);
        data::set_declaration_hash(declaration_hash);
        data::set_timing_hash(timing_hash);
        data::set_helper_hash(helper_hash);
        data::set_bep20_hash(bep20_hash);
        data::set_snapshot_hash(snapshot_hash);
    }
}
