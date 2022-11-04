use crate::data::*;
use casper_contract::contract_api::runtime;
use casper_types::{runtime_args, Key, RuntimeArgs, URef, U256};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use common::{errors::Errors, functions::*};
use liquidity_token::{
    globals, scspr, set_scspr, src::LiquidityToken, uniswap_pair, uniswap_router, ERC20,
};

pub trait StakeableToken<Storage: ContractStorage>:
    ContractContext<Storage> + LiquidityToken<Storage>
{
    fn init(&mut self, contract_hash: Key, package_hash: Key, purse: URef, scspr: Key) {
        ERC20::init(
            self,
            "Stakeable Token".into(),
            "STAKEABLE".into(),
            9,
            0.into(),
            contract_hash,
            key_to_hash(package_hash, Errors::InvalidHash10),
        );
        set_scspr(scspr);
        set_transformer_gate_keeper(self.get_caller());
        set_contract_hash(contract_hash);
        set_package_hash(package_hash);
        set_purse(purse);
    }

    /// @notice ability to define liquidity transformer contract
    /// @dev this method renounce transformerGateKeeper access
    /// @param _immutableTransformer contract address
    fn set_liquidity_transfomer(&self, immutable_transformer: Key, transformer_purse: URef) {
        if transformer_gate_keeper() != self.get_caller() {
            runtime::revert(Errors::TransformerDefined);
        }
        set_liquidity_transformer(immutable_transformer, transformer_purse);
        set_transformer_gate_keeper(account_zero_address());
    }

    /// @notice allows liquidityTransformer to mint supply
    /// @dev executed from liquidityTransformer upon PANCAKESWAP transfer and during reservation payout to contributors and referrers
    /// @param _investorAddress address for minting WISE tokens
    /// @param _amount of tokens to mint for _investorAddress
    fn mint_supply(&mut self, investor_address: Key, amount: U256) {
        if self.get_caller() != liquidity_transformer().0 {
            runtime::revert(Errors::WrongTransformer);
        }
        self.mint(investor_address, amount);
    }

    /// @notice allows to create stake directly with BNB if you don't have WISE tokens method will wrap
    ///     your BNB to SBNB and use that amount on PANCAKESWAP returned amount of WISE tokens will b used to stake
    /// @param _lockDays amount of days it is locked for.
    /// @param _referrer referrer address for +10% bonus
    fn create_stake_with_cspr(
        &mut self,
        lock_days: u64,
        referrer: Key,
        amount: U256,
    ) -> (Vec<u32>, U256, Vec<u32>) {
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
