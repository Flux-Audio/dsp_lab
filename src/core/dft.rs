use crate::core::RawRingBufferNoAlloc;
use crate::utils::math::{c_add, c_mul, c_sub, i_exp};

use std::f64::consts;

pub struct SlidingDft {
    size: usize,
    input_buf: RawRingBufferNoAlloc<2048>,
    frame_buf: [(f64, f64); 2048],
}

impl SlidingDft {
    pub fn new() -> Self {
        Self {
            size: 256,
            input_buf: RawRingBufferNoAlloc::new(),
            frame_buf: [(0.0, 0.0); 2048],
        }
    }

    pub fn set_size(&mut self, size: usize) {
        assert!(size <= 2048);
        self.size = size;
    }
    
    // TODO: windowing
    pub fn step(&mut self, input: f64) -> &[(f64, f64)] {
        self.input_buf.push(input);
        let diff = ((input - self.input_buf[self.size - 1]), 0.0);

        for f in 0..self.size {
            self.frame_buf[f] = c_mul(
                    c_add(self.frame_buf[f], diff), 
                    i_exp(consts::TAU * f as f64 / self.size as f64))
        };
        &self.frame_buf
    }
}

pub fn inverse_dft(frame: &[(f64, f64)]) -> f64 {
    let mut accum = (0.0, 0.0);
    
    for item in frame.iter().step_by(2) {
        accum = c_add(accum, *item);
    }
    for item in frame.iter().skip(1).step_by(2) {
        accum = c_sub(accum, *item);
    }
    accum.0 / frame.len() as f64
}