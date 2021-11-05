//! Linear filters.
//! 
//! + 1-pole high-pass and low-pass topologies
//! + 2-pole filters, based on an Svf core
//! + Generic FIR filters   TODO:

use std::f64::consts;

use crate::traits::Process;
use crate::chain;
use crate::utils::conversion::{f_to_omega, r_to_q, db_to_gain};


// === BASICS ===
// TODO: move differentiators and integrators into a new sub-module "filter_primitives"
// and add the non-linear integrators as well.

// TODO: replace with sample-rate aware diff
/// Discrete sample differentiator
/// 
/// # Caveats
/// This is not sample-rate aware, i.e. it does not scale the volume, i.e. it is
/// not a derivative.
#[deprecated(since="0.2.0", note="Deprecated since it is not sample rate aware.
Use DiffFwd, DiffC or  instead.")]
pub struct Diff { z1: f64 }

impl Diff {
    pub fn new() -> Self {
        Self {
            z1: 0.0,
        }
    }
}

impl Process<f64> for Diff {
    fn step(&mut self, input: f64) -> f64 {
        let ret = input - self.z1;
        self.z1 = input;
        ret
    } 
}

/// Simple forward-differentiator for numeric derivatives. 
/// 
/// Acts as a 6dB/oct tilt that boosts high frequencies and cuts low frequencies.
/// 
/// # Caveats
/// * Deviates from ideal derivative curve near nyquist, good for most audio
/// applications, but not for accurate numeric derivatives. For that use `DiffHQ`
/// 
/// * For noisy audio, i.e real-world audio the noise in the signal is significantly
/// amplified (as expected from differentiation), however using `DiffC`, which
/// performs centered differentiation, attenuates frequencies above fs/4 and
/// reduces noise.
pub struct DiffFwd {
    x_z1: f64,
    sr_scale: f64,
}

impl DiffFwd {
    pub fn new() -> Self {
        Self {
            x_z1: 0.0,
            sr_scale: 1.0,
        }
    }

    pub fn set_sr(&mut self, sr: f64) {
        self.sr_scale = sr / 44100.0;
    }
}

impl Process<f64> for DiffFwd {
    fn step(&mut self, input: f64) -> f64 {
        let ret = (input - self.x_z1) * self.sr_scale;
        self.x_z1 = input;
        ret
    }
}


// TODO: DiffC for centered finite differentiation

// TODO: DiffHQ for minimizing equation error


// TODO: replace with sample-rate aware leaky int
/// Discrete leaky sample integrator
/// 
/// # Caveats
/// This is not sample-rate aware, i.e. it does not scale the volume, i.e. it is
/// not a continuous integral.
#[deprecated(since="0.2.0", note="Deprecated because it does not scale with
the sample rate. Use Integ, IntegLeaky or IntegSafe; or IntegClip, IntegSat, IntegOTA from the
non_lin_filter submodule.")]
pub struct LeakyInt {
    z1: f64,
    pass: f64,
}

impl LeakyInt {
    pub fn new(leak: f64, state: f64) -> Self {
        Self {
            z1: state,
            pass: 1.0 - leak, 
        }
    }
}

impl Process<f64> for LeakyInt {
    fn step(&mut self, input: f64) -> f64 {
        self.z1 = self.z1 * self.pass + input;
        self.z1
    }
}


/// Simple numerical integrator, with no extra bells and whistles.
/// 
/// # Caveats
/// * Bad numerical precision, for approximating integrals use `IntegTrpz` or
/// `IntegRK4`.
/// * Will overflow with DC signals, for overflow protection use `IntegLeaky` or
/// `IntegSafe` or remove DC signals with `BlockDC`.
pub struct Integ {
    y_z1: f64,
    inv_sr_scale: f64,
}

impl Integ {
    pub fn new() -> Self {
        Self {
            y_z1: 0.0,
            inv_sr_scale: 1.0,
        }
    }

    pub fn set_sr(&mut self, sr: f64) {
        self.inv_sr_scale = 44100.0 / sr;
    }
}

impl Process<f64> for Integ {
    fn step(&mut self, input: f64) -> f64 {
        self.y_z1 = self.inv_sr_scale * input + self.y_z1;
        self.y_z1
    }
}


// TODO: IntegLeaky leaky integrator
// TODO: IntegSafe  overflow protected integrator
// TODO: IntegTrpz  trapezoidal rule integrator
// TODO: IntegRK4   4th-order runge kutta integrator


// === SVF CORE 2-POLE FILTERS ===

// 2-pole state variable filter. Implements lowpass, highpass, notch and
// bandpass filters with shared state. Is used internally by filter processes.
struct SvfCore {
    pub lp: f64,
    pub hp: f64,
    pub bs: f64,
    pub bp: f64,
    pub cutoff: f64,
    pub res:    f64,
    pub sr:     f64,
}

impl SvfCore {
    /// Initialize filter state variables.
    fn new() -> Self {
        Self {
            lp: 0.0,
            hp: 0.0,
            bs: 0.0,
            bp: 0.0,
            cutoff: 0.0,
            res:    0.0,
            sr:     0.0,
        }
    }

    // Compute lowpass, highpass, notch and bandpass filtering of input with
    // variable resonance and cutoff.
    fn filter(&mut self, input: f64) {
        // Pre-process
        let f = 2.0 * (std::f64::consts::PI * self.cutoff / self.sr).sin();
        let q = (1.0 - self.res) * 2.0;

        // Filtering
        let lp = self.bp * f + self.lp;
        let hp = input - lp - q * self.bp;
        let bs = hp + lp;
        let bp = hp * f + self.bp;

        // Update state:
        self.lp = lp;
        self.hp = hp;
        self.bs = bs; 
        self.bp = bp;
    }
}


/// 2-pole Svf low-pass filter
/// TODO: test this
pub struct SvfLowPass {
    core: SvfCore,
    pub cutoff: f64,
    pub res: f64,
}

impl Process<f64> for SvfLowPass {
    fn step(&mut self, input: f64) -> f64 {
        self.core.cutoff = self.cutoff;
        self.core.res = self.res;
        self.core.filter(input);
        return self.core.lp;
    }
}

impl SvfLowPass {
    pub fn new() -> Self {
        Self {
            core: SvfCore::new(),
            cutoff: 0.0,
            res: 0.0,
        }
    }

    pub fn set_sr(&mut self, sr: f64) {
        self.core.sr = sr;
    }
}


/// 2-pole Svf high-pass filter
/// TODO: test this
pub struct SvfHighPass {
    core: SvfCore,
    pub cutoff: f64,
    pub res: f64,
}

impl Process<f64> for SvfHighPass {
    fn step(&mut self, input: f64) -> f64 {
        self.core.cutoff = self.cutoff;
        self.core.res = self.res;
        self.core.filter(input);
        self.core.hp
    }
}

impl SvfHighPass {
    pub fn new() -> Self {
        Self {
            core: SvfCore::new(),
            cutoff: 0.0,
            res: 0.0,
        }
    }

    pub fn set_sr(&mut self, sr: f64) {
        self.core.sr = sr;
    }
}


/// 2-pole Svf band-pass filter
/// TODO: test this
pub struct SvfBandPass {
    core: SvfCore,
    pub cutoff: f64,
    pub res: f64,
}

impl Process<f64> for SvfBandPass {
    fn step(&mut self, input: f64) -> f64 {
        self.core.cutoff = self.cutoff;
        self.core.res = self.res;
        self.core.filter(input);
        self.core.bp
    }
}

impl SvfBandPass {
    pub fn new() -> Self {
        Self {
            core: SvfCore::new(),
            cutoff: 0.0,
            res: 0.0,
        }
    }

    pub fn set_sr(&mut self, sr: f64) {
        self.core.sr = sr;
    }
}


/// 2-pole Svf band-stop filter
/// TODO: test this
pub struct SvfBandStop {
    core: SvfCore,
    pub cutoff: f64,
    pub res: f64,
}

impl Process<f64> for SvfBandStop {
    fn step(&mut self, input: f64) -> f64 {
        self.core.cutoff = self.cutoff;
        self.core.res = self.res;
        self.core.filter(input);
        self.core.bs
    }
}

impl SvfBandStop {
    pub fn new() -> Self {
        Self {
            core: SvfCore::new(),
            cutoff: 0.0,
            res: 0.0,
        }
    }

    pub fn set_sr(&mut self, sr: f64) {
        self.core.sr = sr;
    }
}


// === 1-POLE FILTERS ===

/// Single pole, no zero lowpass. Extremely subtle and extremely cheap
pub struct LowPass1P {
    a0: f64,
    b1: f64,
    y_z1: f64,
    two_inv_sr: f64,
}

impl LowPass1P {

    /// constructor
    ///
    /// defaults to sample_rate at 44100.0, cutoff at 0Hz.
    pub fn new() -> Self {
        Self {
            a0: 0.0,
            b1: 0.0,
            y_z1: 0.0,
            two_inv_sr: 2.0 / 44100.0,
        }
    }

    pub fn set_sr(&mut self, sr: f64) {
        self.two_inv_sr = 2.0 / sr;
    }

    /// Set 3dB cutoff point in hertz.
    pub fn set_cutoff(&mut self, cut: f64) {
        let fc = (cut * self.two_inv_sr).clamp(0.0, 1.0);
        self.b1 = (-consts::TAU * fc).exp();
        self.a0 = 1.0 - self.b1;
    }
}

impl Process<f64> for LowPass1P {
    fn step(&mut self, x: f64) -> f64 {
        self.y_z1 = self.a0 * x 
                  + self.b1 * self.y_z1;
        self.y_z1
    }
}


/// Static gentle high-pass to block DC offsets.
pub struct DcBlock { lp: LowPass1P, }

impl DcBlock {
    /// Initialize filter state variables.
    pub fn new() -> Self {
        let mut ret = Self { lp: LowPass1P::new(), };
        ret.lp.set_cutoff(10.0);
        ret
    }

    pub fn set_sr(&mut self, sr: f64) {
        self.lp.set_sr(sr);
    }
}

impl Process<f64> for DcBlock {
    fn step(&mut self, input: f64) -> f64 { 
        let lp = &mut self.lp;
        input - chain!(input => lp)
    }
}


// === BIQUAD 2-POLE FILTERS ===

struct BiquadCore {
    x_z1: f64,
    x_z2: f64,
    y_z1: f64,
    y_z2: f64,
}

impl BiquadCore {
    fn new() -> Self {
        Self {
            x_z1: 0.0,
            x_z2: 0.0,
            y_z1: 0.0,
            y_z2: 0.0,
        }
    }

    fn filter(&mut self, x: f64, a: [f64; 3], b: [f64; 3]) -> f64 {
        let a_0_rec = 1.0 / a[0];
        let a_1 = a[1];
        let a_2 = a[2];
        let b_0 = b[0];
        let b_1 = b[1];
        let b_2 = b[2];

        let res = b_0 * a_0_rec * x 
                + b_1 * a_0_rec * self.x_z1 
                + b_2 * a_0_rec * self.x_z2
                - a_1 * a_0_rec * self.y_z1
                - a_2 * a_0_rec * self.y_z2;
        
        self.x_z2 = self.x_z1;
        self.x_z1 = x;
        self.y_z2 = self.y_z1;
        self.y_z1 = res;

        res
    }
}

pub struct BiquadLowPass {
    core: BiquadCore,
    pub cutoff: f64,
    pub q: f64,
    pub sr: f64,
}

impl Process<f64> for BiquadLowPass {
    fn step(&mut self, input: f64) -> f64 {
        // clamp cutoff at nyquist
        let f = self.cutoff.clamp(0.0, self.sr/2.0);
        let omega = f_to_omega(f, self.sr);
        let c = omega.cos();
        let s = omega.sin();
        let alpha = s / (2.0 * self.q);

        let b_0 = (1.0 - c) / 2.0;
        let b_1 = 1.0 - c;
        let b_2 = b_0;
        let a_0 = 1.0 + alpha;
        let a_1 = -2.0 * c;
        let a_2 = 1.0 - alpha;

        self.core.filter(input, [a_0, a_1, a_2], [b_0, b_1, b_2])
    }
}

impl BiquadLowPass {
    pub fn new() -> Self {
        Self {
            core: BiquadCore::new(),
            cutoff: 440.0,
            q: 0.707,
            sr: 44100.0,
        }
    }

    pub fn set_sr(&mut self, sr: f64) { self.sr = sr; }

    /// Allows to set the Q-factor by giving a resonance parameter between 0 and 1
    /// resembling the resonance setting of an analog filter.
    pub fn set_res(&mut self, res: f64) { self.q = r_to_q(res) + 0.01; }
}

pub struct BiquadHighPass {
    core: BiquadCore,
    pub cutoff: f64,
    pub q: f64,
    pub sr: f64,
}

impl Process<f64> for BiquadHighPass {
    fn step(&mut self, input: f64) -> f64 {
        // clamp cutoff at nyquist
        let f = self.cutoff.clamp(0.0, self.sr/2.0);
        let omega = f_to_omega(f, self.sr);
        let c = omega.cos();
        let s = omega.sin();
        let alpha = s / (2.0 * self.q);

        let b_0 = (1.0 + c) / 2.0;
        let b_1 = -(1.0 + c);
        let b_2 = b_0;
        let a_0 = 1.0 + alpha;
        let a_1 = -2.0 * c;
        let a_2 = 1.0 - alpha;

        self.core.filter(input, [a_0, a_1, a_2], [b_0, b_1, b_2])
    }
}

impl BiquadHighPass {
    pub fn new() -> Self {
        Self {
            core: BiquadCore::new(),
            cutoff: 440.0,
            q: 0.707,
            sr: 44100.0,
        }
    }

    pub fn set_sr(&mut self, sr: f64) { self.sr = sr; }

    /// Allows to set the Q-factor by giving a resonance parameter between 0 and 1
    /// resembling the resonance setting of an analog filter.
    pub fn set_res(&mut self, res: f64) { self.q = r_to_q(res) + 0.01; }
}

pub struct BiquadBandPass {
    core: BiquadCore,
    pub cutoff: f64,
    pub q: f64,
    pub sr: f64,
}

impl Process<f64> for BiquadBandPass {
    fn step(&mut self, input: f64) -> f64 {
        // clamp cutoff at nyquist
        let f = self.cutoff.clamp(0.0, self.sr/2.0);
        let omega = f_to_omega(f, self.sr);
        let c = omega.cos();
        let s = omega.sin();
        let alpha = s / (2.0 * self.q);

        let b_0 = alpha;
        let b_1 = 0.0;
        let b_2 = -alpha;
        let a_0 = 1.0 + alpha;
        let a_1 = -2.0 * c;
        let a_2 = 1.0 - alpha;

        self.core.filter(input, [a_0, a_1, a_2], [b_0, b_1, b_2])
    }
}

impl BiquadBandPass {
    pub fn new() -> Self {
        Self {
            core: BiquadCore::new(),
            cutoff: 440.0,
            q: 0.707,
            sr: 44100.0,
        }
    }

    pub fn set_sr(&mut self, sr: f64) { self.sr = sr; }

    /// Allows to set the Q-factor by giving a resonance parameter between 0 and 1
    /// resembling the resonance setting of an analog filter.
    pub fn set_res(&mut self, res: f64) { self.q = r_to_q(res) + 0.01; }
}

pub struct BiquadNotch {
    core: BiquadCore,
    pub cutoff: f64,
    pub q: f64,
    pub sr: f64,
}

impl Process<f64> for BiquadNotch {
    fn step(&mut self, input: f64) -> f64 {
        // clamp cutoff at nyquist
        let f = self.cutoff.clamp(0.0, self.sr/2.0);
        let omega = f_to_omega(f, self.sr);
        let c = omega.cos();
        let s = omega.sin();
        let alpha = s / (2.0 * self.q);

        let b_0 = 1.0;
        let b_1 = -2.0 * c;
        let b_2 = 1.0;
        let a_0 = 1.0 + alpha;
        let a_1 = -2.0 * c;
        let a_2 = 1.0 - alpha;

        self.core.filter(input, [a_0, a_1, a_2], [b_0, b_1, b_2])
    }
}

impl BiquadNotch {
    pub fn new() -> Self {
        Self {
            core: BiquadCore::new(),
            cutoff: 440.0,
            q: 0.707,
            sr: 44100.0,
        }
    }

    pub fn set_sr(&mut self, sr: f64) { self.sr = sr; }

    /// Allows to set the Q-factor by giving a resonance parameter between 0 and 1
    /// resembling the resonance setting of an analog filter.
    pub fn set_res(&mut self, res: f64) { self.q = r_to_q(res) + 0.01; }
}

pub struct BiquadAllPass {
    core: BiquadCore,
    pub cutoff: f64,
    pub q: f64,
    pub sr: f64,
}

impl Process<f64> for BiquadAllPass {
    fn step(&mut self, input: f64) -> f64 {
        // clamp cutoff at nyquist
        let f = self.cutoff.clamp(0.0, self.sr/2.0);
        let omega = f_to_omega(f, self.sr);
        let c = omega.cos();
        let s = omega.sin();
        let alpha = s / (2.0 * self.q);

        let b_0 = 1.0 - alpha;
        let b_1 = -2.0 * c;
        let b_2 = 1.0 + alpha;
        let a_0 = 1.0 + alpha;
        let a_1 = -2.0 * c;
        let a_2 = 1.0 - alpha;

        self.core.filter(input, [a_0, a_1, a_2], [b_0, b_1, b_2])
    }
}

impl BiquadAllPass {
    pub fn new() -> Self {
        Self {
            core: BiquadCore::new(),
            cutoff: 440.0,
            q: 0.707,
            sr: 44100.0,
        }
    }

    pub fn set_sr(&mut self, sr: f64) { self.sr = sr; }

    /// Allows to set the Q-factor by giving a resonance parameter between 0 and 1
    /// resembling the resonance setting of an analog filter.
    pub fn set_res(&mut self, res: f64) { self.q = r_to_q(res) + 0.01; }
}

pub struct BiquadPeaking {
    core: BiquadCore,
    pub cutoff: f64,
    pub q: f64,
    pub sr: f64,
    pub db_gain: f64,
}

impl Process<f64> for BiquadPeaking {
    fn step(&mut self, input: f64) -> f64 {
        // clamp cutoff at nyquist
        let f = self.cutoff.clamp(0.0, self.sr/2.0);
        let omega = f_to_omega(f, self.sr);
        let c = omega.cos();
        let s = omega.sin();
        let amp = db_to_gain(self.db_gain);
        let alpha = s / (2.0 * self.q);

        let b_0 = 1.0 + alpha * amp;
        let b_1 = -2.0 * c;
        let b_2 = 1.0 - alpha * amp;
        let a_0 = 1.0 + alpha / amp;
        let a_1 = -2.0 * c;
        let a_2 = 1.0 - alpha / amp;

        self.core.filter(input, [a_0, a_1, a_2], [b_0, b_1, b_2])
    }
}

impl BiquadPeaking {
    pub fn new() -> Self {
        Self {
            core: BiquadCore::new(),
            cutoff: 440.0,
            q: 0.707,
            sr: 44100.0,
            db_gain: 0.0,
        }
    }

    pub fn set_sr(&mut self, sr: f64) { self.sr = sr; }

    /// Allows to set the Q-factor by giving a resonance parameter between 0 and 1
    /// resembling the resonance setting of an analog filter.
    pub fn set_res(&mut self, res: f64) { self.q = r_to_q(res) + 0.01; }
}

pub struct BiquadLowShelf {
    core: BiquadCore,
    pub cutoff: f64,
    pub q: f64,
    pub sr: f64,
    pub db_gain: f64,
}

impl Process<f64> for BiquadLowShelf {
    fn step(&mut self, input: f64) -> f64 {
        // clamp cutoff at nyquist
        let f = self.cutoff.clamp(0.0, self.sr/2.0);
        let omega = f_to_omega(f, self.sr);
        let c = omega.cos();
        let s = omega.sin();
        let amp = db_to_gain(self.db_gain);
        let alpha = s * 0.5 * ((amp + 1.0 / amp) * (1.0 / self.q - 1.0) + 2.0).sqrt();
        let aux_shelf = 2.0 * alpha * amp.sqrt();

        let b_0 = amp * ((amp + 1.0) - (amp - 1.0) * c + aux_shelf);
        let b_1 = 2.0 * amp * ((amp - 1.0) - (amp + 1.0) * c);
        let b_2 = amp * ((amp + 1.0) - (amp - 1.0) * c - aux_shelf);
        let a_0 = (amp + 1.0) + (amp - 1.0) * c + aux_shelf;
        let a_1 = -2.0 * ((amp - 1.0) + (amp + 1.0) * c);
        let a_2 = (amp + 1.0) + (amp - 1.0) * c - aux_shelf;

        self.core.filter(input, [a_0, a_1, a_2], [b_0, b_1, b_2])
    }
}

impl BiquadLowShelf {
    pub fn new() -> Self {
        Self {
            core: BiquadCore::new(),
            cutoff: 440.0,
            q: 0.707,
            sr: 44100.0,
            db_gain: 0.0,
        }
    }

    pub fn set_sr(&mut self, sr: f64) { self.sr = sr; }

    /// Allows to set the Q-factor by giving a resonance parameter between 0 and 1
    /// resembling the resonance setting of an analog filter.
    pub fn set_res(&mut self, res: f64) { self.q = r_to_q(res) + 0.01; }
}

pub struct BiquadHighShelf {
    core: BiquadCore,
    pub cutoff: f64,
    pub q: f64,
    pub sr: f64,
    pub db_gain: f64,
}

impl Process<f64> for BiquadHighShelf {
    fn step(&mut self, input: f64) -> f64 {
        // clamp cutoff at nyquist
        let f = self.cutoff.clamp(0.0, self.sr/2.0);
        let omega = f_to_omega(f, self.sr);
        let c = omega.cos();
        let s = omega.sin();
        let amp = db_to_gain(self.db_gain);
        let alpha = s * 0.5 * ((amp + 1.0 / amp) * (1.0 / self.q - 1.0) + 2.0).sqrt();
        let aux_shelf = 2.0 * alpha * amp.sqrt();

        let b_0 = amp * ((amp + 1.0) + (amp - 1.0) * c + aux_shelf);
        let b_1 = 2.0 * amp * ((amp - 1.0) + (amp + 1.0) * c);
        let b_2 = amp * ((amp + 1.0) + (amp - 1.0) * c - aux_shelf);
        let a_0 = (amp + 1.0) - (amp - 1.0) * c + aux_shelf;
        let a_1 = -2.0 * ((amp - 1.0) - (amp + 1.0) * c);
        let a_2 = (amp + 1.0) - (amp - 1.0) * c - aux_shelf;

        self.core.filter(input, [a_0, a_1, a_2], [b_0, b_1, b_2])
    }
}

impl BiquadHighShelf {
    pub fn new() -> Self {
        Self {
            core: BiquadCore::new(),
            cutoff: 440.0,
            q: 0.707,
            sr: 44100.0,
            db_gain: 0.0,
        }
    }

    pub fn set_sr(&mut self, sr: f64) { self.sr = sr; }

    /// Allows to set the Q-factor by giving a resonance parameter between 0 and 1
    /// resembling the resonance setting of an analog filter.
    pub fn set_res(&mut self, res: f64) { self.q = r_to_q(res) + 0.01; }
}


/* FIXME: this has some borrow errors to fix
/// Nested all-pass filter, with dynamic corner frequency
pub struct NestedAP {
    next: Option<Box<Self>>,
    delay_line: VecDeque<f64>,
    corner_f: f64,
    sr: f64,
}

impl NestedAP {
    /// Initialize filter state
    pub fn new(depth: u16) -> Self {
        let mut ret = Self { 
            next: None, 
            delay_line: VecDeque::with_capacity(96000),
            corner_f: 440.0,
            sr: 44100.0
        };
        if depth > 1 {
            ret.next = Some(Box::new(Self::new(depth - 1)));
        }
        return ret;
    }

    pub fn set_sr(&mut self, sr: f64) {
        self.sr = sr;
        match &self.next {
            None => {},
            Some(n) => {
                n.set_sr(sr);
            }
        };
    }
}
*/