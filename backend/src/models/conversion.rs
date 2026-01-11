//! Conversion models

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

/// Pending conversion
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PendingConversion {
    pub id: i64,
    pub person_id: Vec<u8>,
    pub amount_ue: String,
    pub amount_bu: String,
    pub rate_index: String,
    pub unlock_epoch: i32,
    pub status: String, // pending, unlocked, claimed
    pub created_at: DateTime<Utc>,
}

/// Conversion request
#[derive(Debug, Deserialize)]
pub struct ConversionRequest {
    pub amount_ue: String,
    pub min_bu_out: String,
}

/// Conversion response
#[derive(Debug, Serialize)]
pub struct ConversionResponse {
    pub conversion_id: i64,
    pub amount_ue: String,
    pub amount_bu: String,
    pub unlock_epoch: i32,
}

impl From<PendingConversion> for ConversionResponse {
    fn from(conv: PendingConversion) -> Self {
        ConversionResponse {
            conversion_id: conv.id,
            amount_ue: conv.amount_ue,
            amount_bu: conv.amount_bu,
            unlock_epoch: conv.unlock_epoch,
        }
    }
}

