pub mod key_names {

    // keys used globally
    pub const ROUTER_CONTRACT_HASH: &str = "router_contract_hash";
    pub const FACTORY_CONTRACT_HASH: &str = "factory_contract_hash";
    pub const PAIR_CONTRACT_HASH: &str = "pair_contract_hash";
    pub const LIQUIDITY_GUARD_CONTRACT_HASH: &str = "liquidity_guard_contract_hash";
    pub const WCSPR_CONTRACT_HASH: &str = "wcspr_contract_hash";
    pub const SCSPR_CONTRACT_HASH: &str = "scspr_contract_hash";
    pub const BUSD_CONTRACT_HASH: &str = "busd_contract_hash";
    pub const BUSD_EQ_CONTRACT_HASH: &str = "busd_eq_contract_hash";
    pub const UNISWAP_PAIR_CONTRACT_HASH: &str = "uniswap_pair_contract_hash";
    pub const SELF_PACKAGE_HASH: &str = "package_hash";
    pub const SELF_CONTRACT_HASH: &str = "self_hash";
    pub const GUARD_CONTRACT_HASH: &str = "guard_contract_hash";
    pub const WISE_TOKEN_CONTRACT_HASH: &str = "wise_token_contract_hash";
    //pub const BEP20_CONTRACT_HASH: &str = "bep20_contract_hash";

    // key names from globals crate
    pub const GLOBALS_GLOBALS_STRUCT: &str = "globals_globals_struct";
    pub const GLOBALS_TOTAL_STAKED: &str = "globals_total_staked";
    pub const GLOBALS_TOTAL_SHARES: &str = "globals_total_shares";
    pub const GLOBALS_SHARE_PRICE: &str = "globals_share_price";
    pub const GLOBALS_CURRENT_WISE_DAY: &str = "globals_current_wise_day";
    pub const GLOBALS_REFERRAL_SHARES: &str = "globals_referral_shares";
    pub const GLOBALS_LIQUIDITY_SHARES: &str = "globals_liquidity_shares";

    // key names from declaration crate
    pub const DECLARATION_LAUNCH_TIME: &str = "declaration_launch_time";
    pub const DECLARATION_INFLATION_RATE: &str = "declaration_inflation_rate";
    pub const DECLARATION_LIQUIDITY_RATE: &str = "declaration_liquidity_rate";
    pub const DECLARATION_LIQUIDITY_GUARD_STATUS: &str = "declaration_liquidity_guard_status";
    pub const DECLARATION_STAKE_COUNT_DICT: &str = "declaration_stake_count_dict";
    pub const DECLARATION_REFERRAL_COUNT_DICT: &str = "declaration_referral_count_dict";
    pub const DECLARATION_LIQUIDITY_STAKE_COUNT_DICT: &str =
        "declaration_liquidity_stake_count_dict";
    pub const DECLARATION_SCHEDULED_TO_END_DICT: &str = "declaration_scheduled_to_end_dict";
    pub const DECLARATION_REFERRAL_SHARES_TO_END_DICT: &str =
        "declaration_referral_shares_to_end_dict";
    pub const DECLARATION_TOTAL_PENALTIES_DICT: &str = "declaration_total_penalties_dict";
    pub const DECLARATION_CRITICAL_MASS_DICT: &str = "declaration_critical_mass_dict";
    pub const DECLARATION_SCRAPES_DICT: &str = "declaration_scrapes_dict";
    pub const DECLARATION_STAKES_DICT: &str = "declaration_stakes_dict";
    pub const DECLARATION_REFERRER_LINK_DICT: &str = "declaration_referrer_link_dict";
    pub const DECLARATION_LIQUIDITY_STAKES_DICT: &str = "declaration_liquidity_stakes_dict";
    pub const DECLARATION_LTBALANCE: &str = "declaration_ltbalance";
    // key names from snapshot crate
    // pub const SNAPSHOT_SNAPSHOT: &str = "snapshot";
    // pub const SNAPSHOT_RSNAPSHOT: &str = "r_snapshot";
    // pub const SNAPSHOT_LSNAPSHOT: &str = "l_snapshot";
    pub const SNAPSHOT_SNAPSHOTS_DICT: &str = "snapshot_snapshots_dict";
    pub const SNAPSHOT_RSNAPSHOTS_DICT: &str = "snapshot_rsnapshots_dict";
    pub const SNAPSHOT_LSNAPSHOTS_DICT: &str = "snapshot_lsnapshots_dict";

    // key names from referral token
    pub const REFERRAL_TOKEN_OWNER: &str = "referral_token_owner";

    // key names from staking token
    pub const STAKING_TOKEN_OWNER: &str = "staking_token_owner";

    // key names fro BUSD_EQ
    pub const BUSD_EQ_LATEST_BUSD_EQUIVALENT: &str = "busd_eq_latest_busd_equivalent";
    pub const BUSD_EQ_YODAS_PER_WISE: &str = "busd_eq_yodas_per_wise";

    // key names from transfer invoker
    pub const TRANSFER_HELPER_TRANSFER_INVOKER: &str = "transfer_helper_transfer_invoker";

    // key names from liquidity guard
    pub const LIQUIDITY_GUARD_LIQUIDITY_GUARD_STATUS: &str =
        "liquidity_guard_liquidity_guard_status";
    pub const LIQUIDITY_GUARD_INFLATION_DICT: &str = "liquidity_guard_inflation_dict";

    // key names from wise token
    pub const WISE_TOKEN_LIQUIDITY_TRANSFORMER: &str = "wise_token_liquidity_transformer";
    pub const WISE_TOKEN_LIQUIDITY_TRANSFORMER_PURSE: &str =
        "wise_token_liquidity_transformer_purse";
    pub const WISE_TOKEN_TRANSFORMER_GATE_KEEPER: &str = "wise_token_transformer_gate_keeper";

    // bep20 key names
    pub const BEP20_DOMAIN_SEPARATOR: &str = "bep20_domain_separator";
    pub const BEP20_PERMIT_TYPE_HASH: &str = "bep20_permit_type_hash";

    // erc20/bep20 key names
    pub const BALANCES_DICT: &str = "balances";
    pub const NONCES_DICT: &str = "nonces";
    pub const ALLOWANCES_DICT: &str = "allowances";
    pub const NAME: &str = "name";
    pub const SYMBOL: &str = "symbol";
    pub const TOTAL_SUPPLY: &str = "total_supply";
    pub const DECIMALS: &str = "decimals";

    //  misc
    pub const PATH: &str = "path";
    pub const CONSTANTS_DICT: &str = "constants_dict";
    pub const OWNER: &str = "owner";
}
