//! Conversion endpoints

use actix_web::{post, web, HttpResponse, Result};
use crate::models::conversion::{ConversionRequest, ConversionResponse};
use crate::services::conversion::ConversionService;
use crate::services::registry::RegistryService;
use crate::services::rate_index::RateIndexService;
use crate::config::Config;
use sqlx::PgPool;
use log::info;

#[post("/api/conversion/request")]
pub async fn request_conversion(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    wallet: web::Header<String>,
    req: web::Json<ConversionRequest>,
) -> Result<HttpResponse> {
    let registry = RegistryService::new(pool.get_ref().clone());
    let rate_index = RateIndexService::new(pool.get_ref().clone(), config.genesis_timestamp);
    let conversion_service = ConversionService::new(
        pool.get_ref().clone(),
        registry,
        rate_index,
        config.genesis_timestamp,
    );
    
    match conversion_service.request_conversion(&wallet.to_string(), req.into_inner()).await {
        Ok(response) => {
            info!("Conversion requested: {}", wallet.to_string());
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string()
            })))
        }
    }
}

#[post("/api/conversion/claim")]
pub async fn claim_conversion(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    wallet: web::Header<String>,
    conversion_id: web::Path<i64>,
) -> Result<HttpResponse> {
    let registry = RegistryService::new(pool.get_ref().clone());
    let rate_index = RateIndexService::new(pool.get_ref().clone(), config.genesis_timestamp);
    let conversion_service = ConversionService::new(
        pool.get_ref().clone(),
        registry,
        rate_index,
        config.genesis_timestamp,
    );
    
    match conversion_service.claim_converted_bu(&wallet.to_string(), conversion_id.into_inner()).await {
        Ok(amount_bu) => {
            info!("BU claimed: {} BU", amount_bu);
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "amount_bu": amount_bu
            })))
        }
        Err(e) => {
            Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string()
            })))
        }
    }
}

