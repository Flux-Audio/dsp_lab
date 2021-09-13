//! Various mathematical pure functions that are not included in Rust's standard
//! library and fast versions of cmath functions.

use std::f64::consts;
use fastapprox::fast::{sinfull, cosfull};

const FRAC_1_TAU: f64 = 1.0 / consts::TAU;

/// Fast sigmoid. This is not the same as tanh, but quite close. It uses the
/// infamous quake inverse square root, but f64. I left the comments in for
/// extra analog warmth
pub fn fast_sigmoid(x: f64) -> f64 {
    let q = 1.0 + x * x;

    let i = q.to_bits();                                // evil floating point bit level hacking
    let i = 0x5fe6eb50c7b537a9 - (i >> 1);              // what the fuck?
    let y = f64::from_bits(i);
    let inv_sq = y * (1.5 - 0.5 * q * y * y);           // 1st iteration
    let inv_sq_2 = inv_sq * (1.5 - 0.5 * q * y * y);    // 2nd iteration, this can be removed
    inv_sq_2 * x
}

/// Fast rounding, is not correct for values like 0.5, 1.5, 2.5, ...
pub fn fast_round(x: f64) -> f64 {
    let t = (x.abs() + 0.5).floor();
    t * x.signum()
}


pub fn fast_sin(x: f64) -> f64 {
    let cos_core = |x| {
        let x2 = x * x;
        let x4 = x2 * x2;
        let x8 = x4 * x4;
        let term1 = (-2.7236370439787708e-7 * x2 + 2.4799852696610628e-5) * x8;
        let term2 = (-1.3888885054799695e-3 * x2 + 4.1666666636943683e-2) * x4;
        let term3 = -4.9999999999963024e-1 * x2 + 1.0;
        term1 + term2 + term3
    };

    let sin_core = |x| {
        let x2 = x * x;
        let x4 = x2 * x2;
        let term1 = (2.7181216275479732e-6 * x2 - 1.9839312269456257e-4) * x4;
        let term2 = 8.3333293048425631e-3 * x2 - 1.6666666640797048e-1;
        (term1 + term2) * x2 * x + x
    };

    let q = fast_round(x * 6.3661977236758138e-1); // divide by tau and round to get quadrant
    let quadrant = q as u8; // TODO: check if u64 is faster because of allignment

    // t1, t2 and t3 are just three steps of one computation, to avoid mut borrow checks
    let t1 = x - q * 1.5707963267923333e+00;
    let t2 = t1 - q * 2.5633441515945189e-12;

    // TODO: check if branchless implementation is faster, i.e. (quadrant & 1) as f64 * cos_core() ...
    let t3 = if quadrant & 1 > 0 {
        cos_core(t2)
    } else {
        sin_core(t2)
    };
    if quadrant & 2 > 0 {
        -t3
    } else {
        t3
    }
}


/// Crossfade between two values, i.e. linear interpolation.
/// The crossfading parameter is clamped between 0 and 1.
/// This function is inlined for hot use inside of interpolation algorithms.
#[inline]
pub fn x_fade(a: f64, x: f64, b: f64) -> f64 {
    let x_clamp = x.clamp(0.0, 1.0);
    a * (1.0 - x_clamp) + b * x_clamp
}

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
    x.abs() / (1.0 + x.abs().powf(1.0 / s)).powf(s) * x.signum()
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