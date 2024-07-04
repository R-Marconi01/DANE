use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

declare_id!("DaneProgram111111111111111111111111111111");

#[program]
pub mod dane_program {
    use super::*;
    pub fn create_community(ctx: Context<CreateCommunity>, name: String, admin: Pubkey) -> Result<()> {
        let community = &mut ctx.accounts.community;
        community.name = name;
        community.admin = admin;
        community.members = vec![admin];
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateCommunity<'info> {
    #[account(init, payer = user, space = 8 + 32 + 32 + 1024)]
    pub community: Account<'info, Community>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Community {
    pub name: String,
    pub admin: Pubkey,
    pub members: Vec<Pubkey>,
}
