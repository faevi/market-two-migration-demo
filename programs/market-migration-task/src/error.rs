use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCodes {
    #[msg("Invalid account state")]
    InvalidAccountDiscriminator,

    #[msg("Wrong program owner")]
    WrongProgramOwner,

    #[msg("Invalid data length")]
    InvalidDataLength,
    
    #[msg("Market two flash swap fee bps already set")]
    MarketTwoFlashSwapFeeBpsAlreadySet,
}
