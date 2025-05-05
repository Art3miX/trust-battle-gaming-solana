use anchor_lang::prelude::*;

use crate::{GameClient, Player};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct PlayerData {
    pub username: String,
    pub login_hash: [u8; 32],
}

#[derive(Accounts)]
#[instruction(player_data: PlayerData)]
pub struct RegisterPlayer<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        init,
        space= 8 + Player::INIT_SPACE,
        payer=signer,
        seeds=[
            "player".as_bytes(),
            player_data.username.as_bytes()
        ],
        bump
    )]
    pub player: Account<'info, Player>,
    #[account(
        seeds=[
            "game_client".as_bytes(),
            &signer.key().to_bytes()
        ],
        bump = game_client.bump,
        has_one = signer,
    )]
    game_client: Account<'info, GameClient>,
    system_program: Program<'info, System>,
}
