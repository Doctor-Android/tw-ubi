//! Treasury service

use crate::models::treasury::Treasury;
use crate::utils::errors::UBIError;
use sqlx::PgPool;

pub struct TreasuryService {
    pool: PgPool,
}

impl TreasuryService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    /// Get treasury balance
    pub async fn get_balance(&self) -> Result<String, UBIError> {
        let balance = sqlx::query_scalar!(
            "SELECT balance_bu FROM treasury ORDER BY id DESC LIMIT 1"
        )
        .fetch_optional(&self.pool)
        .await?
        .unwrap_or_else(|| "0".to_string());
        
        Ok(balance)
    }
}

