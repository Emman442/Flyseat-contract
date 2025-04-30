use anchor_lang::prelude::*;

use crate::Seat;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct RedeemSeat<'info> {
    pub redeemer: Signer<'info>,

    #[account(mut)]
    pub seat: Account<'info, Seat>,

    pub system_program: Program<'info, System>,
}


pub fn process_redeem_seat(
    ctx: Context<RedeemSeat>,
) -> Result<()> {
     let seat = &mut ctx.accounts.seat;
    let now = Clock::get()?.unix_timestamp;

    require_keys_eq!(ctx.accounts.redeemer.key(), seat.owner, ErrorCode::UnauthorizedRedemption);


    require!(
        now >= seat.departure_time - 7200 && now <= seat.departure_time - 1800,
        ErrorCode::CheckInWindowClosed
    );

    require!(!seat.is_used, ErrorCode::AlreadyCheckedIn);

    seat.is_used = true;
    seat.checked_in_time = Some(now);

    msg!("Seat {} successfully checked in!", seat.seat_number);

    Ok(())
}