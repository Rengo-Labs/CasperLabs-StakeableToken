use crate::data::{self, Allowances, Balances};
use alloc::{string::String, vec::Vec};
use casper_contract::contract_api::runtime;
use casper_contract::contract_api::storage;
use casper_types::ApiError;
use casper_types::{Key, URef, U256};
use contract_utils::{ContractContext, ContractStorage};

use crate::alloc::string::ToString;
use alloc::collections::BTreeMap;

#[repr(u16)]
pub enum Error {
    NotOwner = 1,
    NotSBNB = 2,
    ZERO = 3,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

pub enum BEP20Event {
    Approval {
        owner: Key,
        spender: Key,
        value: U256,
    },
    Transfer {
        from: Key,
        to: Key,
        value: U256,
    },
}

impl BEP20Event {
    pub fn type_name(&self) -> String {
        match self {
            BEP20Event::Approval {
                owner: _,
                spender: _,
                value: _,
            } => "approve",
            BEP20Event::Transfer {
                from: _,
                to: _,
                value: _,
            } => "transfer",
        }
        .to_string()
    }
}

pub trait BEP20<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&self, name: String, symbol: String) {
        data::set_owner(self.get_caller());
        data::set_name(name);
        data::set_symbol(symbol);
        data::set_decimals(18.into());
        data::set_total_supply(0.into());
        Balances::init();
        Allowances::init();
    }

    fn set_sbnb(&self, sbnb: Key) {
        if self.get_caller().to_formatted_string() != data::owner().to_formatted_string() {
            runtime::revert(ApiError::from(Error::NotOwner));
        }
        data::set_sbnb(sbnb);
    }

    fn is_sbnb(&self) {
        if self.get_caller().to_formatted_string() != data::sbnb().to_formatted_string() {
            runtime::revert(ApiError::from(Error::NotSBNB));
        }
    }

    fn sbnb_burn(&self, account: Key, amount: U256) {
        self.is_sbnb();
        self._burn(account, amount);
    }

    fn sbnb_mint(&self, account: Key, amount: U256) {
        self.is_sbnb();
        self._mint(account, amount);
    }

    fn sbnb_approve(&self, owner: Key, spender: Key, amount: U256) {
        self.is_sbnb();
        self._approve(owner, spender, amount);
    }

    fn name(&self) -> String {
        data::name()
    }

    fn symbol(&self) -> String {
        data::symbol()
    }

    fn decimals(&self) -> u8 {
        data::decimals()
    }

    fn total_supply(&self) -> U256 {
        data::total_supply()
    }

    fn balance_of(&self, owner: Key) -> U256 {
        Balances::instance().get(&owner)
    }

    fn transfer(&self, recipient: Key, amount: U256) -> bool {
        self._transfer(self.get_caller(), recipient, amount);
        true
    }

    fn allowance(&self, owner: Key, spender: Key) -> U256 {
        Allowances::instance().get(&owner, &spender)
    }

    fn approve(&self, spender: Key, amount: U256) -> bool {
        self._approve(self.get_caller(), spender, amount);
        true
    }

    fn transfer_from(&self, owner: Key, recipient: Key, amount: U256) -> bool {
        self._approve(
            owner,
            self.get_caller(),
            Allowances::instance().get(&owner, &self.get_caller()) - amount,
        );
        self._transfer(owner, recipient, amount);
        true
    }

    fn _transfer(&self, sender: Key, recipient: Key, amount: U256) {
        let zero_addr: Key = Key::from_formatted_str(
            "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
        )
        .unwrap();
        if sender.to_formatted_string() == zero_addr.to_formatted_string() {
            runtime::revert(ApiError::from(Error::ZERO));
        }
        if recipient.to_formatted_string() == zero_addr.to_formatted_string() {
            runtime::revert(ApiError::from(Error::ZERO));
        }

        Balances::instance().set(&sender, Balances::instance().get(&sender) - amount);
        Balances::instance().set(&recipient, Balances::instance().get(&recipient) + amount);

        self.emit(&BEP20Event::Transfer {
            from: sender,
            to: recipient,
            value: amount,
        });
    }

    fn _mint(&self, account: Key, amount: U256) {
        let zero_addr: Key = Key::from_formatted_str(
            "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
        )
        .unwrap();
        if account.to_formatted_string() == zero_addr.to_formatted_string() {
            runtime::revert(ApiError::from(Error::ZERO));
        }

        data::set_total_supply(data::total_supply() + amount);

        Balances::instance().set(&account, Balances::instance().get(&account) + amount);

        self.emit(&BEP20Event::Transfer {
            from: zero_addr,
            to: account,
            value: amount,
        });
    }

    fn burn(&self, amount: U256) {
        self._burn(self.get_caller(), amount);
    }

    fn _burn(&self, account: Key, amount: U256) {
        let zero_addr: Key = Key::from_formatted_str(
            "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
        )
        .unwrap();
        if account.to_formatted_string() == zero_addr.to_formatted_string() {
            runtime::revert(ApiError::from(1)); // ZERO ADDRESS
        }

        Balances::instance().set(&account, Balances::instance().get(&account) - amount);
        data::set_total_supply(data::total_supply() - amount);
        let address_0: Key = Key::from_formatted_str(
            "account-hash-0000000000000000000000000000000000000000000000000000000000000000",
        )
        .unwrap();
        self.emit(&BEP20Event::Transfer {
            from: account,
            to: address_0,
            value: amount,
        });
    }

    fn _approve(&self, owner: Key, spender: Key, amount: U256) {
        let zero_addr: Key = Key::from_formatted_str(
            "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
        )
        .unwrap();
        if owner.to_formatted_string() == zero_addr.to_formatted_string() {
            runtime::revert(ApiError::from(1)); // ZERO ADDRESS
        }
        if spender.to_formatted_string() == zero_addr.to_formatted_string() {
            runtime::revert(ApiError::from(1)); // ZERO ADDRESS
        }

        Allowances::instance().set(&owner, &spender, amount);
        self.emit(&BEP20Event::Approval {
            owner: self.get_caller(),
            spender: spender,
            value: amount,
        });
    }

    fn emit(&self, bep20_event: &BEP20Event) {
        let mut events = Vec::new();
        let package = data::get_contract_package_hash();
        match bep20_event {
            BEP20Event::Approval {
                owner,
                spender,
                value,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", bep20_event.type_name());
                event.insert("owner", owner.to_string());
                event.insert("spender", spender.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
            BEP20Event::Transfer { from, to, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", bep20_event.type_name());
                event.insert("from", from.to_string());
                event.insert("to", to.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
        };
        for event in events {
            let _: URef = storage::new_uref(event);
        }
    }
}
