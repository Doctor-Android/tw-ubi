//! Pending conversions query endpoint

use actix_web::{get, web, HttpResponse, Result};
use crate::models::conversion::PendingConversion;
use sqlx::PgPool;
use hex;

#[get("/api/conversions/pending")]
pub async fn get_pending_conversions(
    pool: web::Data<PgPool>,
    wallet: web::Header<String>,
) -> Result<HttpResponse> {
    // Get user by wallet
    let user = sqlx::query!(
        "SELECT person_id FROM users WHERE wallet_address = $1",
        wallet.to_string()
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    if let Some(user) = user {
        let conversions = sqlx::query_as!(
            PendingConversion,
            r#"
            SELECT id, person_id, amount_ue, amount_bu, rate_index, unlock_epoch, status, created_at
            FROM pending_conversions
            WHERE person_id = $1 AND status IN ('pending', 'unlocked')
            ORDER BY unlock_epoch ASC
            "#,
            user.person_id
        )
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
        
        Ok(HttpResponse::Ok().json(conversions))
    } else {
        Ok(HttpResponse::Ok().json(Vec::<PendingConversion>::new()))
    }
}

