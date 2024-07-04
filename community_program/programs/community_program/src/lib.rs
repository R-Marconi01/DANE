use anchor_lang::prelude::*;

declare_id!("BYPryRR2jjawR9G8BwaaLmHc77Zt2cHPYgJP1qGH6Tmx");

#[program]
pub mod community_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
