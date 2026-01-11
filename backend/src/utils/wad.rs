//! WAD math (fixed-point arithmetic, 1e18)
//! 
//! STRICT: Integer math only, NO FLOATS

use crate::constants::WAD;
use rust_decimal::Decimal;
use anyhow::Result;

/// Multiply two WAD values: (a * b) / WAD
pub fn mul_wad(a: &str, b: &str) -> Result<String> {
    let a_dec: Decimal = a.parse()?;
    let b_dec: Decimal = b.parse()?;
    let wad = Decimal::from(WAD);
    let result = (a_dec * b_dec) / wad;
    Ok(result.to_string())
}

/// Divide two WAD values: (a * WAD) / b
pub fn div_wad(a: &str, b: &str) -> Result<String> {
    let a_dec: Decimal = a.parse()?;
    let b_dec: Decimal = b.parse()?;
    let wad = Decimal::from(WAD);
    let result = (a_dec * wad) / b_dec;
    Ok(result.to_string())
}

/// Apply decay: rateIndex *= (1 - decayRate) for N epochs
pub fn apply_decay(rate_index: &str, decay_rate: &str, epochs: i32) -> Result<String> {
    let rate_dec: Decimal = rate_index.parse()?;
    let decay_dec: Decimal = decay_rate.parse()?;
    let wad = Decimal::from(WAD);
    let decay_factor = wad - decay_dec;
    
    let mut result = rate_dec;
    for _ in 0..epochs {
        result = (result * decay_factor) / wad;
    }
    
    Ok(result.to_string())
}

/// Convert amount to WAD: amount * WAD
pub fn to_wad(amount: Decimal) -> String {
    let wad = Decimal::from(WAD);
    (amount * wad).to_string()
}

/// Convert WAD to amount: wad / WAD
pub fn from_wad(wad_str: &str) -> Result<Decimal> {
    let wad_dec: Decimal = wad_str.parse()?;
    let wad = Decimal::from(WAD);
    Ok(wad_dec / wad)
}

