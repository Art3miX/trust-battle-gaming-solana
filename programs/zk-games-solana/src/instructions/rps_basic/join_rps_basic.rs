use anchor_lang::prelude::*;

use crate::{errors::MyError, GameClient, Player, RpsBasicGame};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct JoinRpsBasicData {
    pub player2_choice: u8,
}

#[derive(Accounts)]
pub struct JoinRpsBasic<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        seeds=[
            "rps_basic_game".as_bytes(),
            rps_basic_game.game_client.key().as_ref(),
            &rps_basic_game.id.to_le_bytes()
        ],
        bump = rps_basic_game.bump,
        constraint = player1.key() != player2.key() @ MyError::RpsBasicSamePlayer,
        constraint = player1.key() == rps_basic_game.player1.key @ MyError::RpsBasicPlayer1Mismatch,
        constraint = rps_basic_game.player2.is_none() @ MyError::RpsBasicGameJoined,
    )]
    pub rps_basic_game: Account<'info, RpsBasicGame>,
    #[account(
        seeds=[
            "player".as_bytes(),
            player1.username.as_bytes()
        ],
        bump = player1.bump
    )]
    pub player1: Account<'info, Player>,
    #[account(
        seeds=[
            "player".as_bytes(),
            player2.username.as_bytes()
        ],
        bump = player2.bump,
    )]
    pub player2: Account<'info, Player>,
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
