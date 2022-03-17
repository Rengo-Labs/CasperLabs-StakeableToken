pub mod structs{
    use casper_types::{U256, Key};
    use casper_types_derive::{CLTyped, FromBytes, ToBytes};
    extern crate alloc;
    use alloc::vec::Vec;
    // regular shares
    #[derive(Clone, CLTyped, ToBytes, FromBytes)]
    pub struct StakeInfo{
            pub staker: Key,
            pub stake_id: Vec<u32>,
            pub referrer_shares: U256,
            pub referral_interest: U256,
            pub is_active_referral: bool,
            pub is_active_stake: bool,
            pub is_mature_stake: bool,
            pub is_ended_stake: bool,
    }
}