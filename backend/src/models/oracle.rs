//! Oracle models

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

/// Region oracle data
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RegionOracleData {
    pub region_id: i32,
    pub current_basket_index_wad: String,
    pub current_inflation_rate_wad: String,
    pub last_update_timestamp: i64,
}

/// Oracle submission
#[derive(Debug, Deserialize)]
pub struct OracleSubmission {
    pub region_id: i32,
    pub basket_index_wad: String,
}

