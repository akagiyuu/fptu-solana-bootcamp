pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("CcuXEzv6Pu1v2hVmjMrFYBbatweAPMrBhzC6mNnU1bpx");

#[program]
pub mod todo_program {
    use super::*;

    pub fn create_profile(ctx: Context<CreateProfile>, name: String) -> Result<()> {
        create_profile::handler(ctx, name)
    }

    pub fn create_todo(ctx: Context<CreateTodo>, title: String, content: String) -> Result<()> {
        create_todo::handler(ctx, title, content)
    }

    pub fn toggle_todo(ctx: Context<ToggleTodo>) -> Result<()> {
        toggle_todo::handler(ctx)
    }

    pub fn update_todo(ctx: Context<UpdateTodo>, title: Option<String>, content: Option<String>) -> Result<()> {
        update_todo::handler(ctx, title, content)
    }

    pub fn delete_todo(ctx: Content<DeleteTodo>) -> Result<()> {
        delete_todo::handler(ctx)
    }
}
