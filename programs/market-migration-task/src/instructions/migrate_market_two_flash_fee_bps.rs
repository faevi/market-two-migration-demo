use crate::{
    error::ErrorCodes, MarketTwo, MarketTwoOld, DEFAULT_FLASH_SWAP_FEE_BPS, MIGRATE_ADMIN_ADDRESS,
};
use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, system_instruction},
};

#[derive(Accounts)]
pub struct MigrateMarketTwoFlashSwapFeeBps<'info> {
    #[account(mut, address = MIGRATE_ADMIN_ADDRESS)]
    pub admin: Signer<'info>,

    /// CHECK: market_two is checked in the migrate_account function
    #[account(mut)]
    pub market_two: AccountInfo<'info>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

pub fn handler_migrate_market_two_flash_swap_fee_bps(
    ctx: Context<MigrateMarketTwoFlashSwapFeeBps>,
) -> Result<()> {
    let market_two = &mut ctx.accounts.market_two;
    let new_field_size = std::mem::size_of::<u32>();
    let migrated_market_two = get_new_market_two_state(&market_two);

    migrate_account::<MarketTwo>(
        &ctx.accounts.admin,
        market_two,
        &ctx.accounts.rent,
        &ctx.accounts.system_program,
        new_field_size,
    )?;

    migrated_market_two.try_serialize(&mut *market_two.data.borrow_mut())?;

    Ok(())
}

pub fn check_discriminator<T: Discriminator>(acc: &AccountInfo) -> Result<()> {
    let data = acc.try_borrow_data()?;
    let mut disc = [0u8; 8];
    disc.copy_from_slice(&data[..8]);
    if disc != T::DISCRIMINATOR {
        return Err(ErrorCodes::InvalidAccountDiscriminator.into());
    }
    Ok(())
}

pub fn get_new_market_two_state(acc: &AccountInfo) -> MarketTwo {
    let data = acc.data.try_borrow_mut().unwrap();
    let data_slice: &mut &[u8] = &mut &**data;
    let market_two_struct = MarketTwoOld::try_deserialize(data_slice).unwrap();

    let liquidity_net_balance_limits_clone = market_two_struct.liquidity_net_balance_limits.clone();

    let new_market_two = MarketTwo {
        flash_swap_fee_bps: DEFAULT_FLASH_SWAP_FEE_BPS,
        address_lookup_table: market_two_struct.address_lookup_table,
        cpi_accounts: market_two_struct.cpi_accounts,
        emissions: market_two_struct.emissions,
        fee_treasury_sy_bps: market_two_struct.fee_treasury_sy_bps,
        financials: market_two_struct.financials,
        is_current_flash_swap: market_two_struct.is_current_flash_swap,
        liquidity_net_balance_limits: liquidity_net_balance_limits_clone,
        lp_escrow_amount: market_two_struct.lp_escrow_amount,
        lp_farm: market_two_struct.lp_farm,
        max_lp_supply: market_two_struct.max_lp_supply,
        mint_lp: market_two_struct.mint_lp,
        mint_pt: market_two_struct.mint_pt,
        mint_sy: market_two_struct.mint_sy,
        self_address: market_two_struct.self_address,
        signer_bump: market_two_struct.signer_bump,
        status_flags: market_two_struct.status_flags,
        sy_program: market_two_struct.sy_program,
        token_fee_treasury_sy: market_two_struct.token_fee_treasury_sy,
        token_lp_escrow: market_two_struct.token_lp_escrow,
        token_pt_escrow: market_two_struct.token_pt_escrow,
        token_sy_escrow: market_two_struct.token_sy_escrow,
        vault: market_two_struct.vault,
    };

    new_market_two
}

pub fn migrate_account<'info, T>(
    admin: &Signer<'info>,
    account: &AccountInfo<'info>,
    rent: &Sysvar<'info, Rent>,
    system_program: &Program<'info, System>,
    new_field_size: usize,
) -> Result<()>
where
    T: Discriminator,
{
    check_discriminator::<T>(account)?;

    if *account.owner != crate::ID {
        return Err(error!(ErrorCodes::WrongProgramOwner));
    }

    let new_len = account.data_len().checked_add(new_field_size).unwrap();

    let required = rent.minimum_balance(new_len);
    let current = **account.try_borrow_lamports()?;

    if current < required {
        let ix = system_instruction::transfer(
            &admin.key(),
            &account.key(),
            required.checked_sub(current).unwrap(),
        );
        invoke(
            &ix,
            &[
                admin.to_account_info(),
                account.clone(),
                system_program.to_account_info(),
            ],
        )?;
    }

    account.realloc(new_len, false)?;

    Ok(())
}
