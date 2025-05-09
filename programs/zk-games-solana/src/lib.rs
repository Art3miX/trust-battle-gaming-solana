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
    use super::*;

    pub fn init(ctx: Context<Init>, init_data: InitData) -> Result<()> {
        ctx.accounts.init(init_data, ctx.bumps.manager)
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
        ctx.accounts
            .init_rps_basic(init_rps_basic_data, ctx.bumps.rps_basic_game)
    }

    pub fn join_rps_basic(
        ctx: Context<JoinRpsBasic>,
        join_rps_basic_data: JoinRpsBasicData,
    ) -> Result<()> {
        ctx.accounts.join_rps_basic(join_rps_basic_data)
    }

    pub fn complete_rps_basic(
        ctx: Context<CompleteRpsBasic>,
        complete_game_data: CompleteRpsBasicData,
    ) -> Result<()> {
        ctx.accounts.complete_rps_basic(complete_game_data)
    }

    pub fn cancel_rps_basic(ctx: Context<CancelRpsBasic>) -> Result<()> {
        ctx.accounts.cancel_rps_basic()
    }
}
