//! Personhood registry service
//! 
//! CONSTITUTIONAL: Identity = personId, NOT wallet

use crate::models::user::{User, PersonId};
use crate::utils::{errors::UBIError, mfa};
use sqlx::PgPool;
use hex;
use log::info;

pub struct RegistryService {
    pool: PgPool,
}

impl RegistryService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    /// Register a new person
    pub async fn register_person(
        &self,
        person_id_hex: &str,
        wallet_address: &str,
        region_id: i32,
        expiry_epoch: i32,
    ) -> Result<User, UBIError> {
        // Decode personId
        let person_id = hex::decode(person_id_hex)
            .map_err(|_| UBIError::InvalidPersonId)?;
        
        if person_id.len() != 32 {
            return Err(UBIError::InvalidPersonId);
        }
        
        // Check if already registered
        let existing = sqlx::query_as!(
            User,
            "SELECT person_id, wallet_address, region_id, expiry_epoch, last_reset_epoch, is_active, mfa_secret, created_at FROM users WHERE person_id = $1",
            person_id.as_slice()
        )
        .fetch_optional(&self.pool)
        .await?;
        
        if existing.is_some() {
            return Err(UBIError::Other("Person already registered".to_string()));
        }
        
        // Generate MFA secret
        let mfa_secret = mfa::generate_mfa_secret();
        
        // Insert user
        sqlx::query!(
            r#"
            INSERT INTO users (person_id, wallet_address, region_id, expiry_epoch, last_reset_epoch, is_active, mfa_secret)
            VALUES ($1, $2, $3, $4, 0, true, $5)
            "#,
            person_id.as_slice(),
            wallet_address,
            region_id,
            expiry_epoch,
            mfa_secret
        )
        .execute(&self.pool)
        .await?;
        
        // Initialize UE balance
        sqlx::query!(
            "INSERT INTO ue_balances (wallet_address, balance) VALUES ($1, '0')",
            wallet_address
        )
        .execute(&self.pool)
        .await?;
        
        info!("Person registered: {} (region {})", person_id_hex, region_id);
        
        // Emit event
        let event_data = serde_json::to_value(crate::events::PersonRegisteredEvent {
            person_id: person_id_hex.to_string(),
            wallet_address: wallet_address.to_string(),
            region_id,
            expiry_epoch,
        }).unwrap();
        
        // Emit event
        sqlx::query!(
            "INSERT INTO events (event_type, event_data) VALUES ($1, $2)",
            "PersonRegistered",
            event_data
        )
        .execute(&self.pool)
        .await?;
        
        // Get created user
        let user = sqlx::query_as!(
            User,
            "SELECT person_id, wallet_address, region_id, expiry_epoch, last_reset_epoch, is_active, mfa_secret, created_at FROM users WHERE person_id = $1",
            person_id.as_slice()
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(user)
    }
    
    /// Reset wallet (requires MFA)
    pub async fn reset_wallet(
        &self,
        person_id_hex: &str,
        new_wallet: &str,
        mfa_code: &str,
    ) -> Result<(), UBIError> {
        let person_id = hex::decode(person_id_hex)
            .map_err(|_| UBIError::InvalidPersonId)?;
        
        // Get user
        let user = sqlx::query_as!(
            User,
            "SELECT person_id, wallet_address, region_id, expiry_epoch, last_reset_epoch, is_active, mfa_secret, created_at FROM users WHERE person_id = $1",
            person_id.as_slice()
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(UBIError::UserNotFound)?;
        
        // Verify MFA
        let mfa_secret = user.mfa_secret.ok_or(UBIError::MFAVerificationFailed)?;
        if !mfa::verify_mfa_code(&mfa_secret, mfa_code) {
            return Err(UBIError::MFAVerificationFailed);
        }
        
        // Transfer UE balance
        let old_balance = sqlx::query_scalar!(
            "SELECT balance FROM ue_balances WHERE wallet_address = $1",
            user.wallet_address
        )
        .fetch_optional(&self.pool)
        .await?
        .unwrap_or_else(|| "0".to_string());
        
        // Update old wallet balance to 0
        sqlx::query!(
            "UPDATE ue_balances SET balance = '0' WHERE wallet_address = $1",
            user.wallet_address
        )
        .execute(&self.pool)
        .await?;
        
        // Set new wallet balance
        sqlx::query!(
            r#"
            INSERT INTO ue_balances (wallet_address, balance)
            VALUES ($1, $2)
            ON CONFLICT (wallet_address) DO UPDATE SET balance = $2
            "#,
            new_wallet,
            old_balance
        )
        .execute(&self.pool)
        .await?;
        
        // Update user wallet
        // Get genesis timestamp from config (simplified - should be passed in)
        let config = crate::config::Config::load()
            .map_err(|_| UBIError::Other("Config error".to_string()))?;
        let epoch = crate::utils::epoch::current_epoch(config.genesis_timestamp);
        
        sqlx::query!(
            r#"
            UPDATE users
            SET wallet_address = $1, last_reset_epoch = $2
            WHERE person_id = $3
            "#,
            new_wallet,
            epoch,
            person_id.as_slice()
        )
        .execute(&self.pool)
        .await?;
        
        info!("Wallet reset: {} -> {}", user.wallet_address, new_wallet);
        
        // Emit event
        let event_data = serde_json::to_value(crate::events::WalletResetEvent {
            person_id: person_id_hex.to_string(),
            old_wallet: user.wallet_address,
            new_wallet: new_wallet.to_string(),
        }).unwrap();
        
        sqlx::query!(
            "INSERT INTO events (event_type, event_data) VALUES ($1, $2)",
            "WalletReset",
            event_data
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    /// Get user by wallet
    pub async fn get_user_by_wallet(&self, wallet: &str) -> Result<Option<User>, UBIError> {
        let user = sqlx::query_as!(
            User,
            "SELECT person_id, wallet_address, region_id, expiry_epoch, last_reset_epoch, is_active, mfa_secret, created_at FROM users WHERE wallet_address = $1",
            wallet
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(user)
    }
    
    /// Get last claimed epoch
    pub async fn get_last_claimed_epoch(&self, person_id: &[u8], region_id: i32) -> Result<i32, UBIError> {
        let epoch = sqlx::query_scalar!(
            "SELECT epoch FROM last_claimed_epoch WHERE person_id = $1 AND region_id = $2",
            person_id,
            region_id
        )
        .fetch_optional(&self.pool)
        .await?
        .unwrap_or(0);
        
        Ok(epoch)
    }
}

