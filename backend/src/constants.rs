//! Monetary and system constants
//! 
//! CONSTITUTIONAL: These are frozen at deployment

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/// WAD precision (1e18)
pub const WAD: u64 = 1_000_000_000_000_000_000;

/// UE mint per epoch (696 UE, fixed forever)
pub const UE_MINT_PER_EPOCH: &str = "696000000000000000000";

/// Rate index starts at 1.0 (1e18)
pub const RATE_INDEX_START: &str = "1000000000000000000";

/// Base decay rate (1% per epoch = 0.01e18)
pub const BASE_DECAY: &str = "10000000000000000";

/// Minimum decay rate (0.5% per epoch = 0.005e18)
pub const MIN_DECAY: &str = "5000000000000000";

/// Maximum decay rate (2% per epoch = 0.02e18)
pub const MAX_DECAY: &str = "20000000000000000";

/// Maximum decay change per epoch (0.1% = 0.001e18)
pub const MAX_DECAY_CHANGE: &str = "1000000000000000";

/// Conversion delay epochs (1 epoch = 30 days)
pub const CONVERSION_DELAY_EPOCHS: i32 = 1;

/// Conversion fee (0.5% = 50 bps)
pub const CONVERSION_FEE_BPS: u32 = 50;

/// Conversion cap per person per epoch (1000 UE)
pub const CONVERSION_CAP_UE: &str = "1000000000000000000000";

/// Epoch length in seconds (30 days)
pub const EPOCH_LENGTH_SECONDS: i64 = 30 * 24 * 60 * 60;

/// PersonId length (32 bytes)
pub const PERSON_ID_LENGTH: usize = 32;

