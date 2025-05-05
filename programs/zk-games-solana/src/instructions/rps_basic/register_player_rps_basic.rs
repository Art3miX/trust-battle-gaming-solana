use anchor_lang::prelude::*;

use crate::{GameClient, Player, RpsBasicPlayer};

#[derive(Accounts)]
pub struct RegisterPlayerRpsBasic<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        init,
        space= 8 + RpsBasicPlayer::INIT_SPACE,
        payer=signer,
        seeds=[
            "rps_basic_player".as_bytes(),
            player.username.as_bytes()
        ],
        bump
    )]
    pub player_rps_basic: Account<'info, RpsBasicPlayer>,
    #[account(
        seeds=[
            "player".as_bytes(),
            player.username.as_bytes()
        ],
        bump = player.bump
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
    pub game_client: Account<'info, GameClient>,
    system_program: Program<'info, System>,
}
