use anchor_lang::prelude::*;

use crate::{GameClient, Player, RpsBasicGame};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitRpsBasicData {
    pub id: u64,
    pub choice_hash: [u8; 32],
}

#[derive(Accounts)]
#[instruction(init_rps_basic_data: InitRpsBasicData)]
pub struct InitRpsBasic<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        init,
        space= 8 + RpsBasicGame::INIT_SPACE,
        payer=signer,
        seeds=[
            "rps_basic_game".as_bytes(),
            game_client.key().as_ref(),
            &init_rps_basic_data.id.to_le_bytes()
        ],
        bump
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
            "game_client".as_bytes(),
            &game_client.signer.key().to_bytes()
        ],
        bump = game_client.bump,
        has_one = signer,
    )]
    pub game_client: Account<'info, GameClient>,
    system_program: Program<'info, System>,
}
