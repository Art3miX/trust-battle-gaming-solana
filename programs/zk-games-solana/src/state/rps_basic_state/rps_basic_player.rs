use anchor_lang::prelude::*;

#[account]
#[derive(Default, InitSpace)]
pub struct RpsBasicPlayer {
    pub total_games: u64,
    pub total_draws: u64,
    pub total_wins: u64,
    pub total_losses: u64,
    pub total_choices: [u64; 3],
    pub bump: u8,
}

impl RpsBasicPlayer {
    fn add_game(&mut self, choice: u8) {
        self.total_games += 1;
        self.total_choices[choice as usize] += 1;
    }

    pub fn add_win(&mut self, choice: u8) {
        self.add_game(choice);
        self.total_wins += 1;
    }

    pub fn add_lose(&mut self, choice: u8) {
        self.add_game(choice);
        self.total_losses += 1;
    }

    pub fn add_draw(&mut self, choice: u8) {
        self.add_game(choice);
        self.total_draws += 1;
    }
}
