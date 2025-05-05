use anchor_lang::prelude::*;

#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Clone, InitSpace)]
pub struct Player1Info {
    pub key: Pubkey,
    pub choice_hash: [u8; 32],
}

#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Clone, InitSpace)]
pub struct Player2Info {
    pub key: Pubkey,
    pub choice: u8,
}

#[account]
#[derive(InitSpace)]
pub struct RpsBasicGame {
    pub id: u64,
    pub player1: Player1Info,
    pub player2: Option<Player2Info>,
    pub timeout: Option<i64>,
    pub game_client: Pubkey,
    pub bump: u8,
}
