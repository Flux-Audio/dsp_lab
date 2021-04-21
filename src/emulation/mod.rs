//! This module contains emulations of various physical components.

// pub mod capacitors   // TODO: real capacitor modelling, various levels of detail, with thermal noise
// pub mod resistors    // TODO: real resistor modelling, various levels of detail, with thermal noise
// pub mod inductors    // TODO: real inductor modelling, various levels of detial, with thermal noise
// pub mod memristors   // TODO: real memristor modelling, various levels of detail, with thermal noise
// pub mod diodes       // TODO: real diode modelling
// pub mod vactrols     // TODO: real vactrol modelling
// pub mod transformers // TODO: real voltage transformer modelling
// pub mod diff_amps    // TODO: real differential amplifier modelling, op-amps, comparators, schmitt triggers.
// pub mod transistor   // TODO: real transistor modelling
// pub mod tubes        // TODO: real vacuum tube modelling
// pub mod cables       // TODO: real cable modelling, with internal resistance, inductance, capacitance and interference noise
// pub mod dischargers  // TODO: real discharge tube modelling, like stroboscopic tubes, fluorescent tubes
// pub mod crystal      // TODO: real crystal oscillator modelling
pub mod tape;       // real magnetic tape modelling, with read/write heads

use crate::traits::Process;
use crate::utils::math::{fast_powf, fast_tanh};

/// Models magnetic hysteresis found in transformer cores and magnetic tape.
/// Sub-modules depend on this
pub struct Hysteresis {
    x_p: f32,   // previous value of input
    y_p: f32,   // previous value of output
    pub sq:    f32,
    pub coerc: f32,
    pub fast:  bool,
}

impl Process<f32> for Hysteresis{
    fn step(&mut self, input: f32) -> f32 { 
        let dx: f32 = input - self.x_p;
        self.x_p = input;

        // calmp hysteresis parameters to avoid floating point errors and
        // NaN / infinity values
        self.sq = self.sq.clamp(0.0 , 0.95);

        // crossfade to stateless distortion, for small values of coercitivity
        let k   =  self.coerc.clamp(0.1, 1.0);
        let mix = (self.coerc.clamp(0.0, 0.2) * 5.0).sqrt().sqrt().sqrt();

        // hysteresis loop equation (with fast toggle)
        let y_an: f32 = if !self.fast {
            // hq:      ~75 ns/iter
            input.abs()
                 .powf(1.0/(1.0 - self.sq))
                 .tanh()
                 .powf(1.0 - self.sq)
                 * input.signum()
                 
        } else {
            // fast:    ~55 ns/iter
            fast_powf(
                fast_tanh(
                    fast_powf(input.abs(), 1.0/(1.0 - self.sq))), 
                1.0 - self.sq)
            * input.signum()
        };

        let y: f32 = self.y_p + (y_an - self.y_p) * dx.abs() / k;
        
        // prevent runaway accumulation by leaking state and clamping
        self.y_p = (y * mix + y_an * (1.0 - mix)).clamp(-1.25, 1.25);

        // round denormals to zero in feedback loop
        if self.y_p.is_subnormal() { self.y_p = 0.0 };

        return self.y_p;
    }
}

impl Hysteresis{
    pub fn new() -> Self {
        Self{
            x_p: 0.0,
            y_p: 0.0,
            sq:   0.5,
            coerc: 0.5,
            fast:  true,
        }
    }
}