use anchor_lang::prelude::*;

use crate::state::{Todo, Profile};
use crate::error::AppError;

#[derive(Accounts)]
pub struct ToggleTodo<'a> {
    #[account(mut)]
    pub creator: Signer<'a>,

    #[account(
        mut,
        constraint = profile.authority == creator.key() @ AppError::InvalidAuthority
    )]
    pub profile: Account<'a, Profile>,

    #[account(
        init,
        space = Todo::INIT_SPACE,
        payer = creator
    )]
    pub todo: Account<'a, Todo>,

    pub system_program: Program<'a, System>,
}

pub fn handler(ctx: Context<ToggleTodo>) -> Result<()> {
    let todo = &mut ctx.accounts.todo;
    
    todo.is_completed = !todo.is_completed;

    Ok(())
}
