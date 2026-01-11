//! Balance query endpoints

use actix_web::{get, web, HttpResponse, Result};
use sqlx::PgPool;

#[get("/api/balances/ue")]
pub async fn get_ue_balance(
    pool: web::Data<PgPool>,
    wallet: web::Header<String>,
) -> Result<HttpResponse> {
    let balance = sqlx::query_scalar!(
        "SELECT balance FROM ue_balances WHERE wallet_address = $1",
        wallet.to_string()
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "wallet": wallet.to_string(),
        "balance": balance.unwrap_or_else(|| "0".to_string())
    })))
}

#[get("/api/balances/bu")]
pub async fn get_bu_balance(
    pool: web::Data<PgPool>,
    wallet: web::Header<String>,
) -> Result<HttpResponse> {
    let balance = sqlx::query_scalar!(
        "SELECT balance FROM bu_balances WHERE wallet_address = $1",
        wallet.to_string()
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "wallet": wallet.to_string(),
        "balance": balance.unwrap_or_else(|| "0".to_string())
    })))
}

#[get("/api/balances/rate-index/{region_id}")]
pub async fn get_rate_index(
    pool: web::Data<PgPool>,
    region_id: web::Path<i32>,
    config: web::Data<crate::config::Config>,
) -> Result<HttpResponse> {
    use crate::services::rate_index::RateIndexService;
    
    let rate_index_service = RateIndexService::new(
        pool.get_ref().clone(),
        config.genesis_timestamp,
    );
    
    match rate_index_service.get_rate_index(region_id.into_inner()).await {
        Ok(rate_index) => {
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "region_id": region_id.into_inner(),
                "rate_index": rate_index
            })))
        }
        Err(e) => {
            Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string()
            })))
        }
    }
}

