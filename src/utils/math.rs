//! Various mathematical pure functions that are not included in Rust's standard
//! library and fast versions of cmath functions.

use fast_math::{exp2, log2, exp_raw, exp2_raw};

/// Fast version of `powf()`.
/// 
/// Note that the performance gains are small enough that it won't be worth it
/// most of the time.
/// 
/// # Caveats
/// + it does not work properly when the base is negative
/// + it has a large error if the exponent is outside the range -128 < x < 128
pub fn fast_powf(b: f32, x: f32) -> f32 { exp2(x * log2(b)) }

/// Fast version of `tanh()`
pub fn fast_tanh(x: f32) -> f32 {
    if x > -0.075 && x < 0.075 { return x; }
    if x < -5.0 { return -1.0; }
    if x > 5.0 { return 1.0; }
    let ax  = 1.442695 * x;    // divide x by ln(2)
    let ex  = exp2_raw( ax);
    let nex = exp2_raw(-ax);
    (ex - nex) / (ex + nex)
}

/// Crossfade between two values, i.e. linear interpolation.
/// The crossfading parameter is clamped between 0 and 1.
/// This function is inlined for hot use inside of interpolation algorithms.
#[inline(always)]
pub fn x_fade(a: f32, x: f32, b: f32) -> f32 {
    let x_clamp = x.clamp(0.0, 1.0);
    a * (1.0 - x_clamp) + b * x_clamp
}

/// Gives two coefficients for pre/post-gain with equal total gain.
/// # Examples
/// ```rust
/// use dsp_lab::utils::math::pre_post_gains;
/// let drive = 0.5;    // positive values mean pre gain and post cut
/// let (pre, post) = pre_post_gains(drive);
/// assert!(pre > 1.0);
/// assert!(post < 1.0);
/// assert!(2.0*pre*post == 2.0);
/// ```
pub fn pre_post_gains(x: f32) -> (f32, f32) {
    if x < 0.0 {
        (1.0 / (1.0 - x), 1.0 - x)
    } else {
        (1.0 + x, 1.0 / (1.0 + x))
    }
}