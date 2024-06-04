use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Profile {
    pub pubkey: Pubkey,
    /// name.len() <= 100
    #[max_len(100)]
    pub name: String,
    pub authority: Pubkey,
    pub todo_count: u8
}
