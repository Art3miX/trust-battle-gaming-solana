use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{Manager, ADMIN_PUBKEY};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitData {
    // BPS of client fee
    pub client_fee_bps: u16,
    // BPS of platform fee
    pub platform_fee_bps: u16,
    // Pubkey of platform (where to send fee)
    pub platform_key: Pubkey,
}

#[derive(Accounts)]
#[instruction(init_data: InitData)]
pub struct Init<'info> {
    #[account(mut, address = ADMIN_PUBKEY)]
    admin: Signer<'info>,
    #[account(
        init,
        space= 8 + Manager::INIT_SPACE,
        payer=admin,
        seeds=[
            "manager".as_bytes(),
        ],
        bump
    )]
    pub manager: Account<'info, Manager>,
    #[account(address = manager.usdc_mint)]
    pub usdc_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = admin,
        associated_token::mint = usdc_mint,
        associated_token::authority = manager,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    system_program: Program<'info, System>,
}

impl<'info> Init<'info> {
    pub fn init(&mut self, init_data: InitData, bump: u8) -> Result<()> {
        self.manager.set_inner(Manager {
            client_fee: init_data.client_fee_bps,
            platform_fee: init_data.platform_fee_bps,
            platform_key: init_data.platform_key,
            usdc_mint: self.usdc_mint.key(),
            bump,
        });
        Ok(())
    }
}
