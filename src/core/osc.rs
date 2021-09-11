use std::f64::consts;

use crate::traits::Process;
use crate::traits::Source;
use crate::traits::ProcessChain;
use crate::utils::math::{asym_tri_shaper, par_shaper};
use crate::core::lin_filter::{BiquadLowPass};

// === RAMP CORE ===

/// Phase ramp for driving all oscillators in this module
pub struct RampCore{
    phase: f64,
    rad_per_sec: f64,
    pub sr: f64,
}

impl RampCore {
    /// Initialize a new oscillator
    /// - `init_phase`: initial phase of the oscillator, also when reset
    /// - `freq`: frequency in hertz of the oscillator
    /// - `sr`: host sample rate, or sample rate at which `.step()` will be called.
    pub fn new() -> Self {
        Self {
            phase:       0.0,
            rad_per_sec: 440.0 * consts::TAU,
            sr:          44100.0,
        }
    }

    /// Change the frequency of the oscillator, in hertz. This is a method and
    /// not a field, because the frequency is stored internally as radians per second.
    pub fn set_freq(&mut self, freq: f64) {
        self.rad_per_sec = freq*consts::TAU;
    }

    /// Change the phase of the oscillator, in radians.
    pub fn set_phase(&mut self, phase: f64) {
        self.phase = phase.rem_euclid(consts::TAU);
    }


}

impl Source<f64> for RampCore {
    fn step(&mut self) -> f64 {
        let ret = self.phase;
        self.phase += self.rad_per_sec/self.sr;
        self.phase = self.phase.rem_euclid(consts::TAU);
        return ret;
    }
}


// === BASIC SHAPES ===

// TODO: extend morphing so that it can both be a saw and a ramp
/// Variable symmetry trianlge oscillator. The `asym` control, makes the rising
/// and falling slopes different, at the extreme (1.0), it turns into a saw wave.
pub struct AsymTriOsc {
    osc: RampCore,
    downsampling_lp_1: BiquadLowPass,
    downsampling_lp_2: BiquadLowPass,
    downsampling_lp_3: BiquadLowPass,
    pub oversampling: u8,
    pub asym: f64,
}

impl AsymTriOsc {
    pub fn new() -> Self {
        Self {
            osc: RampCore::new(),
            downsampling_lp_1: BiquadLowPass::new(),
            downsampling_lp_2: BiquadLowPass::new(),
            downsampling_lp_3: BiquadLowPass::new(),
            oversampling: 1,
            asym: 0.0,
        }
    }

    pub fn set_sr(&mut self, sr: f64) {
        self.osc.sr = sr * self.oversampling as f64;
        self.downsampling_lp_1.set_sr(sr * self.oversampling as f64);
        self.downsampling_lp_2.set_sr(sr * self.oversampling as f64);
        self.downsampling_lp_3.set_sr(sr * self.oversampling as f64);
        self.downsampling_lp_1.cutoff = sr * 0.48;
        self.downsampling_lp_2.cutoff = sr * 0.48;
        self.downsampling_lp_3.cutoff = sr * 0.48;
    }

    pub fn set_freq(&mut self, freq: f64) {
        self.osc.set_freq(freq);
    }

    pub fn set_phase(&mut self, phase: f64) {
        self.osc.set_phase(phase);
    }
}

impl Source<f64> for AsymTriOsc {
    fn step(&mut self) -> f64 {
        let mut res = 0.0;
        for _ in 0..self.oversampling {
            res = ProcessChain::new(asym_tri_shaper(self.osc.step(), self.asym))
                .pipe(&mut self.downsampling_lp_1)
                .pipe(&mut self.downsampling_lp_2)
                .pipe(&mut self.downsampling_lp_3)
                .consume();
        }
        res
    }
}


/// Parabolic sine approximation oscillator. Much faster than true sine, but has
/// a bit of saturation. Can actually sound very nice as an analog sine.
pub struct ParOsc {
    osc: RampCore,
    downsampling_lp_1: BiquadLowPass,
    downsampling_lp_2: BiquadLowPass,
    downsampling_lp_3: BiquadLowPass,
    pub oversampling: u8,
    pub asym: f64,
}

impl ParOsc {
    pub fn new() -> Self {
        Self {
            osc: RampCore::new(),
            downsampling_lp_1: BiquadLowPass::new(),
            downsampling_lp_2: BiquadLowPass::new(),
            downsampling_lp_3: BiquadLowPass::new(),
            oversampling: 1,
            asym: 0.0,
        }
    }

    pub fn set_sr(&mut self, sr: f64) {
        self.osc.sr = sr * self.oversampling as f64;
        self.downsampling_lp_1.set_sr(sr * self.oversampling as f64);
        self.downsampling_lp_2.set_sr(sr * self.oversampling as f64);
        self.downsampling_lp_3.set_sr(sr * self.oversampling as f64);
        self.downsampling_lp_1.cutoff = sr * 0.48;
        self.downsampling_lp_2.cutoff = sr * 0.48;
        self.downsampling_lp_3.cutoff = sr * 0.48;
    }

    pub fn set_freq(&mut self, freq: f64) {
        self.osc.set_freq(freq);
    }

    pub fn set_phase(&mut self, phase: f64) {
        self.osc.set_phase(phase);
    }
}

impl Source<f64> for ParOsc {
    fn step(&mut self) -> f64 {
        let mut res = 0.0;
        for _ in 0..self.oversampling {
            res = ProcessChain::new(par_shaper(self.osc.step()))
                .pipe(&mut self.downsampling_lp_1)
                .pipe(&mut self.downsampling_lp_2)
                .pipe(&mut self.downsampling_lp_3)
                .consume();
        }
        res
    }
}


// TODO: pulse oscillator