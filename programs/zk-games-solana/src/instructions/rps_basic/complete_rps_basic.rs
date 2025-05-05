use anchor_lang::prelude::*;

use crate::{errors::MyError, GameClient, Player, RpsBasicGame, RpsBasicPlayer};

#[derive(borsh::BorshDeserialize, borsh::BorshSerialize)]
pub struct CompleteRpsBasicData {
    pub player1_choice: u8,
    // TODO: Add proof here
}

#[derive(Accounts)]
pub struct CompleteRpsBasic<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        mut,
        close = signer,
        seeds = [
            "rps_basic_game".as_bytes(),
            rps_basic_game.game_client.key().as_ref(),
            &rps_basic_game.id.to_le_bytes()
        ],
        bump = rps_basic_game.bump,
        constraint = player1.key() == rps_basic_game.player1.key @ MyError::RpsBasicPlayer1Mismatch,
        constraint = rps_basic_game.player2.is_some() @ MyError::RpsBasicGameNotJoined,
        constraint = player2.key() == rps_basic_game.player2.as_ref().unwrap().key @ MyError::RpsBasicPlayer2Mismatch,
    )]
    pub rps_basic_game: Account<'info, RpsBasicGame>,
    #[account(
        seeds = [
            "player".as_bytes(),
            player1.username.as_bytes()
        ],
        bump = player1.bump
    )]
    pub player1: Account<'info, Player>,
    #[account(
        seeds = [
            "rps_basic_player".as_bytes(),
            player1.username.as_bytes()
        ],
        bump = player1_rps_basic.bump
    )]
    pub player1_rps_basic: Account<'info, RpsBasicPlayer>,
    #[account(
        seeds = [
            "player".as_bytes(),
            player2.username.as_bytes()
        ],
        bump = player2.bump,
    )]
    pub player2: Account<'info, Player>,
    #[account(
        seeds = [
            "rps_basic_player".as_bytes(),
            player2.username.as_bytes()
        ],
        bump = player2_rps_basic.bump
    )]
    pub player2_rps_basic: Account<'info, RpsBasicPlayer>,
    #[account(
        seeds = [
            "game_client".as_bytes(),
            &signer.key().to_bytes()
        ],
        bump = game_client.bump,
        has_one = signer,
    )]
    game_client: Account<'info, GameClient>,
    system_program: Program<'info, System>,
}
