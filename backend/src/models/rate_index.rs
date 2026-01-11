//! Rate index models

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Rate index per region
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RateIndex {
    pub region_id: i32,
    pub rate_index_wad: String,
    pub last_epoch: i32,
    pub current_decay_rate_wad: String,
    pub last_decay_update_epoch: i32,
}

