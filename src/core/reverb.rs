//! this module contains reverb primitives

mod primes;

use std::collections::VecDeque;
use crate::traits::{Process, Source};
use crate::core::chaos::NoiseWhite;
use crate::core::RawRingBuffer;
use crate::core::reverb::primes::PRIMES;

/// Maximum density diffuser, has a delay tap at every prime number. Length
/// determines how many delay taps are used.
/// 
/// # Caveats
/// This is designed to work on a fixed sample rate of 44100 Hz. It will work
/// on other sample rates, but it will sound different. It is suggested that
/// you downsample before using this.
pub struct DenseDiffuser {
    buff: RawRingBuffer<16384>,
    pub size: f64,
}

impl DenseDiffuser {
    pub fn new() -> Self {
        Self {
            buff: RawRingBuffer::<16384>::new(),
            size: 0.5,
        }
    }
}

impl Process<f64> for DenseDiffuser {
    fn step(&mut self, input: f64) -> f64 {
        // rotate internal buffer
        self.buff.push(input);

        // return sum of all prime taps up to num
        let num = (self.size.clamp(0.0, 1.0) * PRIMES.len() as f64) as usize;
        let mut accum = 0.0;
        for i in 0..num {
            accum += self.buff.get(i);
        }
        accum
    }
}

// TODO: this is terribly inefficient, use FFT
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

struct FirBlurLinear {}
struct FirBlurQuad {}

/*
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