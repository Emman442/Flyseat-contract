use anchor_lang::prelude::*;

use crate::{Seat, ANCHOR_DISCRIMINATOR};

#[derive(Accounts)]
#[instruction(seat_id: String)]
pub struct MintSeat<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        space=ANCHOR_DISCRIMINATOR + Seat::INIT_SPACE,
        payer=authority,
        seeds = [b"seat", seat_id.as_bytes()],
        bump
    )]
    pub seat: Account<'info, Seat>,

    pub system_program: Program<'info, System>,
}

pub fn process_mint_seat(
    ctx: Context<MintSeat>,
    seat_number: u8,
    reservation_time: i64,
    departure_time: i64,
    arrival_time: i64,
) -> Result<()> {
    let seat = &mut ctx.accounts.seat;
    seat.arrival_time = arrival_time;
    seat.reservation_time = reservation_time;
    seat.seat_number=seat_number;
    seat.departure_time = departure_time;
    seat.is_occupied = false;
    seat.is_used = false;
    seat.owner = ctx.accounts.authority.key();
    seat.bump = *ctx.bumps.seat;
    msg!("Minted seat with ID: {}", seat_number);
    Ok(())
}
