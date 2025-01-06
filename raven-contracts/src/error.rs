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
    
    #[msg("MarketPaused")]
    MarketPaused,

    #[msg("InvalidPythMagic")]
    InvalidPythMagic,
    #[msg("InvalidPythVersion")]
    InvalidPythVersion,
    #[msg("InvalidPythPtype")]
    InvalidPythPtype,
    #[msg("PriceNotTrading")]
    PriceNotTrading,
    #[msg("PriceStale")]
    PriceStale,
    #[msg("ConfidenceTooHigh")]
    ConfidenceTooHigh,
    #[msg("Overflow")]
    Overflow,
}
