//! Utility functions for audio calculations.
//!
//! This module provides helper functions for converting between different
//! audio level representations (dB, multiplier, deflection).

/// Converts a multiplier value to decibels (dB).
///
/// # Arguments
/// * `mul` - The multiplier value (typically 0.0 to 1.0, but can be higher for amplification)
///
/// # Returns
/// The equivalent dB value
///
/// # Example
/// ```
/// use libobs_wrapper::audio::mul_to_db;
///
/// // A multiplier of 1.0 is 0 dB (no change)
/// assert_eq!(mul_to_db(1.0), 0.0);
///
/// // A multiplier of 0.5 is approximately -6 dB
/// let db = mul_to_db(0.5);
/// assert!((db - (-6.0)).abs() < 0.1);
/// ```
pub fn mul_to_db(mul: f32) -> f32 {
    unsafe { libobs::obs_mul_to_db(mul) }
}

/// Converts a decibel (dB) value to a multiplier.
///
/// # Arguments
/// * `db` - The dB value
///
/// # Returns
/// The equivalent multiplier value
///
/// # Example
/// ```
/// use libobs_wrapper::audio::db_to_mul;
///
/// // 0 dB is a multiplier of 1.0 (no change)
/// assert_eq!(db_to_mul(0.0), 1.0);
///
/// // -6 dB is approximately a multiplier of 0.5
/// let mul = db_to_mul(-6.0);
/// assert!((mul - 0.5).abs() < 0.01);
///
/// // -∞ dB (or very low dB) is a multiplier of 0.0 (silence)
/// let mul = db_to_mul(-100.0);
/// assert!(mul < 0.001);
/// ```
pub fn db_to_mul(db: f32) -> f32 {
    unsafe { libobs::obs_db_to_mul(db) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_mul_conversion() {
        // Test that conversions are inverses (within floating point precision)
        let original_mul = 0.5;
        let db = mul_to_db(original_mul);
        let converted_mul = db_to_mul(db);
        assert!((original_mul - converted_mul).abs() < 0.0001);

        // Test 0 dB = 1.0 multiplier
        assert_eq!(db_to_mul(0.0), 1.0);

        // Test that 1.0 multiplier = 0 dB
        assert_eq!(mul_to_db(1.0), 0.0);
    }

    #[test]
    fn test_db_values() {
        // -6 dB should be approximately 0.5 multiplier
        let mul = db_to_mul(-6.0);
        assert!((mul - 0.5).abs() < 0.01);

        // Very negative dB should approach 0
        let mul = db_to_mul(-96.0);
        assert!(mul < 0.0001);
    }
}
