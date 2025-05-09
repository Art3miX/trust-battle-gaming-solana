use anchor_lang::prelude::*;
use anchor_spl::{
    token::Token,
    token_interface::{transfer_checked, Mint, TokenAccount, TransferChecked},
};

use crate::{
    calculate_fee, errors::MyError, GameClient, Manager, Player, RpsBasicGame, RpsBasicPlayer,
};

#[derive(Accounts)]
pub struct CancelRpsBasic<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        mut,
        close=signer,
        seeds=[
            "rps_basic_game".as_bytes(),
            game_client.key().as_ref(),
            &rps_basic_game.id.to_le_bytes()
        ],
        bump = rps_basic_game.bump,
        constraint = rps_basic_game.player2.is_none() @ MyError::RpsBasicGameInProgress
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
        mut,
        associated_token::mint = manager.usdc_mint,
        associated_token::authority = player1
    )]
    pub player1_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [
            "rps_basic_player".as_bytes(),
            player1.username.as_bytes()
        ],
        bump = player1_rps_basic.bump
    )]
    pub player1_rps_basic: Account<'info, RpsBasicPlayer>,
    #[account(
        seeds=[
            "game_client".as_bytes(),
            &game_client.signer.key().to_bytes()
        ],
        bump = game_client.bump,
        has_one = signer,
    )]
    pub game_client: Account<'info, GameClient>,
    #[account(
        mut,
        associated_token::mint = manager.usdc_mint,
        associated_token::authority = signer
    )]
    pub game_client_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        seeds=[
            "manager".as_bytes(),
        ],
        bump = manager.bump
    )]
    pub manager: Account<'info, Manager>,
    #[account(address = manager.usdc_mint)]
    pub usdc_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = manager.usdc_mint,
        associated_token::authority = manager,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

impl<'info> CancelRpsBasic<'info> {
    pub fn cancel_rps_basic(&mut self) -> Result<()> {
        let game = &self.rps_basic_game;

        // calculate fee
        let (send_amount, client_amount, _) =
            calculate_fee(game.amount, self.manager.client_fee, 0);

        // We only take client fee for cancellation
        let cpi_accounts = TransferChecked {
            mint: self.usdc_mint.to_account_info(),
            from: self.vault.to_account_info(),
            to: self.game_client_ata.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        transfer_checked(cpi_context, client_amount, self.usdc_mint.decimals)?;

        // Transfer to player1
        let cpi_accounts = TransferChecked {
            mint: self.usdc_mint.to_account_info(),
            from: self.vault.to_account_info(),
            to: self.player1_ata.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        transfer_checked(cpi_context, send_amount, self.usdc_mint.decimals)?;

        let player1_rps_basic = &mut self.player1_rps_basic;
        player1_rps_basic.add_cancel();

        Ok(())
    }
}
