use anchor_lang::error_code;

#[error_code]
pub enum MyError {
    #[msg("Signer must be a registered game client")]
    SignerMustBeGameClient,

    #[msg("Player2 cannot be the same as Player1")]
    RpsBasicSamePlayer,

    #[msg("Player1 key mismatch")]
    RpsBasicPlayer1Mismatch,

    #[msg("Player2 key mismatch")]
    RpsBasicPlayer2Mismatch,

    #[msg("Someone already joined this game")]
    RpsBasicGameJoined,

    #[msg("No one joined this game yet")]
    RpsBasicGameNotJoined,

    #[msg("Proof verification failed")]
    RpsBasicProofVerify,

    #[msg("Amount is too low")]
    RpsBasicAmountTooLow,
}
