use anchor_lang::prelude::*;

use crate::{GameType, ADMIN_PUBKEY};

#[derive(Accounts)]
#[instruction(game_type_id: u32)]
pub struct RegisterGameType<'info> {
    #[account(mut, address = ADMIN_PUBKEY)]
    admin: Signer<'info>,
    #[account(
        init,
        space= 8 + GameType::INIT_SPACE,
        payer=admin,
        seeds=[
            "game_type".as_bytes(),
            &game_type_id.to_le_bytes()
        ],
        bump
    )]
    pub game_type: Account<'info, GameType>,
    system_program: Program<'info, System>,
}
