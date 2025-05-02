use anchor_lang::prelude::*;

declare_id!("2ky84E3Znacsztj8PcLz4fAMtrGJPdTnZVmC64Tq9KgK");

#[program]
pub mod zk_games_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
