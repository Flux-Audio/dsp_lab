use num::complex::Complex;
use crate::core::RawRingBuffer;

use std::f64::consts;

pub struct SlidingDft {
    size: usize,
    input_buf: RawRingBuffer<2048>,
    frame_buf: Box<[Complex<f64>]>,
}

impl SlidingDft {
    pub fn new() -> Self {
        Self {
            size: 256,
            input_buf: RawRingBuffer::new(),
            frame_buf: vec![Complex::new(0.0, 0.0); 2048].into_boxed_slice(),
        }
    }

    pub fn set_size(&mut self, size: usize) {
        assert!(size <= 2048);
        self.size = size;
    }
    
    // TODO: windowing
    pub fn step(&mut self, input: f64) -> Box<[Complex<f64>]> {
        self.input_buf.push(input);
        let diff = Complex::new(input - self.input_buf[self.size - 1], 0.0);

        for f in 0..self.size {
            self.frame_buf[f] = (self.frame_buf[f] + diff) * Complex::new(0.0, consts::TAU * f as f64 / self.size as f64).exp()
        };
        self.frame_buf.clone()
    }
}

pub fn inverse_dft(frame: Box<[Complex<f64>]>, size: usize) -> f64 {
    let mut accum = Complex::new(0.0, 0.0);
    for i in 0..size {
        accum = if i % 2 == 0 {
            accum + frame[i]
        } else {
            accum - frame[i]
        }
    };
    accum.re / size as f64
}