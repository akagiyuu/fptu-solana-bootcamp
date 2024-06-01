use anchor_lang::prelude::*;

declare_id!("AbLimWYYr2Kcf5a8ZGQAcGpkfVGroCra6DZBpzWQvhxB");

#[error_code]
pub enum UserDataError {
    #[msg("User's name length must be <= 100")]
    NameTooLong,
}

#[account]
#[derive(InitSpace)]
pub struct UserData {
    #[max_len(100)]
    name: String,
    age: u8,
}
impl UserData {
    /// Setter for name to enforce length constraint
    fn set_name(&mut self, name: String) -> Result<()> {
        if name.len() > 100 {
            return err!(UserDataError::NameTooLong);
        }
        self.name = name;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'a> {
    #[account(
        init,
        space = 8 + UserData::INIT_SPACE,
        payer = payer
    )]
    pub user_data: Account<'a, UserData>,
    #[account(mut)]
    pub payer: Signer<'a>,
    pub system_program: Program<'a, System>,
}

#[derive(Accounts)]
pub struct Update<'a> {
    #[account(mut)]
    pub user_data: Account<'a, UserData>,
}

#[program]
pub mod user_info_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, age: u8) -> Result<()> {
        if name.len() > 100 {
            return err!(UserDataError::NameTooLong);
        }

        let user_data = &mut ctx.accounts.user_data;
        user_data.set_name(name)?;
        user_data.age = age;

        Ok(())
    }

    pub fn update(ctx: Context<Update>, name: Option<String>, age: Option<u8>) -> Result<()> {
        let user_data = &mut ctx.accounts.user_data;

        if let Some(name) = name {
            user_data.set_name(name)?;
        }
        if let Some(age) = age {
            user_data.age = age;
        }

        Ok(())
    }
}
