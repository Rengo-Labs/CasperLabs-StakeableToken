use crate::data::*;
use casper_contract::{
    contract_api::{runtime, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{runtime_args, Key, RuntimeArgs, URef, U256, U512};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use common::{
    errors::Errors,
    functions::{set_package_hash, *},
};
use liquidity_token::*;
use num_traits::AsPrimitive;

pub trait STAKEABLETOKEN<Storage: ContractStorage>:
    ContractContext<Storage> + LIQUIDITYTOKEN<Storage>
{
    #[allow(clippy::too_many_arguments)]
    fn init(
        &mut self,
        stable_usd: Key,
        scspr: Key,
        wcspr: Key,
        uniswap_router: Key,
        uniswap_factory: Key,
        uniswap_pair: Key,
        liquidity_guard: Key,
        contract_hash: Key,
        package_hash: Key,
    ) {
        LIQUIDITYTOKEN::init(self);
        ERC20::init(
            self,
            "Stakeable Token".into(),
            "STAKEABLE".into(),
            9,
            0.into(),
            contract_hash,
            key_to_hash(package_hash, Errors::InvalidHash10),
        );
        set_transformer_gate_keeper(self.get_caller());

        set_stable_usd(stable_usd);
        set_scspr(scspr);
        set_wcspr(wcspr);
        set_uniswap_router(uniswap_router);
        set_uniswap_factory(uniswap_factory);
        set_uniswap_pair(uniswap_pair);
        set_liquidity_guard(liquidity_guard);
        set_contract_hash(contract_hash);
        set_package_hash(package_hash);
        set_purse(system::create_purse());
    }

    fn set_liquidity_transfomer(&self, immutable_transformer: Key, transformer_purse: URef) {
        if transformer_gate_keeper() != self.get_caller() {
            runtime::revert(Errors::TransformerDefined);
        }
        set_liquidity_transformer(immutable_transformer, transformer_purse);
        set_transformer_gate_keeper(account_zero_address());
    }

    fn mint_supply(&mut self, investor_address: Key, amount: U256) {
        if self.get_caller() != liquidity_transformer().0 {
            runtime::revert(Errors::WrongTransformer);
        }
        self.mint(investor_address, amount);
    }

    fn create_stake_with_cspr(
        &mut self,
        lock_days: u64,
        referrer: Key,
        _purse: URef,
        amount: U256,
    ) -> (Vec<u32>, U256, Vec<u32>) {
        // Payable
        system::transfer_from_purse_to_purse(
            _purse,
            purse(),
            <casper_types::U256 as AsPrimitive<casper_types::U512>>::as_(amount),
            None,
        )
        .unwrap_or_revert();
        let () = runtime::call_versioned_contract(
            key_to_hash(scspr(), Errors::InvalidHash11),
            None,
            "deposit",
            runtime_args! {
                "amount" => amount,
                "purse" => purse()
            },
        );
        self._create_stake_with_scspr(self.get_caller(), amount, lock_days, referrer)
    }

    fn _create_stake_with_scspr(
        &mut self,
        staker_address: Key,
        token_amount: U256,
        lock_days: u64,
        referrer: Key,
    ) -> (Vec<u32>, U256, Vec<u32>) {
        let () = runtime::call_versioned_contract(
            key_to_hash(scspr(), Errors::InvalidHash11),
            None,
            "approve",
            runtime_args! {
                "spender" => uniswap_router(),
                "amount" => token_amount,
            },
        );
        let path = vec![
            scspr().to_formatted_string(),
            package_hash().to_formatted_string(),
        ];
        let amounts: Vec<U256> = runtime::call_versioned_contract(
            key_to_hash(uniswap_router(), Errors::InvalidHash12),
            None,
            "swap_exact_tokens_for_tokens",
            runtime_args! {
                "amount_in" => token_amount,
                "amount_out_min" => U256::from(1),
                "path" => path,
                "to" => staker_address,
                "deadline" => U256::from(block_timestamp() + (2 * 3600000))
            },
        );
        self.create_stake(amounts[1], lock_days, referrer)
    }

    fn get_pair_address(&self) -> Key {
        uniswap_pair()
    }

    fn get_total_staked(&self) -> U256 {
        globals().total_staked
    }

    fn get_liquidity_transformer(&self) -> Key {
        liquidity_transformer().0
    }

    fn get_synthetic_token_address(&self) -> Key {
        scspr()
    }
}
