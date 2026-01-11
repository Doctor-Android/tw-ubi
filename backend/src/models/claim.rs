//! UBI claim models

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

/// UBI claim record
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UBIClaim {
    pub id: i64,
    pub person_id: Vec<u8>,
    pub epoch: i32,
    pub amount_ue: String,
    pub claimed_at: DateTime<Utc>,
}

/// Claim response
#[derive(Debug, Serialize)]
pub struct ClaimResponse {
    pub epoch: i32,
    pub amount_ue: String,
    pub claimed_at: DateTime<Utc>,
}

impl From<UBIClaim> for ClaimResponse {
    fn from(claim: UBIClaim) -> Self {
        ClaimResponse {
            epoch: claim.epoch,
            amount_ue: claim.amount_ue,
            claimed_at: claim.claimed_at,
        }
    }
}

