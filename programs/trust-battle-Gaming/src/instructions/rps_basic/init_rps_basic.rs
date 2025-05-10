use anchor_lang::prelude::*;
use anchor_spl::{
    token::Token,
    token_interface::{transfer_checked, Mint, TokenAccount, TransferChecked},
};

use crate::{errors::MyError, GameClient, Manager, Player, Player1Info, RpsBasicGame};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitRpsBasicData {
    pub id: u64,
    pub amount: u64,
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
        mut,
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
        seeds=[
            "game_client".as_bytes(),
            &game_client.signer.key().to_bytes()
        ],
        bump = game_client.bump,
        has_one = signer,
    )]
    pub game_client: Account<'info, GameClient>,
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

impl InitRpsBasic<'_> {
    pub fn init_rps_basic(&mut self, init_game_data: InitRpsBasicData, bump: u8) -> Result<()> {
        let decimals = self.usdc_mint.decimals;

        // Check amount is above minimum
        require!(
            init_game_data.amount >= 10_u64.pow(decimals as u32),
            MyError::RpsBasicAmountTooLow
        );

        let player1_pda_seeds = &[
            "player".as_bytes(),
            self.player1.username.as_bytes(),
            &[self.player1.bump],
        ];
        let player1_pda_seeds = &[&player1_pda_seeds[..]];

        let cpi_accounts = TransferChecked {
            mint: self.usdc_mint.to_account_info(),
            from: self.player1_ata.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.player1.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, player1_pda_seeds);
        transfer_checked(cpi_context, init_game_data.amount, decimals)?;

        self.rps_basic_game.set_inner(RpsBasicGame {
            id: init_game_data.id,
            amount: init_game_data.amount,
            player1: Player1Info {
                key: self.player1.key(),
                choice_hash: init_game_data.choice_hash,
            },
            player2: None,
            timeout: None,
            game_client: self.game_client.key(),
            bump,
        });

        Ok(())
    }
}
