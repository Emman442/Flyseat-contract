use crate::error::ErrorCode;
use crate::Seat;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{transfer_checked, Mint, TokenInterface, TransferChecked, TokenAccount};

use mpl_core::{
    accounts::BaseCollectionV1,
    instructions::TransferV1CpiBuilder,
    ID as MPL_CORE_ID,
};

#[derive(Accounts)]
pub struct BuySeat<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(mut)]
    buyer_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub seat: Account<'info, Seat>,

    #[account(mut)]
    pub collection: Option<Account<'info, BaseCollectionV1>>,

    ///CHECK - check assets
    #[account(mut)]
    pub asset: UncheckedAccount<'info>,

    #[account(mut)]
    pub seller: InterfaceAccount<'info, TokenAccount>,

    pub mint: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,
    #[account(address = MPL_CORE_ID)]
    /// CHECK: Checked in mpl-core.
    pub mpl_core_program: UncheckedAccount<'info>,
    pub token_program: Interface<'info, TokenInterface>,
}
pub fn process_buy_seat(ctx: Context<BuySeat>, amount: u64) -> Result<()> {
    let seat = &mut ctx.accounts.seat;

    require!(!seat.is_occupied, ErrorCode::SeatAlreadyBooked);
    let cpi_accounts = TransferChecked {
        from: ctx.accounts.buyer_token_account.to_account_info(),
        to: ctx.accounts.seller.to_account_info(),
        authority: ctx.accounts.buyer.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    transfer_checked(cpi_ctx, amount, ctx.accounts.mint.decimals)?;

    msg!(
        "Tokens Transferred Successfully to {}",
        ctx.accounts.buyer.key()
    );

    let collection = match &ctx.accounts.collection {
        Some(collection) => Some(collection.to_account_info()),
        None => None,
    };

    TransferV1CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
        .asset(&ctx.accounts.asset.to_account_info())
        .collection(collection.as_ref())
        .payer(&ctx.accounts.buyer.to_account_info())
        .system_program(Some(&ctx.accounts.system_program.to_account_info()))
        .new_owner(&ctx.accounts.buyer.to_account_info())
        .invoke()?;

    seat.is_occupied = true;
    seat.owner = ctx.accounts.buyer.key();

    Ok(())
}
