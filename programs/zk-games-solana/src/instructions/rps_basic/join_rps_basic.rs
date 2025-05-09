use anchor_lang::prelude::*;
use anchor_spl::{
    token::Token,
    token_interface::{transfer_checked, Mint, TokenAccount, TransferChecked},
};

use crate::{
    errors::MyError, GameClient, Manager, Player, Player2Info, RpsBasicGame,
    DEFAULT_RPS_BASIC_TIMEOUT,
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct JoinRpsBasicData {
    pub player2_choice: u8,
}

#[derive(Accounts)]
pub struct JoinRpsBasic<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        mut,
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
        mut,
        associated_token::mint = manager.usdc_mint,
        associated_token::authority = player2
    )]
    pub player2_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        seeds=[
            "game_client".as_bytes(),
            &game_client.signer.key().to_bytes()
        ],
        bump = game_client.bump,
        has_one = signer,
    )]
    game_client: Account<'info, GameClient>,
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

impl<'info> JoinRpsBasic<'info> {
    pub fn join_rps_basic(&mut self, join_game_data: JoinRpsBasicData) -> Result<()> {
        let game = &mut self.rps_basic_game;

        let cpi_accounts = TransferChecked {
            mint: self.usdc_mint.to_account_info(),
            from: self.player2_ata.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.player2.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        transfer_checked(cpi_context, game.amount, self.usdc_mint.decimals)?;

        game.player2 = Some(Player2Info {
            key: self.player2.key(),
            choice: join_game_data.player2_choice,
        });

        game.timeout = Some(Clock::get()?.unix_timestamp + DEFAULT_RPS_BASIC_TIMEOUT);
        Ok(())
    }
}
