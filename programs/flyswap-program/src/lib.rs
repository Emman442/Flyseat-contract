use anchor_lang::prelude::*;

declare_id!("4QTwYBM4fNoS7L8pNuWUFWwmgjPqaJML3BmrHv4kzNSK");

#[program]
pub mod flyswap_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
