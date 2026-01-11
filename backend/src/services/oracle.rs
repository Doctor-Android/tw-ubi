//! Oracle service

use crate::models::oracle::{RegionOracleData, OracleSubmission};
use crate::utils::errors::UBIError;
use sqlx::PgPool;
use log::info;

pub struct OracleService {
    pool: PgPool,
}

impl OracleService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    /// Submit oracle data
    pub async fn submit_data(&self, submission: OracleSubmission) -> Result<(), UBIError> {
        info!("Oracle submission for region {}: {}", submission.region_id, submission.basket_index_wad);
        
        // Calculate inflation rate (simplified: assume 0% if no previous data)
        let inflation_rate = "0".to_string();
        
        // Update or insert oracle data
        sqlx::query!(
            r#"
            INSERT INTO region_oracle_data (region_id, current_basket_index_wad, current_inflation_rate_wad, last_update_timestamp)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (region_id)
            DO UPDATE SET
                current_basket_index_wad = $2,
                current_inflation_rate_wad = $3,
                last_update_timestamp = $4
            "#,
            submission.region_id,
            submission.basket_index_wad,
            inflation_rate,
            chrono::Utc::now().timestamp()
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    /// Get oracle data for region
    pub async fn get_data(&self, region_id: i32) -> Result<Option<RegionOracleData>, UBIError> {
        let data = sqlx::query_as!(
            RegionOracleData,
            "SELECT region_id, current_basket_index_wad, current_inflation_rate_wad, last_update_timestamp FROM region_oracle_data WHERE region_id = $1",
            region_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(data)
    }
}

