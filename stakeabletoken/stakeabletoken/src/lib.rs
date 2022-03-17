#![no_std]

extern crate alloc;

pub mod config;
pub mod data;
pub mod stakeabletoken;

pub use stakeabletoken::StakeableToken;
