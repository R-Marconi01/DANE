// use anchor_lang::prelude::*;

// declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgfgTQFJZXS1");

// #[program]
// pub mod dane_master {
//     use super::*;

//     pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
//         Ok(())
//     }

//     pub fn create_community(ctx: Context<CreateCommunity>, community_address: Pubkey, admin: Pubkey) -> ProgramResult {
//         let community = &mut ctx.accounts.community;
//         community.admin = admin;
//         community.community_address = community_address;
//         Ok(())
//     }

//     pub fn get_community_info(ctx: Context<GetCommunityInfo>, user: Pubkey) -> ProgramResult {
//         let community_info = &ctx.accounts.community;
//         msg!("Admin: {:?}", community_info.admin);
//         msg!("Community Address: {:?}", community_info.community_address);
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct Initialize<'info> {
//     #[account(init, payer = user, space = 8 + 40)]
//     pub user: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }

// #[derive(Accounts)]
// pub struct CreateCommunity<'info> {
//     #[account(init, payer = admin, space = 8 + 40 + 32)]
//     pub community: Account<'info, Community>,
//     #[account(mut)]
//     pub admin: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }

// #[derive(Accounts)]
// pub struct GetCommunityInfo<'info> {
//     pub community: Account<'info, Community>,
// }

// #[account]
// pub struct Community {
//     pub admin: Pubkey,
//     pub community_address: Pubkey,
// }

//------------------------------------------------------------------

// use anchor_lang::prelude::*;
// use solana_program::program::invoke;
// use solana_program::system_instruction;

// declare_id!("DaneProgram111111111111111111111111111111111");

// #[program]
// pub mod dane_program {
//     use super::*;
//     pub fn create_community(ctx: Context<CreateCommunity>, name: String, admin: Pubkey) -> Result<()> {
//         let community = &mut ctx.accounts.community;
//         community.name = name;
//         community.admin = admin;
//         community.members = vec![admin];

//         // CPI to create a community account in the community program
//         let ix = system_instruction::create_account(
//             &ctx.accounts.user.key(),
//             &ctx.accounts.community_program.key(),
//             Rent::get()?.minimum_balance(8 + 32 + 32 + 1024),
//             8 + 32 + 32 + 1024,
//             &ctx.accounts.community_program.key(),
//         );
//         invoke(
//             &ix,
//             &[
//                 ctx.accounts.user.to_account_info(),
//                 ctx.accounts.community_program.to_account_info(),
//                 ctx.accounts.system_program.to_account_info(),
//             ],
//         )?;

//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct CreateCommunity<'info> {
//     #[account(init, payer = user, space = 8 + 32 + 32 + 1024)]
//     pub community: Account<'info, Community>,
//     #[account(mut)]
//     pub user: Signer<'info>,
//     /// CHECK: Safe because we are only calling CPI
//     pub community_program: AccountInfo<'info>,
//     pub system_program: Program<'info, System>,
// }

// #[account]
// pub struct Community {
//     pub name: String,
//     pub admin: Pubkey,
//     pub members: Vec<Pubkey>,
// }

//------------------------------------------------------------------

// use anchor_lang::prelude::*;
// use solana_program::pubkey::Pubkey;

// declare_id!("DaneProgram111111111111111111111111111111111");

// #[program]
// pub mod dane_program {
//     use super::*;
//     pub fn create_community(ctx: Context<CreateCommunity>, name: Vec<u8>, admin: Pubkey) -> Result<()> {
//         let community = &mut ctx.accounts.community;
//         community.name = name;
//         community.admin = admin;
//         community.members = vec![admin];
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct CreateCommunity<'info> {
//     #[account(init, payer = user, space = Community::max_space())]
//     pub community: Account<'info, Community>,
//     #[account(mut)]
//     pub user: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }

// #[account]
// pub struct Community {
//     pub name: Vec<u8>, // Fixed-size vector for the name
//     pub admin: Pubkey,
//     pub members: Vec<Pubkey>,
// }

// impl Community {
//     // Function to calculate the maximum space needed for the Community account
//     fn max_space() -> usize {
//         // Assuming name is 32 bytes
//         let name_size = 32;
//         // Pubkey size
//         let pubkey_size = 32;
//         // Maximum number of members, assuming 100 members
//         let max_members = 100;

//         8 + name_size + pubkey_size + (pubkey_size * max_members)
//     }
// }


// use anchor_lang::prelude::*;
// use solana_program::program::invoke;
// use solana_program::system_instruction;

// declare_id!("DaneProgram111111111111111111111111111111111");

// #[program]
// pub mod dane_program {
//     use super::*;
//     pub fn create_community(ctx: Context<CreateCommunity>, name: Vec<u8>, admin: Pubkey) -> Result<()> {
//         let community = &mut ctx.accounts.community;
//         community.name = name;
//         community.admin = admin;
//         community.members = vec![admin];

//         // CPI to create a community account in the community program
//         let ix = system_instruction::create_account(
//             &ctx.accounts.user.key(),
//             &ctx.accounts.community_program.key(),
//             Rent::get()?.minimum_balance(Community::max_space()),
//             Community::max_space() as u64,
//             &ctx.accounts.community_program.key(),
//         );
//         invoke(
//             &ix,
//             &[
//                 ctx.accounts.user.to_account_info(),
//                 ctx.accounts.community_program.to_account_info(),
//                 ctx.accounts.system_program.to_account_info(),
//             ],
//         )?;

//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct CreateCommunity<'info> {
//     #[account(init, payer = user, space = Community::max_space())]
//     pub community: Account<'info, Community>,
//     #[account(mut)]
//     pub user: Signer<'info>,
//     /// CHECK: Safe because we are only calling CPI
//     pub community_program: AccountInfo<'info>,
//     pub system_program: Program<'info, System>,
// }

// #[account]
// pub struct Community {
//     pub name: Vec<u8>, // Fixed-size vector for the name
//     pub admin: Pubkey,
//     pub members: Vec<Pubkey>,
// }

// impl Community {
//     // Function to calculate the maximum space needed for the Community account
//     fn max_space() -> usize {
//         // Assuming name is 32 bytes
//         let name_size = 32;
//         // Pubkey size
//         let pubkey_size = 32;
//         // Maximum number of members, assuming 100 members
//         let max_members = 100;

//         8 + name_size + pubkey_size + (pubkey_size * max_members)
//     }
// }


// use anchor_lang::prelude::*;
// use solana_program::program::invoke;
// use solana_program::system_instruction;

// declare_id!("DaneProgram111111111111111111111111111111111");

// #[program]
// pub mod dane_program {
//     use super::*;

//     pub fn create_community(ctx: Context<CreateCommunity>, name: Vec<u8>, admin: Pubkey) -> Result<()> {
//         let community = &mut ctx.accounts.community;
//         community.name = name;
//         community.admin = admin;
//         community.members = vec![admin];

//         // CPI to create a community account in the community program
//         let ix = system_instruction::create_account(
//             &ctx.accounts.user.key(),
//             &ctx.accounts.community_program.key(),
//             Rent::get()?.minimum_balance(Community::max_space()),
//             Community::max_space() as u64,
//             &ctx.accounts.community_program.key(),
//         );
//         invoke(
//             &ix,
//             &[
//                 ctx.accounts.user.to_account_info(),
//                 ctx.accounts.community_program.to_account_info(),
//                 ctx.accounts.system_program.to_account_info(),
//             ],
//         )?;

//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct CreateCommunity<'info> {
//     #[account(init, payer = user, space = Community::max_space())]
//     pub community: Account<'info, Community>,
//     #[account(mut)]
//     pub user: Signer<'info>,
//     /// CHECK: Safe because we are only calling CPI
//     pub community_program: AccountInfo<'info>,
//     pub system_program: Program<'info, System>,
// }

// #[account]
// pub struct Community {
//     pub name: Vec<u8>,
//     pub admin: Pubkey,
//     pub members: Vec<Pubkey>,
// }

// impl Community {
//     // Function to calculate the maximum space needed for the Community account
//     fn max_space() -> usize {
//         // Assuming name is 32 bytes
//         let name_size = 32;
//         // Pubkey size
//         let pubkey_size = 32;
//         // Maximum number of members, assuming 100 members
//         let max_members = 100;

//         8 + name_size + pubkey_size + (pubkey_size * max_members)
//     }
// }


// use anchor_lang::prelude::*;

// declare_id!("DaneProgram111111111111111111111111111111111");

// #[program]
// pub mod dane {
//     use super::*;

//     pub fn initialize(ctx: Context<Initialize>, admin: Pubkey) -> Result<()> {
//         let dane_account = &mut ctx.accounts.dane_account;
//         dane_account.admin = admin;
//         dane_account.community_count = 0;
//         Ok(())
//     }

//     pub fn create_community(ctx: Context<CreateCommunity>, community_id: Pubkey) -> Result<()> {
//         let dane_account = &mut ctx.accounts.dane_account;
//         let community_account = &mut ctx.accounts.community_account;

//         if dane_account.admin != ctx.accounts.admin.key() {
//             return Err(ErrorCode::Unauthorized.into());
//         }

//         community_account.id = community_id;
//         community_account.admin = ctx.accounts.admin.key();
//         community_account.user_count = 0;

//         dane_account.community_count += 1;
//         dane_account.communities.push(community_id);

//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct Initialize<'info> {
//     #[account(init, payer = admin, space = 8 + 32 + 8 + 32 * 100)]
//     pub dane_account: Account<'info, DaneAccount>,
//     #[account(mut)]
//     pub admin: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }

// #[derive(Accounts)]
// pub struct CreateCommunity<'info> {
//     #[account(mut)]
//     pub dane_account: Account<'info, DaneAccount>,
//     #[account(init, payer = admin, space = 8 + 32 + 32 + 8 + 32 * 1000)]
//     pub community_account: Account<'info, CommunityAccount>,
//     #[account(mut)]
//     pub admin: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }

// #[account]
// pub struct DaneAccount {
//     pub admin: Pubkey,
//     pub community_count: u64,
//     pub communities: Vec<Pubkey>,
// }

// #[account]
// pub struct CommunityAccount {
//     pub id: Pubkey,
//     pub admin: Pubkey,
//     pub user_count: u64,
//     pub users: Vec<Pubkey>,
// }

// #[error_code]
// pub enum ErrorCode {
//     #[msg("Unauthorized action.")]
//     Unauthorized,
// }


use anchor_lang::prelude::*;
use solana_program::entrypoint::ProgramResult;

declare_id!("13YVAfE694jp4BF1poBcjcaafVkWtKhdhUZutNvHQw1z");

#[program]
pub mod dane {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, admin: Pubkey) -> ProgramResult {
        let dane_account: &mut Account<DaneAccount> = &mut ctx.accounts.dane_account;
        dane_account.admin = admin;
        Ok(())
    }

    pub fn create_community(ctx: Context<CreateCommunity>, community_admin: Pubkey) -> ProgramResult {
        let community = &mut ctx.accounts.community;
        community.admin = community_admin;
        Ok(())
    }

    pub fn add_user(ctx: Context<AddUser>, user: Pubkey) -> ProgramResult {
        let community = &mut ctx.accounts.community;
        community.users.push(user);
        Ok(())
    }

    pub fn publish_energy_data(ctx: Context<PublishEnergyData>, data: EnergyData) -> ProgramResult {
        let community = &mut ctx.accounts.community;
        community.energy_data.push(data);
        Ok(())
    }

    pub fn get_community_data(ctx: Context<GetCommunityData>) -> ProgramResult {
        let community = &ctx.accounts.community;
        msg!("{:?}", community.energy_data);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32)]
    pub dane_account: Account<'info, DaneAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateCommunity<'info> {
    #[account(init, payer = user, space = 8 + 32)]
    pub community: Account<'info, Community>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddUser<'info> {
    #[account(mut)]
    pub community: Account<'info, Community>,
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct PublishEnergyData<'info> {
    #[account(mut)]
    pub community: Account<'info, Community>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetCommunityData<'info> {
    pub community: Account<'info, Community>,
}

#[account]
pub struct DaneAccount {
    pub admin: Pubkey,
}

#[account]
pub struct Community {
    pub admin: Pubkey,
    pub users: Vec<Pubkey>,
    pub energy_data: Vec<EnergyData>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct EnergyData {
    pub user: Pubkey,
    pub consumption: u64,
    pub production: u64,
    pub timestamp: i64,
}
