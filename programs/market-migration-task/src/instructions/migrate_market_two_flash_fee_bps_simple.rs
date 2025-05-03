use crate::{error::ErrorCodes, MarketTwo, DEFAULT_FLASH_SWAP_FEE_BPS, MIGRATE_ADMIN_ADDRESS};
use anchor_lang::
    prelude::*
;

#[derive(Accounts)]
pub struct MigrateMarketTwoFlashSwapFeeBpsSimple<'info> {
    #[account(mut, address = MIGRATE_ADMIN_ADDRESS)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        realloc = market_two.to_account_info().data_len() + std::mem::size_of::<u32>(),
        realloc::payer = admin,
        realloc::zero = true,
        constraint = market_two.flash_swap_fee_bps == 0 @ ErrorCodes::MarketTwoFlashSwapFeeBpsAlreadySet,
    )]
    pub market_two: Account<'info, MarketTwo>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

pub fn handler_migrate_market_two_flash_swap_fee_bps_simple(
    ctx: Context<MigrateMarketTwoFlashSwapFeeBpsSimple>,
) -> Result<()> {
    let market_two = &mut ctx.accounts.market_two;
    market_two.flash_swap_fee_bps = DEFAULT_FLASH_SWAP_FEE_BPS;
   
    Ok(())
}