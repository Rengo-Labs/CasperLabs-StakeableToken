// common keys
pub const CONTRACT_HASH: &str = "contract_hash";
pub const PACKAGE_HASH: &str = "package_hash";
pub const PURSE: &str = "purse";

// session keys
pub const GET_INFLATION: &str = "get_inflation";
pub const GET_STABLE_USD_EQUIVALENT: &str = "get_stable_usd_equivalent";
pub const FORWARD_FUNDS: &str = "forward_funds";
pub const GET_TRANSFER_INVOKER_ADDRESS: &str = "get_transfer_invoker_address";

// global
pub const GLOBALS: &str = "globals";

// declaration
pub const INFLATION_RATE: &str = "inflation_rate";
pub const LIQUIDITY_RATE: &str = "liquidity_rate";
pub const LAUNCH_TIME: &str = "launch_time";
pub const STABLE_USD: &str = "stable_usd";
pub const WCSPR: &str = "wcspr";
pub const SCSPR: &str = "scspr";
pub const UNISWAP_ROUTER: &str = "uniswap_router";
pub const UNISWAP_FACTORY: &str = "uniswap_factory";
pub const LIQUIDITY_GUARD: &str = "liquidity_guard";
pub const IS_LIQUIDITY_GUARD_ACTIVE: &str = "is_liquidity_guard_active";
pub const UNISWAP_PAIR: &str = "uniswap_pair";
pub const LATEST_STABLE_USD_EQUIVALENT: &str = "latest_stable_usd_equivalent";
pub const STAKE_COUNT_DICT: &str = "stake_count_dict";
pub const REFERRAL_COUNT_DICT: &str = "referral_count_dict";
pub const LIQUIDITY_STAKE_COUNT_DICT: &str = "liquidity_stake_count_dict";
pub const CRITICAL_MASS_DICT: &str = "critical_mass_dict";
pub const SCRAPES_DICT: &str = "scrapes_dict";
pub const STAKES_DICT: &str = "stakes_dict";
pub const REFERRER_LINKS_DICT: &str = "referrer_links_dict";
pub const LIQUIDITY_STAKES_DICT: &str = "liquidity_stakes_dict";
pub const SCHEDULED_TO_END_DICT: &str = "scheduled_to_end_dict";
pub const REFERRAL_SHARES_TO_END_DICT: &str = "referral_shares_to_end_dict";
pub const TOTAL_PENALTIES_DICT: &str = "total_penalties_dict";

// transfer helper
pub const TRANSFER_INVOKER: &str = "transfer_invoker";

// snapshot
pub const SNAPSHOTS_DICT: &str = "snapshots_dict";
pub const RSNAPSHOTS_DICT: &str = "rsnapshots_dict";
pub const LSNAPSHOTS_DICT: &str = "lsnapshots_dict";

// liquidity_transformer
pub const LIQUIDITY_TRANSFORMER: &str = "liquidity_transformer";
pub const LIQUIDITY_TRANSFORMER_PURSE: &str = "liquidity_transformer_purse";
pub const TRANSFORMER_GATE_KEEPER: &str = "transformer_gate_keeper";

// stable usd equivalent
pub const STAKEABLE: &str = "stakeable";

// liquidity guard
pub const INFLATION_LN: &str = "inflation_ln";
pub const IS_READY: &str = "is_ready";

//Stakeable Token Session Code Keys
pub const CREATE_STAKE_WITH_TOKEN:&str = "create_stake_with_cspr";
pub const CREATE_STAKE_WITH_CSPR:&str = "create_stake_with_token";
pub const TRANSFER:&str = "transfer";
pub const TRANSFER_FROM:&str = "transfer_from";
pub const CHECK_REFERRALS_BY_ID:&str = "check_referrals_by_id";
pub const CREATE_STAKE:&str = "create_stake";
pub const END_STAKE:&str = "end_stake";
pub const SCRAPE_INTEREST:&str = "scrape_interest";
pub const CHECK_MATURE_STAKE:&str = "check_mature_stake";
pub const CHECK_STAKE_BY_ID:&str  = "check_stake_by_id";
pub const CREATE_LIQUIDITY_STAKE:&str  = "create_liquidity_stake";
pub const END_LIQUIDITY_STAKE:&str  = "end_liquidity_stake";
pub const CHECK_LIQUIDITY_STAKE_BY_ID:&str  = "check_liquidity_stake_by_id";
