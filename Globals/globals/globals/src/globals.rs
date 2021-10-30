extern crate alloc;

use casper_types::{
    contracts::{ContractPackageHash},Key,  U256};
use contract_utils::{ContractContext, ContractStorage};

use crate::data::{self, GlobalsStruct};

pub trait Globals<Storage: ContractStorage>: ContractContext<Storage> 
{
    // Will be called by constructor
    fn init(&mut self, contract_hash: Key, package_hash: ContractPackageHash) 
    {
        data::set_package_hash(package_hash);
        data::set_self_hash(contract_hash);

        let mut number:U256 = U256::from(10).pow(15.into());            // 10 ^ 15                        
        number = U256::from(100) * number;                              // 100 * (10 ^ 15)  =  100E15;

        GlobalsStruct::init();
        GlobalsStruct::instance().set(data::SHARE_PRICE, number);
    }

    fn increase_globals(&mut self, _staked: U256, _shares: U256, _rshares: U256)
    {
        let globals : GlobalsStruct = GlobalsStruct::instance();
        
        globals.set(data::TOTAL_STAKED, globals.get(data::TOTAL_STAKED).checked_add(_staked).unwrap());       // globals.totalStaked = globals.totalStaked.add(_staked)
        globals.set(data::TOTAL_SHARES, globals.get(data::TOTAL_SHARES).checked_add(_shares).unwrap());       // globals.total_shares = globals.totalShares.add(_shares)

        if _rshares > 0.into()
        {
            globals.set(data::REFERRAL_SHARES, globals.get(data::REFERRAL_SHARES).checked_add(_rshares).unwrap()); 
        }
    }

    fn decrease_globals(&mut self, _staked: U256, _shares: U256, _rshares: U256)
    {
        let globals : GlobalsStruct = GlobalsStruct::instance();

        globals.set(data::TOTAL_STAKED, if globals.get(data::TOTAL_STAKED) > _staked { globals.get(data::TOTAL_STAKED) - _staked } else {0.into()});
        globals.set(data::TOTAL_SHARES, if globals.get(data::TOTAL_SHARES) > _shares { globals.get(data::TOTAL_SHARES) - _shares } else {0.into()});

        if _rshares > 0.into() 
        {
            globals.set(data::REFERRAL_SHARES, if globals.get(data::REFERRAL_SHARES) > _rshares { globals.get(data::REFERRAL_SHARES) - _rshares } else {0.into()});
        }
    }
}