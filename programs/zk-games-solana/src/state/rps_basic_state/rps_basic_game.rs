use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Player1Info {
    pub key: Pubkey,
    pub choice_hash: [u8; 32],
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Player2Info {
    pub key: Pubkey,
    pub choice: u8,
}

#[account]
#[derive(InitSpace)]
pub struct RpsBasicGame {
    pub game_client: Pubkey,
    pub player1: Player1Info,
    pub player2: Option<Player2Info>,
    pub id: u64,
    pub amount: u64,
    pub timeout: Option<i64>,
    pub bump: u8,
}
