use anchor_lang::prelude::*;

use crate::{GameClient, ADMIN_PUBKEY};

#[derive(borsh::BorshDeserialize, borsh::BorshSerialize)]
pub struct GameClientData {
    pub name: String,
    pub signer: Pubkey,
}

#[derive(Accounts)]
#[instruction(game_client_data: GameClientData)]
pub struct RegisterGameClient<'info> {
    #[account(mut, address = ADMIN_PUBKEY)]
    pub admin: Signer<'info>,
    #[account(
        init,
        space= 8 + GameClient::INIT_SPACE,
        payer=admin,
        seeds=[
            "game_client".as_bytes(),
            &game_client_data.signer.key().to_bytes()
        ],
        bump
    )]
    pub game_client: Account<'info, GameClient>,
    system_program: Program<'info, System>,
}
