//! this module contains reverb primitives

use std::collections::VecDeque;
use crate::traits::{Process, Source};
use crate::core::chaos::NoiseWhite;

pub struct FirBlurFlat {
    delay_line: VecDeque<f64>,
    max_size: usize,
    pub len: f64, 
    pub sr: f64,
    noise_source: NoiseWhite,
}

impl FirBlurFlat {
    pub fn new(size: f64, sr: f64, seed: u8) -> Self {
        let num = (size/1000.0 * sr) as usize + 1;
        Self {
            delay_line: VecDeque::from(vec![0.0; num]),
            max_size: num,
            len: 0.0,
            sr: sr,
            noise_source: NoiseWhite::new(seed),
        }
    }
}

impl Process<f64> for FirBlurFlat {
    fn step(&mut self, input: f64) -> f64 {
        self.delay_line.push_front(input);
        self.delay_line.pop_back();

        let len = (self.len/1000.0 * self.sr) as usize;
        let len = len.clamp(0, self.max_size);

        let mut accum = 0.0;
        for i in 0..len {
            accum += self.delay_line[i] * self.noise_source.step();
        }
        accum
    }
}




struct FirBlurLinear {

}


struct FirBlurQuad {

}