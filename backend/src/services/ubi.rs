//! UBI issuance service
//! 
//! CONSTITUTIONAL: One person can claim UBI once per epoch
//! UE issuance is fixed: 696 UE per epoch

use crate::models::claim::{UBIClaim, ClaimResponse};
use crate::services::registry::RegistryService;
use crate::utils::{epoch::current_epoch, errors::UBIError, wad};
use crate::constants::UE_MINT_PER_EPOCH;
use crate::events::{emit_event, UBIClaimedEvent};
use sqlx::PgPool;
use log::info;
use hex;

pub struct UBIService {
    pool: PgPool,
    registry: RegistryService,
    genesis_timestamp: i64,
}

impl UBIService {
    pub fn new(pool: PgPool, registry: RegistryService, genesis_timestamp: i64) -> Self {
        Self {
            pool,
            registry,
            genesis_timestamp,
        }
    }
    
    /// Claim UBI for current epoch
    pub async fn claim_ubi(&self, wallet: &str) -> Result<ClaimResponse, UBIError> {
        info!("UBI claim request for wallet: {}", wallet);
        
        // Start transaction
        let mut tx = self.pool.begin().await?;
        
        // Get user
        let user = self.registry
            .get_user_by_wallet(wallet)
            .await?
            .ok_or(UBIError::WalletNotActive(wallet.to_string()))?;
        
        if !user.is_active {
            return Err(UBIError::Other("User not active".to_string()));
        }
        
        // Check expiry
        let epoch = current_epoch(self.genesis_timestamp);
        if epoch > user.expiry_epoch {
            return Err(UBIError::RegistrationExpired);
        }
        
        // Check if already claimed
        let last_claimed = self.registry
            .get_last_claimed_epoch(&user.person_id, user.region_id)
            .await?;
        
        if last_claimed >= epoch {
            return Err(UBIError::AlreadyClaimed(epoch));
        }
        
        // UBI amount is fixed: 696 UE per epoch
        let ubi_amount = UE_MINT_PER_EPOCH.to_string();
        
        // Record claim
        sqlx::query!(
            r#"
            INSERT INTO ubi_claims (person_id, epoch, amount_ue)
            VALUES ($1, $2, $3)
            "#,
            user.person_id.as_slice(),
            epoch,
            &ubi_amount
        )
        .execute(&mut *tx)
        .await?;
        
        // Update last claimed epoch
        sqlx::query!(
            r#"
            INSERT INTO last_claimed_epoch (person_id, region_id, epoch)
            VALUES ($1, $2, $3)
            ON CONFLICT (person_id, region_id) 
            DO UPDATE SET epoch = $3
            "#,
            user.person_id.as_slice(),
            user.region_id,
            epoch
        )
        .execute(&mut *tx)
        .await?;
        
        // Update UE balance
        sqlx::query!(
            r#"
            INSERT INTO ue_balances (wallet_address, balance)
            VALUES ($1, $2)
            ON CONFLICT (wallet_address)
            DO UPDATE SET balance = ue_balances.balance + $2
            "#,
            wallet,
            &ubi_amount
        )
        .execute(&mut *tx)
        .await?;
        
        // Emit event
        let event_data = serde_json::to_value(crate::events::UBIClaimedEvent {
            person_id: hex::encode(&user.person_id),
            wallet_address: wallet.to_string(),
            epoch,
            amount_ue: ubi_amount.clone(),
        }).unwrap();
        
        emit_event(&mut *tx, "UBIClaimed", &event_data).await?;
        
        // Commit transaction
        tx.commit().await?;
        
        info!("UBI claim successful: {} UE to wallet {} for epoch {}", ubi_amount, wallet, epoch);
        
        // Get created claim
        let claim = sqlx::query_as!(
            UBIClaim,
            "SELECT id, person_id, epoch, amount_ue, claimed_at FROM ubi_claims WHERE person_id = $1 AND epoch = $2",
            user.person_id.as_slice(),
            epoch
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(claim.into())
    }
}

