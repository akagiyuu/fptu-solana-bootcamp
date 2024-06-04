use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Todo {
    pub profile: Pubkey,
    /// title.len() <= 50
    #[max_len(50)]
    pub title: String,
    /// content.len() <= 200
    #[max_len(200)]
    pub content: String,
    pub is_completed: bool
}
