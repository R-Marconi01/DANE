use anchor_lang::prelude::*;
use std::vec::Vec;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("EduhnXzkm9ZKHZsV9AneSFk6BRhJ6bEP5YJG9BjaUwcA");

#[program]
pub mod energy_tracking {
    use super::*;

    pub fn initialize_cer(ctx: Context<InitializeCer>, cer_id: String) -> ProgramResult {
        let cer = &mut ctx.accounts.cer;
        cer.cer_id = cer_id;
        cer.total_consumption = 0;
        cer.total_production = 0;
        cer.last_update_time = Clock::get()?.unix_timestamp;
        cer.users = Vec::new();
        Ok(())
    }

    pub fn add_user(ctx: Context<AddUser>, user_id: String) -> ProgramResult {
        let cer = &mut ctx.accounts.cer;
        let user = User {
            user_id,
            consumption: 0,
            production: 0,
            last_update_time: Clock::get()?.unix_timestamp,
            records: Vec::new(),
        };
        cer.users.push(user);
        Ok(())
    }

    pub fn update_user_data(ctx: Context<UpdateUserData>, user_id: String, consumption: u64, production: u64, timestamp: i64) -> ProgramResult {
        let cer = &mut ctx.accounts.cer;
        if let Some(user) = cer.users.iter_mut().find(|u| u.user_id == user_id) {
            user.consumption += consumption;
            user.production += production;
            user.last_update_time = timestamp;
            let record = Record {
                timestamp,
                consumption,
                production,
            };
            user.records.push(record);
        }
        Ok(())
    }

    pub fn remove_user(ctx: Context<RemoveUser>, user_id: String) -> ProgramResult {
        let cer = &mut ctx.accounts.cer;
        cer.users.retain(|user| user.user_id != user_id);
        Ok(())
    }

    pub fn upload_entire_cer_data(ctx: Context<UploadEntireCERData>, user_data: Vec<(String, u64, u64, i64)>) -> ProgramResult {
        let cer = &mut ctx.accounts.cer;
        let mut total_consumption = 0;
        let mut total_production = 0;

        for (user_id, consumption, production, timestamp) in user_data.iter() {
            if let Some(user) = cer.users.iter_mut().find(|u| &u.user_id == user_id) {
                user.consumption += consumption;
                user.production += production;
                user.last_update_time = *timestamp;
                let record = Record {
                    timestamp: *timestamp,
                    consumption: *consumption,
                    production: *production,
                };
                user.records.push(record);
                total_consumption += consumption;
                total_production += production;
            }
        }

        cer.total_consumption += total_consumption;
        cer.total_production += total_production;
        cer.last_update_time = Clock::get()?.unix_timestamp;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCer<'info> {
    #[account(init, payer = user, space = 8 + 32 + 4 + 100 * 32 + 8 + 8 + 8 + 4 + (32 + 8 + 8 + 8 + 8 * 100) * 100)]
    pub cer: Account<'info, CER>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddUser<'info> {
    #[account(mut)]
    pub cer: Account<'info, CER>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateUserData<'info> {
    #[account(mut)]
    pub cer: Account<'info, CER>,
}

#[derive(Accounts)]
pub struct RemoveUser<'info> {
    #[account(mut)]
    pub cer: Account<'info, CER>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct UploadEntireCERData<'info> {
    #[account(mut)]
    pub cer: Account<'info, CER>,
}

#[account]
pub struct CER {
    pub cer_id: String,
    pub users: Vec<User>,
    pub total_consumption: u64,
    pub total_production: u64,
    pub last_update_time: i64,
}

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct User {
    pub user_id: String,
    pub consumption: u64,
    pub production: u64,
    pub last_update_time: i64,
    pub records: Vec<Record>,
}

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct Record {
    pub timestamp: i64,
    pub consumption: u64,
    pub production: u64,
}