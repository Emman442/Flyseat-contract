use anchor_lang::prelude::*;

pub mod constant;
pub mod error;
pub mod mint;
pub mod states;
pub mod buySeat;
pub mod redeem;

pub use constant::*;
pub use error::*;
pub use mint::*;
pub use states::*;
pub use buySeat::*; 
pub use redeem::*;
declare_id!("EkwZGxeTUfUhHhVATjXt48yus7RQM7qPayBAyGVHJk9d");

#[program]
pub mod flyswap_program {

    use super::*;

   pub fn mint_seat(
        ctx: Context<MintSeat>,
        seat_number: u8,
        reservation_time: i64,
        departure_time: i64,
        arrival_time: i64,
        name: String,
        uri: String
    ) -> Result<()> {
        process_mint_seat( 
            ctx,
            seat_number,
            reservation_time,
            departure_time,
            arrival_time,
            name,
            uri
        )
    }   


    pub fn buy_seat(
        ctx: Context<BuySeat>,
        amount: u64,
    ) -> Result<()> {
        process_buy_seat(ctx, amount)
    }


    pub fn redeem_seat(
        ctx: Context<RedeemSeat>,
    ) -> Result<()> {
        process_redeem_seat(ctx)?;
        Ok(())
    }


}
