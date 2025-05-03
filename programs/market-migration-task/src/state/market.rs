use anchor_lang::prelude::*;

pub const STATUS_CAN_DEPOSIT_LIQUIDITY: u8 = 0b0000_0001;
pub const STATUS_CAN_WITHDRAW_LIQUIDITY: u8 = 0b0000_0010;
pub const STATUS_CAN_BUY_PT: u8 = 0b0000_0100;
pub const STATUS_CAN_SELL_PT: u8 = 0b0000_1000;
pub const STATUS_CAN_BUY_YT: u8 = 0b0001_0000;
pub const STATUS_CAN_SELL_YT: u8 = 0b0010_0000;

pub const ALL_FLAGS: u8 = STATUS_CAN_DEPOSIT_LIQUIDITY
    | STATUS_CAN_WITHDRAW_LIQUIDITY
    | STATUS_CAN_BUY_PT
    | STATUS_CAN_SELL_PT
    | STATUS_CAN_BUY_YT
    | STATUS_CAN_SELL_YT;

    
#[account]
pub struct MarketTwo {
    /// Address to ALT
    pub address_lookup_table: Pubkey, 

    /// Mint of the vault's PT token
    pub mint_pt: Pubkey, // 72

    /// Mint of the SY program's SY token
    pub mint_sy: Pubkey, // 104

    /// Link to yield-stripping vault
    pub vault: Pubkey, // 136

    /// Mint for the market's LP tokens
    pub mint_lp: Pubkey, // 168

    /// Holds the LP tokens that are earning emissions
    /// This is where LP holders "stake" their LP tokens
    pub token_lp_escrow: Pubkey, // 200

    /// Token account that holds PT liquidity
    pub token_pt_escrow: Pubkey, // 232

    /// Pass-through token account for SY moving from the depositor to the SY program
    pub token_sy_escrow: Pubkey, // 264

    /// Token account that holds SY fees from trade_pt
    pub token_fee_treasury_sy: Pubkey, // 296

    /// Fee treasury SY BPS
    pub fee_treasury_sy_bps: u16, // 298

    /// Authority for CPI calls owned by the market struct
    pub self_address: Pubkey, // 330

    /// Bump for signing the PDA
    pub signer_bump: [u8; 1], //331

    pub status_flags: u8, // 335

    /// Link to the SY program ID
    pub sy_program: Pubkey, // 367

    pub financials: MarketFinancials, 

    pub emissions: MarketEmissions,

    pub lp_farm: LpFarm,

    pub max_lp_supply: u64,

    pub lp_escrow_amount: u64,

    /// Record of CPI accounts
    pub cpi_accounts: CpiAccounts,

    pub is_current_flash_swap: bool,

    pub liquidity_net_balance_limits: LiquidityNetBalanceLimits,

    pub flash_swap_fee_bps: u32,
}

/// Financial parameters for the market
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Default)]
pub struct MarketFinancials {
    /// Expiration timestamp, which is copied from the vault associated with the PT
    pub expiration_ts: u64,

    /// Balance of PT in the market
    /// This amount is tracked separately to prevent bugs from token transfers directly to the market
    pub pt_balance: u64,

    /// Balance of SY in the market
    /// This amount is tracked separately to prevent bugs from token transfers directly to the market
    pub sy_balance: u64,

    /// Initial log of fee rate, which decreases over time
    pub ln_fee_rate_root: f64,

    /// Last seen log of implied rate (APY) for PT
    /// Used to maintain continuity of the APY between trades over time
    pub last_ln_implied_rate: f64,

    /// Initial rate scalar, which increases over time
    pub rate_scalar_root: f64,
}

#[derive(AnchorDeserialize, AnchorSerialize, Default, Clone)]
pub struct MarketEmissions {
    pub trackers: Vec<MarketEmission>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Default, Clone)]
pub struct MarketEmission {
    /// Escrow account that receives the emissions from the SY program
    /// And then passes them through to the user
    pub token_escrow: Pubkey,

    /// Index for converting LP shares into earned emissions
    pub lp_share_index: Number,

    /// The difference between the staged amount and collected emission amount
    pub last_seen_staged: u64,
}

/// High precision number, stored as 4 u64 words in little endian
#[derive(Default, Clone, Debug, Copy, PartialEq, Eq, AnchorSerialize, AnchorDeserialize)]
pub struct Number([u64; 4]);

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct LpFarm {
    pub last_seen_timestamp: u32,
    pub farm_emissions: Vec<FarmEmission>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct FarmEmission {
    /// Mint for the emission token
    pub mint: Pubkey,
    /// Rate at which the emission token is emitted per second
    pub token_rate: u64,
    /// Expiration timestamp for the emission token
    pub expiry_timestamp: u32,
    /// Index for converting LP shares into earned emissions
    pub index: Number,
}

/// Account lists for validating CPI calls to the SY program
#[derive(AnchorDeserialize, AnchorSerialize, Default, Clone, Debug)]
pub struct CpiAccounts {
    /// Fetch SY state
    pub get_sy_state: Vec<CpiInterfaceContext>,

    /// Deposit SY into personal account owned by vault
    pub deposit_sy: Vec<CpiInterfaceContext>,

    /// Withdraw SY from personal account owned by vault
    pub withdraw_sy: Vec<CpiInterfaceContext>,

    /// Settle rewards for vault to accounts owned by the vault
    pub claim_emission: Vec<Vec<CpiInterfaceContext>>,

    /// Get personal yield position
    pub get_position_state: Vec<CpiInterfaceContext>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct CpiInterfaceContext {
    /// Address-lookup-table index
    pub alt_index: u8,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(AnchorDeserialize, AnchorSerialize, Default, Clone)]
pub struct LiquidityNetBalanceLimits {
    pub window_start_timestamp: u32,
    pub window_start_net_balance: u64,
    /// Maximum allowed negative change in basis points (10000 = 100%)
    pub max_net_balance_change_negative_percentage: u16,
    /// Maximum allowed positive change in basis points (10000 = 100%)
    /// Using u32 to allow for very large increases (up to ~429,496%)
    pub max_net_balance_change_positive_percentage: u32,
    pub window_duration_seconds: u32,
}

#[cfg(feature = "migration")]
#[account]
pub struct MarketTwoOld {
    /// Address to ALT
    pub address_lookup_table: Pubkey, 

    /// Mint of the vault's PT token
    pub mint_pt: Pubkey, // 72

    /// Mint of the SY program's SY token
    pub mint_sy: Pubkey, // 104

    /// Link to yield-stripping vault
    pub vault: Pubkey, // 136

    /// Mint for the market's LP tokens
    pub mint_lp: Pubkey, // 168

    /// Holds the LP tokens that are earning emissions
    /// This is where LP holders "stake" their LP tokens
    pub token_lp_escrow: Pubkey, // 200

    /// Token account that holds PT liquidity
    pub token_pt_escrow: Pubkey, // 232

    /// Pass-through token account for SY moving from the depositor to the SY program
    pub token_sy_escrow: Pubkey, // 264

    /// Token account that holds SY fees from trade_pt
    pub token_fee_treasury_sy: Pubkey, // 296

    /// Fee treasury SY BPS
    pub fee_treasury_sy_bps: u16, // 298

    /// Authority for CPI calls owned by the market struct
    pub self_address: Pubkey, // 330

    /// Bump for signing the PDA
    pub signer_bump: [u8; 1], //331

    pub status_flags: u8, // 335

    /// Link to the SY program ID
    pub sy_program: Pubkey, // 367

    pub financials: MarketFinancials, 

    pub emissions: MarketEmissions,

    pub lp_farm: LpFarm,

    pub max_lp_supply: u64,

    pub lp_escrow_amount: u64,

    /// Record of CPI accounts
    pub cpi_accounts: CpiAccounts,

    pub is_current_flash_swap: bool,

    pub liquidity_net_balance_limits: LiquidityNetBalanceLimits,

    pub flash_swap_fee_bps: u32,
}