use anchor_lang::prelude::*;

declare_id!("CommunityProgram11111111111111111111111111111");

#[program]
pub mod community_program {
    use super::*;
    pub fn publish_data(ctx: Context<PublishData>, consumption: u64, production: u64) -> Result<()> {
        let energy_data = &mut ctx.accounts.energy_data;
        energy_data.consumption = consumption;
        energy_data.production = production;
        energy_data.owner = *ctx.accounts.user.key;
        Ok(())
    }

    pub fn add_member(ctx: Context<AddMember>, new_member: Pubkey) -> Result<()> {
        let community = &mut ctx.accounts.community;
        require!(ctx.accounts.admin.key == community.admin, ErrorCode::Unauthorized);
        community.members.push(new_member);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PublishData<'info> {
    #[account(init, payer = user, space = 8 + 8 + 8 + 32)]
    pub energy_data: Account<'info, EnergyData>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub community: Account<'info, Community>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddMember<'info> {
    #[account(mut)]
    pub community: Account<'info, Community>,
    #[account(mut)]
    pub admin: Signer<'info>,
}

#[account]
pub struct EnergyData {
    pub consumption: u64,
    pub production: u64,
    pub owner: Pubkey,
}

#[account]
pub struct Community {
    pub admin: Pubkey,
    pub members: Vec<Pubkey>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized access")]
    Unauthorized,
}
