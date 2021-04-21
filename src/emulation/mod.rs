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

/// Models magnetic hysteresis found in transformer cores and magnetic tape.
/// Sub-modules depend on this
pub struct Hysteresis {
    x_p: f64,   // previous value of input
    y_p: f64,   // previous value of output
    pub sq:    f64,
    pub coerc: f64,
}

impl Process<f64> for Hysteresis{
    fn step(&mut self, input: f64) -> f64 { 
        let dx: f64 = input - self.x_p;
        self.x_p = input;

        // calmp hysteresis parameters to avoid floating point errors and
        // NaN / infinity values
        self.sq    = self.sq   .clamp(0.0 , 0.99);
        // self.coerc = self.coerc.clamp(0.07, 1.0);
        let k = self.coerc.clamp(0.125, 1.0);
        let mix = self.coerc.clamp(0.0, 0.25) * 4.0;

        // hysteresis loop equation
        let y_an: f64 = input.abs()
                             .powf(1.0/(1.0 - self.sq))
                             .tanh()
                             .powf(1.0 - self.sq)
                             * input.signum();
        let y: f64 = self.y_p + (y_an - self.y_p) * dx.abs() / k;
        
        // prevent runaway accumulation by leaking state and clamping
        self.y_p = (y * mix + y_an * (1.0 - mix)).clamp(-1.25, 1.25);

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
        }
    }
}