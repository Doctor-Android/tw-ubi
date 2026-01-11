//! User/Personhood models
//! 
//! CONSTITUTIONAL: Identity = personId, NOT wallet

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

/// PersonId is an opaque 32-byte identifier
pub type PersonId = [u8; 32];

/// User registration record
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub person_id: Vec<u8>, // personId as bytes
    pub wallet_address: String,
    pub region_id: i32,
    pub expiry_epoch: i32,
    pub last_reset_epoch: i32,
    pub is_active: bool,
    pub mfa_secret: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// User registration request
#[derive(Debug, Deserialize)]
pub struct RegisterUserRequest {
    pub person_id: String, // Hex-encoded personId
    pub wallet_address: String,
    pub region_id: i32,
    pub expiry_epoch: i32,
    pub attestation_sig: String, // EIP-712 signature
}

/// Wallet reset request (requires MFA)
#[derive(Debug, Deserialize)]
pub struct ResetWalletRequest {
    pub person_id: String,
    pub new_wallet: String,
    pub mfa_code: String,
}

/// User response
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub person_id: String,
    pub wallet_address: String,
    pub region_id: i32,
    pub expiry_epoch: i32,
    pub is_active: bool,
    pub mfa_enabled: bool,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            person_id: hex::encode(&user.person_id),
            wallet_address: user.wallet_address,
            region_id: user.region_id,
            expiry_epoch: user.expiry_epoch,
            is_active: user.is_active,
            mfa_enabled: user.mfa_secret.is_some(),
        }
    }
}

