//! Application configuration

use std::env;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
    pub mfa_issuer: String,
    pub global_salt: String,
    pub genesis_timestamp: i64,
}

impl Config {
    pub fn load() -> Result<Self> {
        dotenv::dotenv().ok();
        
        Ok(Config {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost/ubi".to_string()),
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "change-me-in-production".to_string()),
            mfa_issuer: env::var("MFA_ISSUER").unwrap_or_else(|_| "TW-UBI".to_string()),
            global_salt: env::var("GLOBAL_SALT")
                .unwrap_or_else(|_| "change-me-in-production".to_string()),
            genesis_timestamp: env::var("GENESIS_TIMESTAMP")
                .unwrap_or_else(|_| chrono::Utc::now().timestamp().to_string())
                .parse()
                .unwrap_or_else(|_| chrono::Utc::now().timestamp()),
        })
    }
}

