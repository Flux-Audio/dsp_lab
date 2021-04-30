//! Linear filters.
//! 
//! + 1-pole high-pass and low-pass topologies  TODO:
//! + 2-pole filters, based on an Svf core
//! + Generic FIR filters   TODO:

use std::f64::consts;

use crate::traits::Process;
use crate::chain;

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
    pub fn new() -> Self {
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
    pub fn filter(&mut self, input: f64) {
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
        return self.core.hp;
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
        return self.core.bp;
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
        return self.core.bs;
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
}


/// Single pole, no zero lowpass. Extremely subtle and extremely cheap
pub struct LowPass1P {
    b1: f64,
    z1: f64,
    inv_sr: f64,
}

impl LowPass1P {
    pub fn new(sr: f64) -> Self {
        Self {
            b1: 0.0,
            z1: 0.0,
            inv_sr: 1.0 / sr,
        }
    }

    /// Set 3dB cutoff point in hertz.
    pub fn set_cutoff(&mut self, cut: f64) {
        let cut_rad = -consts::TAU * cut * self.inv_sr;
        self.b1 = cut_rad.exp();
    }
}

impl Process<f64> for LowPass1P {
    fn step(&mut self, input: f64) -> f64 {
        self.z1 += self.b1 * (input - self.z1);
        return self.z1;
    }
}


/// Static gentle high-pass to block DC offsets.
pub struct DcBlock { lp: LowPass1P, }

impl DcBlock {
    /// Initialize filter state variables.
    pub fn new(sr: f64) -> Self {
        let mut ret = Self { lp: LowPass1P::new(sr), };
        ret.lp.set_cutoff(10.0);
        return ret;
    }
}

impl Process<f64> for DcBlock {
    fn step(&mut self, input: f64) -> f64 { 
        let lp = &mut self.lp;
        input - chain!(input => lp)
    }
}