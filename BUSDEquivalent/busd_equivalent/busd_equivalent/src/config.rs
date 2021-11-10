pub mod structs {
    use casper_types::{
        bytesrepr::{FromBytes, ToBytes},
        CLType, CLTyped, Key, U256,
    };
    extern crate serde;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct Constants {
        pub _decimals: u32,
        pub yodas_per_wise: U256,
        pub wbnb: Key,
        pub sbnb: Key,
        pub busd: Key,
        pub wise: Key,
        pub router: Key,
        latest_busd_equivalent: U256,
    }
}
