use anchor_lang::prelude::*;

use crate::state::Profile;
use crate::error::AppError;

#[derive(Accounts)]
pub struct CreateProfile<'a> {
    #[account(mut)]
    pub creator: Signer<'a>,

    #[account(
        init,
        payer = creator,
        space = Profile::INIT_SPACE
    )]
    pub profile: Account<'a, Profile>,

    pub system_program: Program<'a, System>
}

pub fn handler(ctx: Context<CreateProfile>, name: String) -> Result<()> {
    require!(name.len() <= 100, AppError::NameTooLong);
    
    let profile = &mut ctx.accounts.profile;

    profile.pubkey = profile.key();
    profile.name = name;
    profile.authority = ctx.accounts.creator.key();
    profile.todo_count = 0;

    Ok(())
}
