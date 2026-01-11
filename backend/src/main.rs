//! TW-UBI Backend Server
//! 
//! Constitutional invariants enforced throughout

mod api;
mod config;
mod constants;
mod models;
mod services;
mod utils;
mod events;

use actix_web::{web, App, HttpServer};
use config::Config;
use log::info;
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    let config = Config::load()
        .map_err(|e| {
            eprintln!("Failed to load configuration: {}", e);
            std::process::exit(1);
        })?;
    
    // Initialize database pool
    let pool = PgPool::connect(&config.database_url)
        .await
        .map_err(|e| {
            eprintln!("Failed to connect to database: {}", e);
            std::process::exit(1);
        })?;
    
    info!("Starting TW-UBI System on {}:{}", config.host, config.port);
    // Database URL not logged for security
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(config.clone()))
            .service(api::health::health)
            .service(api::users::register_user)
            .service(api::users::reset_wallet)
            .service(api::ubi::claim_ubi)
            .service(api::conversion::request_conversion)
            .service(api::conversion::claim_conversion)
            .service(api::oracle::submit_oracle)
            .service(api::admin::export_state)
            .service(api::balances::get_ue_balance)
            .service(api::balances::get_bu_balance)
            .service(api::balances::get_rate_index)
            .service(api::pending_conversions::get_pending_conversions)
    })
    .bind((config.host.clone(), config.port))?
    .run()
    .await
}

