use anchor_lang::prelude::*;
use crate::{Seat, ANCHOR_DISCRIMINATOR};
use mpl_core::{
    // accounts::{BaseAssetV1, BaseCollectionV1},
    instructions::CreateV1CpiBuilder,
};

#[derive(Accounts)]
pub struct MintSeat<'info> {

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        space=ANCHOR_DISCRIMINATOR + Seat::INIT_SPACE,
        payer=payer,
        seeds = [b"seat", payer.key().as_ref().as_bytes()],
        bump
    )]
    pub seat: Account<'info, Seat>,
    #[account(mut)]
    /// CHECK
    pub asset: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK
    pub collection: AccountInfo<'info>,
    #[account(mut)]
    pub update_authority: Signer<'info>,
  
    pub system_program: Program<'info, System>,

    /// CHECK: Checked in mpl-core.
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: AccountInfo<'info>,
}

pub fn mint_seat(
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
    // seat.bump = ctx.accounts.bump;
    // seat.bump = ctx.bumps.seat; // .bumps needs ownership of the account
    msg!("Seat Minted");

    msg!("Time to create Asset");

    CreateV1CpiBuilder::new(&ctx.accounts.mpl_core_program)
        .asset(&ctx.accounts.asset)
        .payer(&ctx.accounts.payer)
        .collection(Some(&ctx.accounts.collection))
        .authority(Some(&ctx.accounts.payer))
        .system_program(&ctx.accounts.system_program)
        .name(name.clone())
        .uri(uri.clone())
        .invoke()?;

    Ok(())
}
