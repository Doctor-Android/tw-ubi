//! Conversion service
//! 
//! CONSTITUTIONAL: UE balances do NOT decay
//! Conversion power decays via rateIndex

use crate::models::conversion::{PendingConversion, ConversionRequest, ConversionResponse};
use crate::services::registry::RegistryService;
use crate::services::rate_index::RateIndexService;
use crate::utils::{epoch::current_epoch, errors::UBIError, wad};
use crate::constants::{CONVERSION_CAP_UE, CONVERSION_DELAY_EPOCHS, CONVERSION_FEE_BPS};
use crate::events::{emit_event, ConversionRequestedEvent};
use sqlx::PgPool;
use rust_decimal::Decimal;
use log::info;
use hex;

pub struct ConversionService {
    pool: PgPool,
    registry: RegistryService,
    rate_index: RateIndexService,
    genesis_timestamp: i64,
}

impl ConversionService {
    pub fn new(
        pool: PgPool,
        registry: RegistryService,
        rate_index: RateIndexService,
        genesis_timestamp: i64,
    ) -> Self {
        Self {
            pool,
            registry,
            rate_index,
            genesis_timestamp,
        }
    }
    
    /// Request UE→BU conversion
    pub async fn request_conversion(
        &self,
        wallet: &str,
        req: ConversionRequest,
    ) -> Result<ConversionResponse, UBIError> {
        info!("Conversion request: {} UE from wallet {}", req.amount_ue, wallet);
        
        // Start transaction
        let mut tx = self.pool.begin().await?;
        
        // Get user
        let user = self.registry
            .get_user_by_wallet(wallet)
            .await?
            .ok_or(UBIError::WalletNotActive(wallet.to_string()))?;
        
        let epoch = current_epoch(self.genesis_timestamp);
        
        // Check per-epoch cap
        let converted_this_epoch = self.get_converted_this_epoch(&user.person_id, epoch).await?;
        let converted_decimal: Decimal = converted_this_epoch.parse()
            .map_err(|_| UBIError::Other("Invalid converted amount".to_string()))?;
        let amount_decimal: Decimal = req.amount_ue.parse()
            .map_err(|_| UBIError::Other("Invalid amount".to_string()))?;
        let total_converted = converted_decimal + amount_decimal;
        let cap_wad: Decimal = CONVERSION_CAP_UE.parse()
            .map_err(|_| UBIError::Other("Invalid cap".to_string()))?;
        
        if total_converted > cap_wad {
            return Err(UBIError::ConversionCapExceeded(
                format!("Exceeds per-epoch cap of {}", CONVERSION_CAP_UE)
            ));
        }
        
        // Roll rate index to current epoch
        self.rate_index.roll_rate_index(user.region_id).await?;
        
        // Get current rate index
        let rate_index_value = self.rate_index.get_rate_index(user.region_id).await?;
        if rate_index_value.is_empty() || rate_index_value == "0" {
            return Err(UBIError::RateIndexNotInitialized(user.region_id));
        }
        
        // Calculate BU amount (with fee)
        let fee_amount = wad::mul_wad(&req.amount_ue, &CONVERSION_FEE_BPS.to_string())?;
        let fee_decimal: Decimal = fee_amount.parse()
            .map_err(|_| UBIError::Other("Invalid fee".to_string()))?;
        let amount_ue_decimal: Decimal = req.amount_ue.parse()
            .map_err(|_| UBIError::Other("Invalid amount".to_string()))?;
        
        let amount_ue_after_fee = (amount_ue_decimal - (fee_decimal / Decimal::from(10000))).to_string();
        let amount_bu = wad::mul_wad(&amount_ue_after_fee, &rate_index_value)?;
        
        // Slippage protection
        let min_bu_decimal: Decimal = req.min_bu_out.parse()
            .map_err(|_| UBIError::Other("Invalid min_bu_out".to_string()))?;
        let amount_bu_decimal: Decimal = amount_bu.parse()
            .map_err(|_| UBIError::Other("Invalid amount_bu".to_string()))?;
        
        if amount_bu_decimal < min_bu_decimal {
            return Err(UBIError::SlippageTooHigh(req.min_bu_out, amount_bu));
        }
        
        // Check UE balance
        let ue_balance = sqlx::query_scalar!(
            "SELECT balance FROM ue_balances WHERE wallet_address = $1",
            wallet
        )
        .fetch_optional(&mut *tx)
        .await?
        .unwrap_or_else(|| "0".to_string());
        
        let balance_decimal: Decimal = ue_balance.parse()
            .map_err(|_| UBIError::Other("Invalid balance".to_string()))?;
        
        if balance_decimal < amount_ue_decimal {
            return Err(UBIError::InsufficientBalance);
        }
        
        // Burn UE (deduct from balance)
        let new_balance = (balance_decimal - amount_ue_decimal).to_string();
        sqlx::query!(
            "UPDATE ue_balances SET balance = $1 WHERE wallet_address = $2",
            &new_balance,
            wallet
        )
        .execute(&mut *tx)
        .await?;
        
        // Record conversion
        let unlock_epoch = epoch + CONVERSION_DELAY_EPOCHS;
        sqlx::query!(
            r#"
            INSERT INTO pending_conversions (person_id, amount_ue, amount_bu, rate_index, unlock_epoch, status)
            VALUES ($1, $2, $3, $4, $5, 'pending')
            "#,
            user.person_id.as_slice(),
            &req.amount_ue,
            &amount_bu,
            &rate_index_value,
            unlock_epoch
        )
        .execute(&mut *tx)
        .await?;
        
        // Update converted this epoch
        let new_converted = total_converted.to_string();
        sqlx::query!(
            r#"
            INSERT INTO converted_this_epoch (person_id, epoch, amount_ue)
            VALUES ($1, $2, $3)
            ON CONFLICT (person_id, epoch)
            DO UPDATE SET amount_ue = $3
            "#,
            user.person_id.as_slice(),
            epoch,
            &new_converted
        )
        .execute(&mut *tx)
        .await?;
        
        // Emit event
        let event_data = serde_json::to_value(crate::events::ConversionRequestedEvent {
            person_id: hex::encode(&user.person_id),
            wallet_address: wallet.to_string(),
            amount_ue: req.amount_ue.clone(),
            amount_bu: amount_bu.clone(),
            rate_index: rate_index_value.clone(),
            unlock_epoch,
        }).unwrap();
        
        emit_event(&mut *tx, "ConversionRequested", &event_data).await?;
        
        // Commit transaction
        tx.commit().await?;
        
        info!("Conversion successful: {} UE -> {} BU, unlocks at epoch {}", 
              req.amount_ue, amount_bu, unlock_epoch);
        
        // Get created conversion
        let conversion = sqlx::query_as!(
            PendingConversion,
            r#"
            SELECT id, person_id, amount_ue, amount_bu, rate_index, unlock_epoch, status, created_at
            FROM pending_conversions
            WHERE person_id = $1 AND unlock_epoch = $2
            ORDER BY created_at DESC
            LIMIT 1
            "#,
            user.person_id.as_slice(),
            unlock_epoch
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(conversion.into())
    }
    
    /// Claim unlocked BU from conversion
    pub async fn claim_converted_bu(
        &self,
        wallet: &str,
        conversion_id: i64,
    ) -> Result<String, UBIError> {
        // Get user
        let user = self.registry
            .get_user_by_wallet(wallet)
            .await?
            .ok_or(UBIError::WalletNotActive(wallet.to_string()))?;
        
        let epoch = current_epoch(self.genesis_timestamp);
        
        // Get conversion
        let conversion = sqlx::query_as!(
            PendingConversion,
            r#"
            SELECT id, person_id, amount_ue, amount_bu, rate_index, unlock_epoch, status, created_at
            FROM pending_conversions
            WHERE id = $1 AND person_id = $2
            "#,
            conversion_id,
            user.person_id.as_slice()
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(UBIError::Other("Conversion not found".to_string()))?;
        
        // Check unlock epoch
        if epoch < conversion.unlock_epoch {
            return Err(UBIError::Other(format!("Conversion unlocks at epoch {}", conversion.unlock_epoch)));
        }
        
        // Check status
        if conversion.status != "pending" && conversion.status != "unlocked" {
            return Err(UBIError::Other("Conversion already claimed".to_string()));
        }
        
        // Start transaction
        let mut tx = self.pool.begin().await?;
        
        // Update status to unlocked if needed
        if conversion.status == "pending" {
            sqlx::query!(
                "UPDATE pending_conversions SET status = 'unlocked' WHERE id = $1",
                conversion_id
            )
            .execute(&mut *tx)
            .await?;
        }
        
        // Transfer BU from treasury
        sqlx::query!(
            r#"
            UPDATE treasury
            SET balance_bu = balance_bu - $1
            WHERE balance_bu >= $1
            "#,
            &conversion.amount_bu
        )
        .execute(&mut *tx)
        .await?;
        
        // Update user BU balance
        sqlx::query!(
            r#"
            INSERT INTO bu_balances (wallet_address, balance)
            VALUES ($1, $2)
            ON CONFLICT (wallet_address)
            DO UPDATE SET balance = bu_balances.balance + $2
            "#,
            wallet,
            &conversion.amount_bu
        )
        .execute(&mut *tx)
        .await?;
        
        // Mark as claimed
        sqlx::query!(
            "UPDATE pending_conversions SET status = 'claimed' WHERE id = $1",
            conversion_id
        )
        .execute(&mut *tx)
        .await?;
        
        // Emit event
        let event_data = serde_json::to_value(crate::events::ConversionClaimedEvent {
            person_id: hex::encode(&user.person_id),
            wallet_address: wallet.to_string(),
            conversion_id,
            amount_bu: conversion.amount_bu.clone(),
        }).unwrap();
        
        emit_event(&mut *tx, "ConversionClaimed", &event_data).await?;
        
        // Commit transaction
        tx.commit().await?;
        
        info!("BU claimed: {} BU to wallet {}", conversion.amount_bu, wallet);
        
        Ok(conversion.amount_bu)
    }
    
    /// Get converted amount this epoch
    async fn get_converted_this_epoch(&self, person_id: &[u8], epoch: i32) -> Result<String, UBIError> {
        let amount = sqlx::query_scalar!(
            "SELECT amount_ue FROM converted_this_epoch WHERE person_id = $1 AND epoch = $2",
            person_id,
            epoch
        )
        .fetch_optional(&self.pool)
        .await?
        .unwrap_or_else(|| "0".to_string());
        
        Ok(amount)
    }
}

