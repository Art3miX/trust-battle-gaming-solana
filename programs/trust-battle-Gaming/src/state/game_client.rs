use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct GameClient {
    #[max_len(260)]
    pub name: String,
    pub signer: Pubkey,
    pub bump: u8,
}
