//! User endpoints

use actix_web::{post, web, HttpResponse, Result};
use crate::models::user::{RegisterUserRequest, ResetWalletRequest, UserResponse};
use crate::services::registry::RegistryService;
use crate::utils::errors::UBIError;
use sqlx::PgPool;
use log::info;

#[post("/api/users/register")]
pub async fn register_user(
    pool: web::Data<PgPool>,
    req: web::Json<RegisterUserRequest>,
) -> Result<HttpResponse> {
    let registry = RegistryService::new(pool.get_ref().clone());
    
    match registry.register_person(
        &req.person_id,
        &req.wallet_address,
        req.region_id,
        req.expiry_epoch,
    ).await {
        Ok(user) => {
            info!("User registered: {}", req.person_id);
            Ok(HttpResponse::Ok().json(UserResponse::from(user)))
        }
        Err(e) => {
            Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string()
            })))
        }
    }
}

#[post("/api/users/reset-wallet")]
pub async fn reset_wallet(
    pool: web::Data<PgPool>,
    req: web::Json<ResetWalletRequest>,
) -> Result<HttpResponse> {
    let registry = RegistryService::new(pool.get_ref().clone());
    
    match registry.reset_wallet(
        &req.person_id,
        &req.new_wallet,
        &req.mfa_code,
    ).await {
        Ok(_) => {
            info!("Wallet reset: {}", req.person_id);
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "status": "ok"
            })))
        }
        Err(e) => {
            Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string()
            })))
        }
    }
}

