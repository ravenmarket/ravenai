use anchor_lang::prelude::*;

#[error_code]
pub enum MarketError {
    #[msg("InvalidInstructionData")]
    InvalidInstructionData,

    #[msg("InvalidAccountData")]
    InvalidAccountData,

    #[msg("IllegalOwner")]
    IllegalOwner,

    #[msg("MissingRequiredSignature")]
    MissingRequiredSignature,

    #[msg("AccountAlreadyInitialized")]
    AccountAlreadyInitialized,

    #[msg("InvalidArgument")]
    InvalidArgument,
    
    #[msg("InvalidTime")]
    InvalidTime,
    
    #[msg("InvalidMarket")]
    InvalidMarket,
    
    #[msg("InvalidPriceFeed")]
    InvalidPriceFeed,
    
    #[msg("MarketPaused")]
    MarketPaused,

    // Pyth相关
    #[msg("InvalidPythPrice")]
    InvalidPythPrice,
    #[msg("PriceStale")]
    PriceStale,
    #[msg("ConfidenceTooHigh")]
    ConfidenceTooHigh,
}
