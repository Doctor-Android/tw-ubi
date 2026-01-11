//! Treasury models

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

/// Treasury state
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Treasury {
    pub id: i64,
    pub balance_bu: String, // WAD format
    pub created_at: DateTime<Utc>,
}

