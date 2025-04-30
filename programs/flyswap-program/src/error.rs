use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The provided seed is invalid.")]
    InvalidSeed,
    #[msg("The provided bump is invalid.")]
    InvalidBump,
    #[msg("The provided mint is invalid.")]
    InvalidMint,
    #[msg("The provided token account is invalid.")]
    InvalidTokenAccount,
    #[msg("The provided authority is invalid.")]
    InvalidAuthority,
    #[msg("The provided amount is invalid.")]
    InvalidAmount,
    #[msg("This seat is already booked.")]
    SeatAlreadyBooked,
    #[msg("The seat is already checked in.")]
    AlreadyCheckedIn,
    #[msg("The check-in window is closed.")]
    CheckInWindowClosed,
    #[msg("The provided collection is invalid.")]
    InvalidCollection,
    #[msg("This Authority is not the owner of the seat.")]
    UnauthorizedRedemption,
   
  
    

}