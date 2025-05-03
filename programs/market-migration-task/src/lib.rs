pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("ExponentnaRg3CQbW6dqQNZKXp7gtZ9DGMp1cwC4HAS7");

#[program]
pub mod market_migration_task {
    use super::*;
    
    #[cfg(feature = "migration")]
    pub fn migrate_market_two_flash_fee_bps(ctx: Context<MigrateMarketTwoFlashSwapFeeBps>) -> Result<()> {
        instructions::handler_migrate_market_two_flash_swap_fee_bps(ctx)
    }

    #[cfg(not(feature = "migration"))]
    pub fn migrate_market_two_flash_fee_bps(ctx: Context<MigrateMarketTwoFlashSwapFeeBpsSimple>) -> Result<()> {
        instructions::handler_migrate_market_two_flash_swap_fee_bps_simple(ctx)
    }

    #[cfg(feature = "migration")]
    pub fn mock_instruction(_ctx: Context<MockContext>) -> Result<()> {
       Ok(())
    }
}

// Just for generating correct idl with MarketTwo, when we use feature = "migration", case no reference to MarketTwo
#[derive(Accounts)]
pub struct MockContext<'info> {
    /// CHECK:
    #[account(
        mut
    )]
    pub market_two: Account<'info, MarketTwo>,
}
