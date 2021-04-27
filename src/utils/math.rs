//! Various mathematical pure functions that are not included in Rust's standard
//! library and fast versions of cmath functions.


/// Fast sigmoid. This is not the same as tanh, but quite close. It uses the
/// infamous quake inverse square root, but f64. I left the comments in for
/// extra analog warmth
pub fn fast_sigmoid(x: f64) -> f64 {
    let q = 1.0 + x * x;

    let i = q.to_bits();                                // evil floating point bit level hacking
    let i = 0x5fe6eb50c7b537a9 - (i >> 1);              // what the fuck?
    let y = f64::from_bits(i);
    let inv_sq = y * (1.5 - 0.5 * q * y * y);           // 1st iteration
 // let inv_sq_2 = inv_sq * (1.5 - 0.5 * q * y * y);    // 2nd iteration, this can be removed
    inv_sq * x
}

/// Crossfade between two values, i.e. linear interpolation.
/// The crossfading parameter is clamped between 0 and 1.
/// This function is inlined for hot use inside of interpolation algorithms.
#[inline(always)]
pub fn x_fade(a: f64, x: f64, b: f64) -> f64 {
    let x_clamp = x.clamp(0.0, 1.0);
    a * (1.0 - x_clamp) + b * x_clamp
}

/// Quadratic interpolation, for high quality (but slower) sample interpolation
pub fn quad_interp(y_m: f64, y_0: f64, y_1: f64, x_01: f64) -> f64 {
    let x_01_clamp = x_01.clamp(0.0, 1.0);
    let x_01_2 = x_01 * x_01;
    let l_m = (x_01_2 - x_01) * 0.5;
    let l_0 = -x_01_2 + 1.0;
    let l_1 = (x_01_2 + x_01) * 0.5;
    y_m*l_m + y_0*l_0 + y_1*l_1
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
pub fn pre_post_gains(x: f64) -> (f64, f64) {
    if x < 0.0 {
        (1.0 / (1.0 - x), 1.0 - x)
    } else {
        (1.0 + x, 1.0 / (1.0 + x))
    }
}