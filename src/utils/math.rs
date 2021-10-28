//! Various mathematical pure functions that are not included in Rust's standard
//! library and fast versions of cmath functions.

use std::f64::consts;
use fastapprox::fast::{sinfull, cosfull};

#[cfg(not(feature = "no_fpu"))]
use crate::utils::math_impl;
#[cfg(feature = "no_fpu")]
use crate::utils::math_impl_no_fpu as math_impl;

const FRAC_1_TAU: f64 = 1.0 / consts::TAU;


/// Fast sigmoid. This is not the same as tanh, but quite close, with the bonus
/// of being much simpler computation-wise
#[inline(always)]
pub fn fast_sigmoid(x: f64) -> f64 { math_impl::impl_fast_sigmoid(x) }

/// Fast rounding, is not correct for values like 0.5, 1.5, 2.5, ...
pub fn fast_round(x: f64) -> f64 {
    let t = (x.abs() + 0.5).floor();
    t * x.signum()
}

/// Crossfade between two values, i.e. linear interpolation.
/// The crossfading parameter is clamped between 0 and 1.
/// This function is inlined for hot use inside of interpolation algorithms.
#[inline]
pub fn x_fade(a: f64, x: f64, b: f64) -> f64 {
    let x_clamp = x.clamp(0.0, 1.0);
    a * (1.0 - x_clamp) + b * x_clamp
}

/// Linear interpolation of two samples
/// 
/// Identical to `x_fade` provided only for completeness as it follows the same
/// naming scheme of other interpolation functions.
#[inline(always)]
pub fn lin_interp(y_0: f64, y_1: f64, x_01: f64) -> f64 { x_fade(y_0, x_01, y_1) }

/// Quadratic interpolation, for high quality (but slower) sample interpolation
pub fn quad_interp(y_m: f64, y_0: f64, y_1: f64, x_01: f64) -> f64 {
    let x_01_clamp = x_01.clamp(0.0, 1.0);
    let x_01_2 = x_01_clamp * x_01_clamp;
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


/// Variable hardness clipper/saturator
/// 
/// `h` controls the hardness of the clipping, where values approaching 1.0
/// approximate a hard-clip curve, values around 0.5 resemble a `tanh()` curve, 
/// and values below 0.5 resemble a log curve.
pub fn var_clip(x: f64, h: f64) -> f64 {
    let s = (1.0 - h).clamp(1e-30, 1.0);
    x / (1.0 + x.abs().powf(1.0 / s)).powf(s)
}


/// Unbounded saturator
///
/// Has the property of not converging to a ceiling value, and also of having
/// roughly the same amount of curvature indipendently on the range of the input
/// signal, so it can be driven more and more and never fully saturate.
pub fn unbounded_sat(x: f64) -> f64 {
    x.asinh()
    // TODO: implement fast no_fpu implementation
}


// === PHASE SHAPERS ===

/// Asymmetric triangle shaper
/// 
/// Takes a phase as an input, and produces an asymmetric triangle wave, the
/// symmetry is controlled by the "asym" parameter.
/// 
/// # Caveats
/// For artifact-free operation, the phase input should be wrapped into the range
/// [0, TAU]. Not complying with this requirement, will produce some distortion,
/// but it is otherwise safe. Similarly, the asym parameter should be bounded by
/// [0, 1.0], again this is not necessary for stability.
pub fn asym_tri_shaper(mut phi: f64, asym: f64) -> f64 {
    phi *= FRAC_1_TAU;
    let two_phi = 2.0 * phi;
    let two_m_a = 2.0 - asym;
    let inv_2ma = 1.0 / two_m_a;
    if      two_phi <= inv_2ma       { two_m_a * two_phi }
    else if two_phi <= 2.0 - inv_2ma { 1.0 - two_m_a / (1.0 - asym) * (two_phi - inv_2ma) }
    else                             { two_m_a * (two_phi - 2.0) }
}

/// Parabolic sine approximation
/// 
/// Takes a phase as an input, and produces a parabolic sine approximation. It
/// is much faster than sin, but produces some extra harmonics. It's great for
/// LFO's and analog sounding sine waves. It is extremely fast.
/// 
/// # Caveats
/// For artifact-free operation, the phase input should be wrapped into the range
/// [0, TAU].
pub fn par_shaper(mut phi: f64) -> f64 {
    phi *= FRAC_1_TAU;
    let fgh = 0.25 - (phi - 0.25).abs();
    let tgh = 1.0 - 2.0*fgh.abs();
    8.0 * fgh * tgh
}

#[inline]
pub fn c_add(a: (f64, f64), b: (f64, f64)) -> (f64, f64) { (a.0 + b.0, a.1 + b.1) }

#[inline]
pub fn c_sub(a: (f64, f64), b: (f64, f64)) -> (f64, f64) { (a.0 - b.0, a.1 - b.1) }

#[inline]
pub fn c_mul(a: (f64, f64), b: (f64, f64)) -> (f64, f64) { (a.0 * b.0 - a.1 * b.1, a.0 * b.1 + a.1 * b.0) }

#[inline]
pub fn i_exp(x: f64) -> (f64, f64) { 
    let x = x as f32; 
    (cosfull(x) as f64, sinfull(x) as f64) 
}

#[inline(always)]
pub fn win_box(_: f64, _: f64) -> f64 { 1.0 }

#[inline]
pub fn win_tri(n: f64, size: f64) -> f64 {
    1.0 - ((n - size / 2.0) / ((n + 1.0) / 2.0)).abs()
}

#[inline]
pub fn win_welch(n: f64, size: f64) -> f64 {
    let size_div_2 = size / 2.0;
    let aux = (n - size_div_2) / size_div_2;
    1.0 - aux * aux
}

#[inline]
pub fn win_hann(n: f64, size: f64) -> f64 {
    let s = sinfull((consts::PI * n / size) as f32) as f64;
    s * s
}

#[inline]
pub fn win_blackman(n: f64, size: f64) -> f64 {
    let tau_n_div_size = (consts::TAU * n / size) as f32;
    0.42659 - 0.49656 * cosfull(tau_n_div_size) as f64 
            + 0.076849 * cosfull(2.0 * tau_n_div_size) as f64
}

#[inline]
pub fn win_blackman_harris(n: f64, size: f64) -> f64 {
    let tau_n_div_size = (consts::TAU * n / size) as f32;
    0.35875 - 0.48829 * cosfull(tau_n_div_size) as f64 
            + 0.14128 * cosfull(2.0 * tau_n_div_size) as f64 
            + 0.001168 * cosfull(3.0 * tau_n_div_size) as f64
}

#[inline]
pub fn win_nuttal(n: f64, size: f64) -> f64 {
    let tau_n_div_size = (consts::TAU * n / size) as f32;
    0.355768 - 0.487396 * cosfull(tau_n_div_size) as f64 
             + 0.144232 * cosfull(2.0 * tau_n_div_size) as f64 
             + 0.012604 * cosfull(3.0 * tau_n_div_size) as f64
}