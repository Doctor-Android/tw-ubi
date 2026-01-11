//! Event sourcing
//! 
//! CONSTITUTIONAL: All critical state changes emit append-only events
//! State must be reconstructible from events

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Event {
    pub id: i64,
    pub event_type: String,
    pub event_data: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

/// Event types (append-only)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    PersonRegistered,
    UBIClaimed,
    ConversionRequested,
    ConversionClaimed,
    WalletReset,
    RateIndexUpdated,
    OracleDataSubmitted,
}

/// Event data structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonRegisteredEvent {
    pub person_id: String, // hex-encoded
    pub wallet_address: String,
    pub region_id: i32,
    pub expiry_epoch: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UBIClaimedEvent {
    pub person_id: String,
    pub wallet_address: String,
    pub epoch: i32,
    pub amount_ue: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionRequestedEvent {
    pub person_id: String,
    pub wallet_address: String,
    pub amount_ue: String,
    pub amount_bu: String,
    pub rate_index: String,
    pub unlock_epoch: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionClaimedEvent {
    pub person_id: String,
    pub wallet_address: String,
    pub conversion_id: i64,
    pub amount_bu: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletResetEvent {
    pub person_id: String,
    pub old_wallet: String,
    pub new_wallet: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateIndexUpdatedEvent {
    pub region_id: i32,
    pub rate_index: String,
    pub decay_rate: String,
    pub epoch: i32,
}

/// Emit event to database
pub async fn emit_event(
    executor: &mut (impl sqlx::Executor<'_, Database = sqlx::Postgres> + Send),
    event_type: &str,
    event_data: &serde_json::Value,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO events (event_type, event_data) VALUES ($1, $2)",
        event_type,
        event_data
    )
    .execute(executor)
    .await?;
    Ok(())
}

