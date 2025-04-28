use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Seat {
    pub bump: u8,
    pub owner: Pubkey,
    pub seat_number: u8,
    pub is_occupied: bool,
    pub reservation_time: i64,
    pub departure_time: i64,
    pub arrival_time: i64,
    pub is_used: bool,
}