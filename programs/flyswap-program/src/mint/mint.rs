
use crate::{Seat, ANCHOR_DISCRIMINATOR};
use anchor_lang::prelude::*;
use mpl_core::{
    ID as MPL_CORE_ID,
    accounts::BaseCollectionV1, 
    instructions::CreateV1CpiBuilder, 
};


#[derive(Accounts)]
pub struct MintSeat<'info> {
    pub signer: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        space=ANCHOR_DISCRIMINATOR + Seat::INIT_SPACE,
        payer=payer,
        seeds = [b"seat", payer.key().as_ref()],
        bump
    )]
    pub seat: Account<'info, Seat>,

    #[account(mut)]
    pub update_authority: Signer<'info>,

    #[account(
        mut,
        constraint = collection.update_authority == signer.key()
    )]
    pub collection: Option<Account<'info, BaseCollectionV1>>,

    #[account(mut)]
    pub asset: Signer<'info>,

    #[account(address = MPL_CORE_ID)]
    /// CHECK: Checked in mpl-core.
    pub mpl_core_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn process_mint_seat(
    ctx: Context<MintSeat>,
    seat_number: u8,
    reservation_time: i64,
    departure_time: i64,
    arrival_time: i64,
    name: String,
    uri: String,
) -> Result<()> {
    let seat = &mut ctx.accounts.seat;
    seat.arrival_time = arrival_time;
    seat.reservation_time = reservation_time;
    seat.seat_number = seat_number;
    seat.departure_time = departure_time;
    seat.is_occupied = false;
    seat.is_used = false;
    seat.bump = ctx.bumps.seat;
    msg!("Seat Minted");

    msg!("Time to create Asset");

    let collection = match &ctx.accounts.collection {
        Some(collection) => Some(collection.to_account_info()),
        None => None,
    };

    CreateV1CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
        .asset(&ctx.accounts.asset.to_account_info())
        .collection(collection.as_ref())
        .payer(&ctx.accounts.payer.to_account_info())
        .system_program(&ctx.accounts.system_program.to_account_info())
        .name(name.clone())
        .uri(uri.clone())
        .invoke()?;

    Ok(())
}
