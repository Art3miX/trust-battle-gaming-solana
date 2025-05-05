use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct GameType {
    // The id of a specific game type
    pub id: u32,
    pub bump: u8,
}
