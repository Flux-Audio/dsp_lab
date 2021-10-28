//! This file contains implementations for some of the functions in the math
//! module used when target architecture does not support floating point
//! instructions. These are all tradeoffs between performance and precision.

/// implementation on legacy platforms or embedded platforms that don't have a
/// dedicated floating point instruction for square roots. This is less precise
/// but avoids the overhead of a precise square root algorithm being implemented
/// as machine code rather than hardware instructions.
/// Uses quake inverse square root algorithm, adapted to 64-bit floats. Original
/// comments were kept for extra ✨*analog warmth*✨
pub fn impl_fast_sigmoid(x: f64) -> f64 {
    let q = 1.0 + x * x;

    let i = q.to_bits();                                // evil floating point bit level hacking
    let i = 0x5fe6eb50c7b537a9 - (i >> 1);              // what the fuck?
    let y = f64::from_bits(i);
    let inv_sq = y * (1.5 - 0.5 * q * y * y);           // 1st iteration
    let inv_sq_2 = inv_sq * (1.5 - 0.5 * q * y * y);    // 2nd iteration, this can be removed
    inv_sq_2 * x
}