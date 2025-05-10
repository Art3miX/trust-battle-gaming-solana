use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Player {
    #[max_len(260)]
    pub username: String,
    pub login_hash: [u8; 32],
    pub bump: u8,
}
