#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

pub mod errors;

pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

declare_id!("2ky84E3Znacsztj8PcLz4fAMtrGJPdTnZVmC64Tq9KgK");

// TODO: Replace with admin pubkey
const ADMIN_PUBKEY: Pubkey = pubkey!("2ky84E3Znacsztj8PcLz4fAMtrGJPdTnZVmC64Tq9KgK");

const DEFAULT_TIMEOUT: i64 = 2629800;

#[program]
pub mod zk_games_solana {
    use super::*;

    pub fn register_game_type(ctx: Context<RegisterGameType>, game_type_id: u32) -> Result<()> {
        ctx.accounts.game_type.set_inner(GameType {
            id: game_type_id,
            bump: ctx.bumps.game_type,
        });
        Ok(())
    }

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
            result: None,
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

        game.timeout = Some(Clock::get()?.unix_timestamp + DEFAULT_TIMEOUT);
        Ok(())
    }
}
