use anchor_lang::prelude::*;

use crate::error::AppError;
use crate::state::{Profile, Todo};

#[derive(Accounts)]
pub struct CreateTodo<'a> {
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

pub fn handler(ctx: Context<CreateTodo>, title: String, content: String) -> Result<()> {
    require!(title.len() <= 50, AppError::TitleTooLong);
    require!(content.len() <= 200, AppError::ContentTooLong);

    let profile = &mut ctx.accounts.profile;
    let todo = &mut ctx.accounts.todo;

    todo.profile = profile.key();
    todo.title = title;
    todo.content = content;
    todo.is_completed = false;

    profile.todo_count += 1;

    Ok(())
}
