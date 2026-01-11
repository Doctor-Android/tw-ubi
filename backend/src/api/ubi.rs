//! UBI endpoints

use actix_web::{post, web, HttpResponse, Result};
use crate::models::claim::ClaimResponse;
use crate::services::ubi::UBIService;
use crate::services::registry::RegistryService;
use crate::config::Config;
use sqlx::PgPool;
use log::info;

#[post("/api/ubi/claim")]
pub async fn claim_ubi(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    wallet: web::Header<String>,
) -> Result<HttpResponse> {
    let registry = RegistryService::new(pool.get_ref().clone());
    let ubi_service = UBIService::new(
        pool.get_ref().clone(),
        registry,
        config.genesis_timestamp,
    );
    
    match ubi_service.claim_ubi(&wallet.to_string()).await {
        Ok(response) => {
            info!("UBI claimed: {}", wallet.to_string());
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string()
            })))
        }
    }
}

