use std::f64::consts;

use crate::traits::Process;
use crate::traits::Source;

// === RAMP CORE ===

/// Phase ramp for driving all oscillators in this module
pub struct RampCore{
    init_phase: f64,
    phase: f64,
    rad_per_sec: f64,
    sr: f64,
}

impl RampCore {
    /// Initialize a new oscillator
    /// - `init_phase`: initial phase of the oscillator, also when reset
    /// - `freq`: frequency in hertz of the oscillator
    /// - `sr`: host sample rate, or sample rate at which `.step()` will be called.
    pub fn new(init_phase: f64, freq: f64, sr: f64) -> Self {
        Self {
            init_phase:  init_phase.rem_euclid(consts::TAU),
            phase:       init_phase.rem_euclid(consts::TAU),
            rad_per_sec: freq*consts::TAU,
            sr:          sr,
        }
    }

    /// Reset phase to `init_phase`
    pub fn reset(&mut self) {
        self.phase = self.init_phase;
    }

    /// Change the frequency of the oscillator, in hertz. This is a method and
    /// not a field, because the frequency is stored internally as radians per second.
    pub fn set_freq(&mut self, freq: f64) {
        self.rad_per_sec = freq*consts::TAU;
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

/// Variable symmetry trianlge oscillator. The `asym` control, makes the rising
/// and falling slopes different, at the extreme (1.0), it turns into a saw wave.
pub struct VarTriOsc {

}

