//! Epoch calculations
//! 
//! CONSTITUTIONAL: epoch = floor((unix_ts - genesis_ts) / 30 days)

use crate::constants::EPOCH_LENGTH_SECONDS;

/// Get current epoch number
pub fn current_epoch(genesis_timestamp: i64) -> i32 {
    let now = chrono::Utc::now().timestamp();
    if now < genesis_timestamp {
        return 0;
    }
    ((now - genesis_timestamp) / EPOCH_LENGTH_SECONDS) as i32
}

/// Get epoch start timestamp
pub fn epoch_start_timestamp(epoch: i32, genesis_timestamp: i64) -> i64 {
    genesis_timestamp + (epoch as i64 * EPOCH_LENGTH_SECONDS)
}

/// Get epoch end timestamp
pub fn epoch_end_timestamp(epoch: i32, genesis_timestamp: i64) -> i64 {
    epoch_start_timestamp(epoch + 1, genesis_timestamp)
}

