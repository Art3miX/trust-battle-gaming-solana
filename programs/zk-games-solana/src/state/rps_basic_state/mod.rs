pub mod rps_basic_game;
pub use rps_basic_game::*;

pub mod rps_basic_player;
pub use rps_basic_player::*;

pub enum GameResult {
    Player1,
    Player2,
    Draw,
}

pub fn calculate_result(choice_1: u8, choice_2: u8) -> GameResult {
    match (choice_1, choice_2) {
        (1, 0) | (2, 1) | (0, 2) => GameResult::Player1,
        (0, 1) | (1, 2) | (2, 0) => GameResult::Player2,
        _ => GameResult::Draw,
    }
}
