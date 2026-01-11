//! Oracle endpoints

use actix_web::{post, web, HttpResponse, Result};
use crate::models::oracle::OracleSubmission;
use crate::services::oracle::OracleService;
use sqlx::PgPool;
use log::info;

#[post("/api/oracle/submit")]
pub async fn submit_oracle(
    pool: web::Data<PgPool>,
    req: web::Json<OracleSubmission>,
) -> Result<HttpResponse> {
    let oracle_service = OracleService::new(pool.get_ref().clone());
    
    match oracle_service.submit_data(req.into_inner()).await {
        Ok(_) => {
            info!("Oracle data submitted");
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

