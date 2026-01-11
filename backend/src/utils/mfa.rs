//! MFA (Multi-Factor Authentication) for wallet rotation
//! 
//! CONSTITUTIONAL: Wallet rotation requires MFA success

use totp_lite::{totp_custom, Sha1};

/// Generate TOTP secret for user
pub fn generate_mfa_secret() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bytes: [u8; 20] = rng.gen();
    base32::encode(base32::Alphabet::RFC4648 { padding: false }, &bytes)
}

/// Verify TOTP code
pub fn verify_mfa_code(secret: &str, code: &str) -> bool {
    let secret_bytes = match base32::decode(base32::Alphabet::RFC4648 { padding: false }, secret) {
        Some(bytes) => bytes,
        None => return false,
    };
    
    let timestamp = chrono::Utc::now().timestamp();
    let step = 30; // 30 second window
    
    // Check current and previous window (for clock skew)
    for offset in [0, -1, 1] {
        let time = timestamp + (offset * step);
        let expected = totp_custom::<Sha1>(&secret_bytes, 6, 0, step, time);
        if expected == code {
            return true;
        }
    }
    
    false
}

/// Generate QR code data URI for TOTP
pub fn generate_mfa_qr_data(secret: &str, issuer: &str, account: &str) -> String {
    format!("otpauth://totp/{}:{}?secret={}&issuer={}", issuer, account, secret, issuer)
}

