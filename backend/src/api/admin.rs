//! Admin endpoints (forkability)

use actix_web::{get, post, web, HttpResponse, Result};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use log::info;

#[derive(Serialize, Deserialize)]
struct SystemState {
    users: Vec<serde_json::Value>,
    claims: Vec<serde_json::Value>,
    conversions: Vec<serde_json::Value>,
    rate_indexes: Vec<serde_json::Value>,
    oracle_data: Vec<serde_json::Value>,
    treasury: serde_json::Value,
    events: Vec<serde_json::Value>,
}

#[get("/api/admin/export-state")]
pub async fn export_state(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    // Export all state
    let users = sqlx::query!("SELECT * FROM users")
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    let claims = sqlx::query!("SELECT * FROM ubi_claims")
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    let conversions = sqlx::query!("SELECT * FROM pending_conversions")
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    let rate_indexes = sqlx::query!("SELECT * FROM rate_index")
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    let oracle_data = sqlx::query!("SELECT * FROM region_oracle_data")
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    let treasury = sqlx::query!("SELECT * FROM treasury ORDER BY id DESC LIMIT 1")
        .fetch_optional(pool.get_ref())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    let events = sqlx::query!("SELECT * FROM events ORDER BY id")
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    let state = SystemState {
        users: users.into_iter().map(|r| serde_json::to_value(r).unwrap()).collect(),
        claims: claims.into_iter().map(|r| serde_json::to_value(r).unwrap()).collect(),
        conversions: conversions.into_iter().map(|r| serde_json::to_value(r).unwrap()).collect(),
        rate_indexes: rate_indexes.into_iter().map(|r| serde_json::to_value(r).unwrap()).collect(),
        oracle_data: oracle_data.into_iter().map(|r| serde_json::to_value(r).unwrap()).collect(),
        treasury: treasury.map(|r| serde_json::to_value(r).unwrap()).unwrap_or(serde_json::json!({})),
        events: events.into_iter().map(|r| serde_json::to_value(r).unwrap()).collect(),
    };
    
    info!("State exported");
    Ok(HttpResponse::Ok().json(state))
}

