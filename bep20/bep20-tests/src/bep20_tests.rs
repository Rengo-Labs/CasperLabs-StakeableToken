use casper_engine_test_support::AccountHash;
use casper_types::{Key, U256};
use test_env::{Sender, TestEnv};

use crate::bep20_instance::BEP20Instance;

const NAME: &str = "BEP20";
const SYMBOL: &str = "BEP";
const DECIMALS: u8 = 18;

fn deploy() -> (TestEnv, BEP20Instance, AccountHash) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let token = BEP20Instance::new(
        &env,
        NAME,
        Sender(owner),
        NAME,
        SYMBOL,
        DECIMALS,
        (404000000000000000 as u128).into(),
    );
    (env, token, owner)
}

#[test]
fn test_bep_deploy() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    assert_eq!(token.name(), NAME);
    assert_eq!(token.symbol(), SYMBOL);
    assert_eq!(token.decimals(), DECIMALS);
    assert_eq!(token.total_supply(), (404000000000000000 as u128).into());
    assert_eq!(token.balance_of(owner), (404000000000000000 as u128).into());
    assert_eq!(token.balance_of(user), 0.into());
    assert_eq!(token.allowance(owner, user), 0.into());
    assert_eq!(token.allowance(user, owner), 0.into());
}

#[test]
fn test_bep_transfer() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    let amount = 10.into();
    token.transfer(Sender(owner), user, amount);
    assert_eq!(
        token.balance_of(owner),
        U256::from(404000000000000000 as u128) - amount
    );
    assert_eq!(token.balance_of(user), amount);
}

#[test]
#[should_panic]
fn test_bep_transfer_too_much() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    let amount = U256::from(404000000000000000 as u128) + U256::one();
    token.transfer(Sender(owner), user, amount);
}

#[test]
fn test_bep_approve() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    let amount = 10.into();
    token.approve(Sender(owner), user.into(), amount);
    assert_eq!(token.balance_of(owner), (404000000000000000 as u128).into());
    assert_eq!(token.balance_of(user), 0.into());
    assert_eq!(token.allowance(owner, user), amount);
    assert_eq!(token.allowance(user, owner), 0.into());
}

#[test]
fn test_bep_mint() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    let amount: U256 = 10.into();
    token.mint(Sender(owner), user.into(), amount);
    assert_eq!(token.balance_of(owner), (404000000000000000 as u128).into());
    assert_eq!(token.balance_of(user), amount);
    assert_eq!(token.balance_of(user), 10.into());
}

#[test]
fn test_bep_burn() {
    let (env, token, owner) = deploy();
    let user = env.next_user();
    let amount = 10.into();
    assert_eq!(
        token.balance_of(owner),
        U256::from(404000000000000000 as u128)
    );
    token.burn(Sender(owner), owner, amount);
    assert_eq!(
        token.balance_of(owner),
        U256::from(404000000000000000 as u128) - amount
    );
    assert_eq!(token.balance_of(user), 0.into());
}

#[test]
fn test_bep_transfer_from() {
    let (env, token, owner) = deploy();
    let spender = env.next_user();
    let recipient = env.next_user();
    let allowance: U256 = 10.into();
    let amount: U256 = 3.into();
    token.approve(Sender(owner), spender.into(), allowance);
    token.transfer_from(Sender(spender), owner, recipient, amount);
    assert_eq!(
        token.balance_of(owner),
        U256::from(404000000000000000 as u128) - amount
    );
    assert_eq!(token.balance_of(spender), 0.into());
    assert_eq!(token.balance_of(recipient), amount);
    assert_eq!(token.allowance(owner, spender), allowance - amount);
}

#[test]
#[should_panic]
fn test_bep_transfer_from_too_much() {
    let (env, token, owner) = deploy();
    let spender = env.next_user();
    let recipient = env.next_user();
    let allowance = 10.into();
    let amount = 12.into();
    token.approve(Sender(owner), spender.into(), allowance);
    token.transfer_from(Sender(spender), owner, recipient, amount);
}

#[test]
#[should_panic]
fn test_calling_construction() {
    let (_, token, owner) = deploy();
    token.constructor(
        Sender(owner),
        NAME,
        SYMBOL,
        DECIMALS,
        (404000000000000000 as u128).into(),
    );
}
