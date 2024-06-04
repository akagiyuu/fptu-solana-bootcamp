use anchor_lang::prelude::*;

use crate::error::AppError;
use crate::state::{Profile, Todo};

#[derive(Accounts)]
pub struct UpdateTodo<'a> {
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

pub fn handler(
    ctx: Context<UpdateTodo>,
    title: Option<String>,
    content: Option<String>,
) -> Result<()> {
    let todo = &mut ctx.accounts.todo;

    if let Some(title) = title {
        require!(title.len() <= 50, AppError::TitleTooLong);
        todo.title = title;
    }

    if let Some(content) = content {
        require!(content.len() <= 200, AppError::ContentTooLong);
        todo.content = content;
    }

    Ok(())
}
