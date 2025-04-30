use anchor_lang::prelude::*;

pub mod constant;
pub mod error;
pub mod mint;
pub mod states;

pub use constant::*;
pub use error::*;
pub use mint::*;
pub use states::*;
declare_id!("77rK3AyovcpMkukLEUfwDp59wfRXkzXomNQGCNmscWeJ");

#[program]
pub mod flyswap_program {

    use super::*;

   pub fn process_mint_seat(
        ctx: Context<MintSeat>,
        seat_number: u8,
        reservation_time: i64,
        departure_time: i64,
        arrival_time: i64,
        name: String,
        uri: String
    ) -> Result<()> {
        mint_seat( 
            ctx,
            seat_number,
            reservation_time,
            departure_time,
            arrival_time,
            name,
            uri
        )
    }   
}
