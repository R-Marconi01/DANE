use anchor_lang::prelude::*;

declare_id!("75TxYoyXwZTDSjCR8QtryQyGmohFKD5kXDzVik9PAKwf");

#[program]
pub mod dane_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
