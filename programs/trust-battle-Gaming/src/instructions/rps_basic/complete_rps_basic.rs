use anchor_lang::prelude::*;
use anchor_spl::{
    token::Token,
    token_interface::{transfer_checked, Mint, TokenAccount, TransferChecked},
};
use sp1_solana::verify_proof;

use crate::{
    calculate_fee, calculate_result, errors::MyError, GameClient, GameResult, Manager, Player,
    RpsBasicGame, RpsBasicPlayer, VK_RPS_BASIC_COMPLETE,
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CompleteRpsBasicData {
    pub proof: Vec<u8>,
    pub player1_choice: u8,
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
    pub rps_basic_game: Box<Account<'info, RpsBasicGame>>,
    #[account(
        seeds = [
            "player".as_bytes(),
            player1.username.as_bytes()
        ],
        bump = player1.bump
    )]
    pub player1: Account<'info, Player>,
    #[account(
        mut,
        seeds = [
            "rps_basic_player".as_bytes(),
            player1.username.as_bytes()
        ],
        bump = player1_rps_basic.bump
    )]
    pub player1_rps_basic: Box<Account<'info, RpsBasicPlayer>>,
    #[account(
        mut,
        associated_token::mint = manager.usdc_mint,
        associated_token::authority = player1
    )]
    pub player1_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        seeds = [
            "player".as_bytes(),
            player2.username.as_bytes()
        ],
        bump = player2.bump,
    )]
    pub player2: Account<'info, Player>,
    #[account(
        mut,
        seeds = [
            "rps_basic_player".as_bytes(),
            player2.username.as_bytes()
        ],
        bump = player2_rps_basic.bump
    )]
    pub player2_rps_basic: Box<Account<'info, RpsBasicPlayer>>,
    #[account(
        mut,
        associated_token::mint = manager.usdc_mint,
        associated_token::authority = player2
    )]
    pub player2_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        seeds = [
            "game_client".as_bytes(),
            &signer.key().to_bytes()
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
    #[account(
        mut,
        associated_token::mint = manager.usdc_mint,
        associated_token::authority = manager.platform_key,
    )]
    pub platform_ata: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

impl<'info> CompleteRpsBasic<'info> {
    pub fn complete_rps_basic(&mut self, complete_game_data: CompleteRpsBasicData) -> Result<()> {
        let vk = sp1_solana::GROTH16_VK_4_0_0_RC3_BYTES;
        let game = &self.rps_basic_game;

        // Get public input for verification
        let public_public: Vec<u8> = zk_games_types::RpsBasicPublic {
            client_pubkey: self.game_client.key().to_string(),
            game_id: game.id,
            choice_hash: game.player1.choice_hash,
            choice: complete_game_data.player1_choice,
        }
        .into();

        // Verify proof
        verify_proof(
            &complete_game_data.proof,
            &public_public,
            VK_RPS_BASIC_COMPLETE,
            vk,
        )
        .map_err(|x| {
            msg!("{:?}", x);
            MyError::RpsBasicProofVerify
        })?;

        let player1_choice = complete_game_data.player1_choice;
        let player2_choice = game
            .player2
            .clone()
            .expect("Player2 must be set to complete game")
            .choice;

        let game_result = calculate_result(player1_choice, player2_choice);

        let (winning_amount, client_amount, platform_amount) = calculate_fee(
            game.amount
                .checked_mul(2)
                .expect("Mul game winning amount overflow"),
            self.manager.client_fee,
            self.manager.platform_fee,
        );

        // Transfer platform fee
        self.transfer_from_vault(self.platform_ata.clone(), platform_amount)?;

        // Transfer client fee
        self.transfer_from_vault(self.game_client_ata.clone(), client_amount)?;

        match game_result {
            GameResult::Player1 => {
                // Transfer winning amount to player1
                self.transfer_from_vault(self.player1_ata.clone(), winning_amount)?;

                self.player1_rps_basic.add_win(player1_choice);
                self.player2_rps_basic.add_lose(player2_choice);
            }
            GameResult::Player2 => {
                // Transfer winning amount to player2
                self.transfer_from_vault(self.player2_ata.clone(), winning_amount)?;

                self.player1_rps_basic.add_lose(player1_choice);
                self.player2_rps_basic.add_win(player2_choice);
            }
            GameResult::Draw => {
                // TODO: Create rematch logic
                // Split winning
                let split_amount = winning_amount
                    .checked_div(2)
                    .expect("Split winning amount zero");

                self.transfer_from_vault(self.player1_ata.clone(), split_amount)?;
                self.transfer_from_vault(self.player2_ata.clone(), split_amount)?;

                self.player1_rps_basic.add_draw(player1_choice);
                self.player2_rps_basic.add_draw(player2_choice);
            }
        }

        Ok(())
    }

    fn transfer_from_vault(
        &mut self,
        acc_info: InterfaceAccount<'info, TokenAccount>,
        amount: u64,
    ) -> Result<()> {
        let manager_pda_seeds = &["manager".as_bytes(), &[self.manager.bump]];
        let manager_pda_seeds = &[&manager_pda_seeds[..]];

        let cpi_accounts = TransferChecked {
            mint: self.usdc_mint.to_account_info(),
            from: self.vault.to_account_info(),
            to: acc_info.to_account_info(),
            authority: self.manager.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, manager_pda_seeds);
        transfer_checked(cpi_context, amount, self.usdc_mint.decimals)
    }
}
