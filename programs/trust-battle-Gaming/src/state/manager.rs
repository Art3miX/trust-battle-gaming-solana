use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Manager {
    pub client_fee: u16,
    pub platform_fee: u16,
    pub usdc_mint: Pubkey,
    pub platform_key: Pubkey,
    pub bump: u8,
}
