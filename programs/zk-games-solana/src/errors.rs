use anchor_lang::error_code;

#[error_code]
pub enum MyError {
    #[msg("Player2 cannot be the same as Player1")]
    RpsBasicSamePlayer,
    #[msg("Someone already joined this game")]
    RpsBasicGameJoined,
}
