//! Rate index service
//! 
//! CONSTITUTIONAL: Conversion power decays via rateIndex

use crate::models::rate_index::RateIndex;
use crate::utils::{epoch::current_epoch, errors::UBIError, wad};
use crate::constants::{
    RATE_INDEX_START, BASE_DECAY, MIN_DECAY, MAX_DECAY, MAX_DECAY_CHANGE
};
use sqlx::PgPool;
use rust_decimal::Decimal;
use log::info;

pub struct RateIndexService {
    pool: PgPool,
    genesis_timestamp: i64,
}

impl RateIndexService {
    pub fn new(pool: PgPool, genesis_timestamp: i64) -> Self {
        Self {
            pool,
            genesis_timestamp,
        }
    }
    
    /// Roll rate index to current epoch
    pub async fn roll_rate_index(&self, region_id: i32) -> Result<(), UBIError> {
        let epoch = current_epoch(self.genesis_timestamp);
        
        // Get current rate index
        let rate_data = sqlx::query!(
            "SELECT rate_index_wad, last_epoch, current_decay_rate_wad, last_decay_update_epoch FROM rate_index WHERE region_id = $1",
            region_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(data) = rate_data {
            if epoch <= data.last_epoch {
                return Ok(()); // Already up to date
            }
            
            // Update decay rate first
            self.update_decay_rate(region_id, epoch).await?;
            
            // Get updated decay rate
            let decay_rate = sqlx::query_scalar!(
                "SELECT current_decay_rate_wad FROM rate_index WHERE region_id = $1",
                region_id
            )
            .fetch_one(&self.pool)
            .await?
            .unwrap_or_else(|| BASE_DECAY.to_string());
            
            // Apply decay for epochs since last update
            let epochs_to_apply = epoch - data.last_epoch;
            let new_rate_index = wad::apply_decay(&data.rate_index_wad, &decay_rate, epochs_to_apply)?;
            
            // Update rate index
            sqlx::query!(
                r#"
                UPDATE rate_index
                SET rate_index_wad = $1, last_epoch = $2
                WHERE region_id = $3
                "#,
                &new_rate_index,
                epoch,
                region_id
            )
            .execute(&self.pool)
            .await?;
        } else {
            // Initialize rate index (starts at 1.0)
            sqlx::query!(
                r#"
                INSERT INTO rate_index (region_id, rate_index_wad, last_epoch, current_decay_rate_wad, last_decay_update_epoch)
                VALUES ($1, $2, $3, $4, $3)
                "#,
                region_id,
                RATE_INDEX_START,
                epoch,
                BASE_DECAY
            )
            .execute(&self.pool)
            .await?;
        }
        
        Ok(())
    }
    
    /// Update decay rate based on oracle inflation signal
    async fn update_decay_rate(&self, region_id: i32, epoch: i32) -> Result<(), UBIError> {
        // Get last decay update epoch
        let last_update = sqlx::query_scalar!(
            "SELECT last_decay_update_epoch FROM rate_index WHERE region_id = $1",
            region_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(last_epoch) = last_update {
            if epoch <= last_epoch {
                return Ok(()); // Already updated
            }
        }
        
        // Get inflation rate from oracle
        let inflation_rate = self.get_inflation_rate(region_id).await?;
        
        // Calculate target decay rate: baseDecay - (k * inflation)
        // k = 0.5 (hardcoded for simplicity)
        let k_wad = "500000000000000000"; // 0.5e18
        let k_inflation = wad::mul_wad(k_wad, &inflation_rate)?;
        let base_decay: Decimal = BASE_DECAY.parse()
            .map_err(|_| UBIError::Other("Invalid base decay".to_string()))?;
        let k_inf_decimal: Decimal = k_inflation.parse()
            .map_err(|_| UBIError::Other("Invalid k inflation".to_string()))?;
        
        let target_decimal = base_decay - (k_inf_decimal / Decimal::from(1_000_000_000_000_000_000u64));
        
        // Clamp to bounds
        let min_decay: Decimal = MIN_DECAY.parse()
            .map_err(|_| UBIError::Other("Invalid min decay".to_string()))?;
        let max_decay: Decimal = MAX_DECAY.parse()
            .map_err(|_| UBIError::Other("Invalid max decay".to_string()))?;
        
        let target_clamped = if target_decimal < min_decay {
            min_decay
        } else if target_decimal > max_decay {
            max_decay
        } else {
            target_decimal
        };
        
        // Apply max change per epoch
        let current_decay = sqlx::query_scalar!(
            "SELECT current_decay_rate_wad FROM rate_index WHERE region_id = $1",
            region_id
        )
        .fetch_optional(&self.pool)
        .await?
        .unwrap_or_else(|| BASE_DECAY.to_string());
        
        let current: Decimal = current_decay.parse()
            .map_err(|_| UBIError::Other("Invalid current decay".to_string()))?;
        let max_change: Decimal = MAX_DECAY_CHANGE.parse()
            .map_err(|_| UBIError::Other("Invalid max change".to_string()))?;
        let epochs_since_update = epoch - last_update.unwrap_or(epoch);
        let max_total_change = max_change * Decimal::from(epochs_since_update);
        
        let final_target = if target_clamped < current - max_total_change {
            current - max_total_change
        } else if target_clamped > current + max_total_change {
            current + max_total_change
        } else {
            target_clamped
        };
        
        // Update decay rate
        sqlx::query!(
            r#"
            UPDATE rate_index
            SET current_decay_rate_wad = $1, last_decay_update_epoch = $2
            WHERE region_id = $3
            "#,
            &final_target.to_string(),
            epoch,
            region_id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    /// Get rate index for region
    pub async fn get_rate_index(&self, region_id: i32) -> Result<String, UBIError> {
        // Ensure rate index is up to date
        self.roll_rate_index(region_id).await?;
        
        let rate = sqlx::query_scalar!(
            "SELECT rate_index_wad FROM rate_index WHERE region_id = $1",
            region_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(rate.unwrap_or_else(|| "0".to_string()))
    }
    
    /// Get inflation rate from oracle
    async fn get_inflation_rate(&self, region_id: i32) -> Result<String, UBIError> {
        let inflation = sqlx::query_scalar!(
            "SELECT current_inflation_rate_wad FROM region_oracle_data WHERE region_id = $1",
            region_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(inflation.unwrap_or_else(|| "0".to_string()))
    }
}

