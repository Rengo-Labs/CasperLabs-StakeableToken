pub mod structs {
    use casper_types::U256;
    use casper_types_derive::{CLTyped, FromBytes, ToBytes};
    extern crate alloc;
    use alloc::vec::Vec;
    // regular shares
    #[derive(Clone, CLTyped, ToBytes, FromBytes)]
    pub struct Snapshot {
        pub total_shares: U256,
        pub inflation_amount: U256,
        pub scheduled_to_end: U256,
    }

    impl Snapshot {
        pub fn new() -> Snapshot {
            Snapshot {
                total_shares: 0.into(),
                inflation_amount: 0.into(),
                scheduled_to_end: 0.into(),
            }
        }
    }
    // referral shares
    #[derive(Clone, CLTyped, ToBytes, FromBytes)]
    pub struct RSnapshot {
        pub total_shares: U256,
        pub inflation_amount: U256,
        pub scheduled_to_end: U256,
    }

    impl RSnapshot {
        pub fn new() -> RSnapshot {
            RSnapshot {
                total_shares: 0.into(),
                inflation_amount: 0.into(),
                scheduled_to_end: 0.into(),
            }
        }
    }
    // liquidity shares
    #[derive(Clone, CLTyped, ToBytes, FromBytes)]
    pub struct LSnapShot {
        pub total_shares: U256,
        pub inflation_amount: U256,
    }

    impl LSnapShot {
        pub fn new() -> LSnapShot {
            LSnapShot {
                total_shares: 0.into(),
                inflation_amount: 0.into(),
            }
        }
    }
}
