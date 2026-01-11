//! Error types

use thiserror::Error;

#[derive(Error, Debug)]
pub enum UBIError {
    #[error("Wallet not active: {0}")]
    WalletNotActive(String),
    
    #[error("Already claimed epoch {0}")]
    AlreadyClaimed(i32),
    
    #[error("Registration expired")]
    RegistrationExpired,
    
    #[error("User not found")]
    UserNotFound,
    
    #[error("Insufficient balance")]
    InsufficientBalance,
    
    #[error("Conversion cap exceeded")]
    ConversionCapExceeded(String),
    
    #[error("Slippage too high: expected {0}, got {1}")]
    SlippageTooHigh(String, String),
    
    #[error("Rate index not initialized for region {0}")]
    RateIndexNotInitialized(i32),
    
    #[error("MFA verification failed")]
    MFAVerificationFailed,
    
    #[error("Invalid personId")]
    InvalidPersonId,
    
    #[error("Unauthorized")]
    Unauthorized,
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Other error: {0}")]
    Other(String),
}

