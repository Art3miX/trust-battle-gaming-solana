#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

pub mod errors;

pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

declare_id!("2ky84E3Znacsztj8PcLz4fAMtrGJPdTnZVmC64Tq9KgK");

// Replace with admin pubkey
const ADMIN_PUBKEY: Pubkey = pubkey!("3xoJZkhxuzKpKATL7UhskTA17uBuEnMeuLAqhovETHg4");

const DEFAULT_RPS_BASIC_TIMEOUT: i64 = 2629800;

// verifying key of our rps basic program
const VK_RPS_BASIC_COMPLETE: &str =
    "0x00c4cf1292d6730be2cfdebe7a064a26bd09db12a6f5a547a46db8e72c72acd8";

#[program]
pub mod zk_games_solana {
    use sp1_solana::verify_proof;

    use crate::errors::MyError;

    use super::*;

    pub fn register_game_client(
        ctx: Context<RegisterGameClient>,
        game_client_data: GameClientData,
    ) -> Result<()> {
        ctx.accounts.game_client.set_inner(GameClient {
            name: game_client_data.name,
            signer: game_client_data.signer,
            bump: ctx.bumps.game_client,
        });
        Ok(())
    }

    pub fn register_player(ctx: Context<RegisterPlayer>, player_data: PlayerData) -> Result<()> {
        ctx.accounts.player.set_inner(Player {
            username: player_data.username,
            login_hash: player_data.login_hash,
            bump: ctx.bumps.player,
        });
        Ok(())
    }

    pub fn register_player_rps_basic(ctx: Context<RegisterPlayerRpsBasic>) -> Result<()> {
        ctx.accounts.player_rps_basic.set_inner(RpsBasicPlayer {
            bump: ctx.bumps.player_rps_basic,
            ..Default::default()
        });
        Ok(())
    }

    pub fn init_rps_basic(
        ctx: Context<InitRpsBasic>,
        init_rps_basic_data: InitRpsBasicData,
    ) -> Result<()> {
        ctx.accounts.rps_basic_game.set_inner(RpsBasicGame {
            id: init_rps_basic_data.id,
            player1: Player1Info {
                key: ctx.accounts.player1.key(),
                choice_hash: init_rps_basic_data.choice_hash,
            },
            player2: None,
            timeout: None,
            game_client: ctx.accounts.game_client.key(),
            bump: ctx.bumps.rps_basic_game,
        });
        Ok(())
    }

    pub fn join_rps_basic(
        ctx: Context<JoinRpsBasic>,
        join_rps_basic_data: JoinRpsBasicData,
    ) -> Result<()> {
        let game = &mut ctx.accounts.rps_basic_game;
        game.player2 = Some(Player2Info {
            key: ctx.accounts.player2.key(),
            choice: join_rps_basic_data.player2_choice,
        });

        game.timeout = Some(Clock::get()?.unix_timestamp + DEFAULT_RPS_BASIC_TIMEOUT);
        Ok(())
    }

    pub fn complete_rps_basic(
        ctx: Context<CompleteRpsBasic>,
        complete_rps_basic_data: CompleteRpsBasicData,
    ) -> Result<()> {
        let vk = sp1_solana::GROTH16_VK_4_0_0_RC3_BYTES;
        let game = &ctx.accounts.rps_basic_game;

        // Get public input for verification
        let public_public: Vec<u8> = zk_games_types::RpsBasicPublic {
            client_pubkey: ctx.accounts.game_client.key().to_string(),
            game_id: game.id,
            choice_hash: game.player1.choice_hash,
            choice: complete_rps_basic_data.player1_choice,
        }
        .into();

        msg!("{:?}", ctx.accounts.game_client.key().to_string());
        msg!("{:?}", game.id);
        msg!("{:?}", game.player1.choice_hash);
        msg!("{:?}", complete_rps_basic_data.player1_choice);

        // Verify proof
        verify_proof(
            &complete_rps_basic_data.proof,
            &public_public,
            &VK_RPS_BASIC_COMPLETE,
            vk,
        )
        .map_err(|x| {
            msg!("{:?}", x);
            MyError::RpsBasicProofVerify
        })?;

        let player1_rps = &mut ctx.accounts.player1_rps_basic;
        let player2_rps = &mut ctx.accounts.player2_rps_basic;

        let player1_choice = complete_rps_basic_data.player1_choice;
        let player2_choice = game.player2.clone().unwrap().choice;

        let game_result = calculate_result(player1_choice, player2_choice);

        match game_result {
            GameResult::Player1 => {
                player1_rps.add_win(player1_choice);
                player2_rps.add_lose(player2_choice);
            }
            GameResult::Player2 => {
                player1_rps.add_lose(player1_choice);
                player2_rps.add_win(player2_choice);
            }
            GameResult::Draw => {
                player1_rps.add_draw(player1_choice);
                player2_rps.add_draw(player2_choice);
            }
        }
        Ok(())
    }
}
