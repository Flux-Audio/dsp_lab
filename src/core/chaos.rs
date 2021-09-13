use crate::traits::{Source};

use std::f64::consts;

/// Generate random u64, this is used to implement all other chaotic
/// processes (except for physical modelling chaos)
/// Note that this is not a process, as u64 does not implement the Float
/// trait.
pub struct RandomCore {
    state: u64,
}

impl RandomCore {
    pub fn new() -> Self {
        Self {
            state: 17,
        }
    }

    /// Use this to initialize multiple instances, it is very lightweight
    /// and low quality, but it's good enough for audio.
    pub fn reseed(&mut self, seed: u8) {
        let start_state = seed & 0x0f;
        let skips = ((seed & 0xf0) >> 4) + 1;

        // generate a new seed from combining previous state and new state
        self.state = start_state as u64 + 1;
        
        // mangle the new state a bit by shifting
        for _ in 0..skips{
            self.next();
        }
    }

    /// Generate a random u64. Note that this algorithm provides bad quality in
    /// the lowest bit, so either use the upper 32, or cast to float.
    pub fn next(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        return self.state;
    }
}


/// Random weighted coin toss, akin to a Bernoulli gate
pub struct RandomCoin {
    pub p: f64,
    rng: NoiseWhite,
}

impl RandomCoin {
    pub fn new(seed: u8) -> Self {
        Self {
            p: 0.5,
            rng: NoiseWhite::new(seed),
        }
    }
}

impl Source<f64> for RandomCoin {
    fn step(&mut self) -> f64 { if self.rng.step() < self.p { 1.0 } else { 0.0 } }
}

/// Random weighted toggle, with asymmetrical probabilities
pub struct RandomToggle {
    pub p_up: f64,
    pub p_down: f64,
    rng: NoiseWhite,
    toggle: bool,
}

impl RandomToggle {
    pub fn new(seed: u8) -> Self {
        Self {
            p_up: 0.25,
            p_down: 0.25,
            rng: NoiseWhite::new(seed),
            toggle: false,
        }
    }
}

impl Source<f64> for RandomToggle {
    fn step(&mut self) -> f64 {
        let nse = self.rng.step();
        if self.toggle {
            if nse < self.p_down {
                self.toggle = false;
            }
            return 1.0;
        } else {
            if nse < self.p_up {
                self.toggle = true;
            }
            return 0.0;
        };
    }
}

/// Random impulses, with variable rate and regularity
/// TODO:
pub struct RandomGeiger {

}


/// Generate white noise, i.e. a float in the range [0, 1) with uniform distribution.
/// 
/// White noise has a uniform power spectrum.
pub struct NoiseWhite {
    rng: RandomCore,
}

impl NoiseWhite {
    pub fn new(seed: u8) -> Self {
        let mut rng = RandomCore::new();
        rng.reseed(seed);
        Self { rng, }
    }
}

impl Source<f64> for NoiseWhite {
    fn step(&mut self) -> f64 {

        // Cast upper 52 bits
        let mut bits = self.rng.next() >> 11;
        bits &= 0b0_00000000000_1111111111111111111111111111111111111111111111111111;
        bits |= 0b0_01111111110_0000000000000000000000000000000000000000000000000000;
        f64::from_bits(bits) * 2.0 - 1.0
    }
}


/// Sample and hold random
pub struct SnhRandom {
    rng: NoiseWhite,
    phase: f64,
    rad_per_sec: f64,
    sr: f64,
    latch: f64,
}

impl SnhRandom {
    pub fn new(sr: f64, seed: u8) -> Self {
        Self {
            rng: NoiseWhite::new(seed),
            phase: 0.0,
            rad_per_sec: 1.0,
            sr,
            latch: 0.0,
        }
    }

    /// Change the frequency of the generator, in hertz. This is a method and
    /// not a field, because the frequency is stored internally as radians per second.
    pub fn set_freq(&mut self, freq: f64) {
        self.rad_per_sec = freq * consts::TAU;
    }
}

impl Source<f64> for SnhRandom {
    fn step(&mut self) -> f64 {
        self.phase += self.rad_per_sec / self.sr;
        if self.phase >= consts::TAU {
            self.phase -= consts::TAU;
            self.latch = self.rng.step();
        }
        return self.latch;
    }
}


/// Makes bound red/brown noise if the input is white noise
/// 
/// red/Brown noise has the power spectrum 1 / f^2
/// TODO:
pub struct RedFilter {

}


/// Makes bound violet noise if the input is white noise
/// 
/// Violet noise has the power spectrum f^2
/// TODO:
pub struct VioletFilter {

}