pub mod rps_basic_game;
pub use rps_basic_game::*;

pub mod rps_basic_player;
pub use rps_basic_player::*;

pub enum GameResult {
    Player1,
    Player2,
    Draw,
}

/// Calculate the result of 2 RPS choices where
/// ```
/// 0 = Rock
/// 1 = Paper
/// 2 = Scissors
/// ```
pub fn calculate_result(choice_1: u8, choice_2: u8) -> GameResult {
    match (choice_1, choice_2) {
        (1, 0) | (2, 1) | (0, 2) => GameResult::Player1,
        (0, 1) | (1, 2) | (2, 0) => GameResult::Player2,
        _ => GameResult::Draw,
    }
}

pub fn calculate_fee(amount: u64, client_bps: u16, platform_bps: u16) -> (u64, u64, u64) {
    let client_amount = amount
        .checked_mul(client_bps as u64)
        .expect("Client fee mul overflow")
        .checked_div(10000)
        .unwrap();

    let platform_amount = amount
        .checked_mul(platform_bps as u64)
        .expect("Platform fee mul overflow")
        .checked_div(10000)
        .unwrap();

    let final_amount = amount
        .checked_sub(client_amount)
        .expect("Final amount client fee sub overflow")
        .checked_sub(platform_amount)
        .expect("Final amount platform fee sub overflow");

    (final_amount, client_amount, platform_amount)
}
