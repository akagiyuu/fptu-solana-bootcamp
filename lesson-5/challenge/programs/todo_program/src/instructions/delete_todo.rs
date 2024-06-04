use anchor_lang::prelude::*;

use crate::error::AppError;
use crate::state::{Profile, Todo};

#[derive(Accounts)]
pub struct DeleteTodo<'a> {
    #[account(mut)]
    pub creator: Signer<'a>,

    #[account(
        mut,
        constraint = profile.authority == creator.key() @ AppError::InvalidAuthority
    )]
    pub profile: Account<'a, Profile>,

    #[account(mut, close = creator)]
    pub todo: Account<'a, Todo>,

    pub system_program: Program<'a, System>,
}

pub fn handler(ctx: Context<DeleteTodo>) -> Result<()> {
    ctx.accounts.profile.todo_count -= 1;

    Ok(())
}
