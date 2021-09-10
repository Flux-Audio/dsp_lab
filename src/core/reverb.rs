//! this module contains reverb primitives

mod tuning;

use std::collections::VecDeque;
use crate::traits::{Process, Source};
use crate::core::chaos::NoiseWhite;
use crate::core::RawRingBuffer;
use crate::core::reverb::tuning::{PRIMES, HO_PRIMES, DENSE_COEFFS, SPARSE_COEFFS};

/// Maximum density diffuser, has a delay tap at every prime number. Length
/// determines how many delay taps are used.
/// 
/// # Caveats
/// This is designed to work on a fixed sample rate of 44100 Hz. It will work
/// on other sample rates, but it will sound different. It is suggested that
/// you downsample before using this.
/// 
/// It is also very CPU intensive on `opt-level=0`, but in `opt-level=3` it is
/// instead extremely efficient.
pub struct DenseFirDiffuser {
    buff: RawRingBuffer<8192>,
    pub size: f64,
}

impl DenseFirDiffuser {
    pub fn new() -> Self {
        Self {
            buff: RawRingBuffer::<8192>::new(),
            size: 0.5,
        }
    }
}

impl Process<f64> for DenseFirDiffuser {
    fn step(&mut self, input: f64) -> f64 {
        // rotate internal buffer
        self.buff.push(input);

        // return sum of all prime taps up to num
        let range = (self.size.clamp(0.0, 1.0) * 1027.0) as usize;
        let mut accum = 0.0;
        for i in 0..range {
            let idx = PRIMES[i];
            let coeff = DENSE_COEFFS[i];
            accum += self.buff[idx] * coeff;
        }
        accum
    }
}

pub struct SparseFirDiffuser {
    buff: RawRingBuffer<16384>,
    pub size: f64,
}

impl SparseFirDiffuser {
    pub fn new() -> Self {
        Self {
            buff: RawRingBuffer::<16384>::new(),
            size: 0.5,
        }
    }
}

impl Process<f64> for SparseFirDiffuser {
    fn step(&mut self, input: f64) -> f64 {
        // rotate internal buffer
        self.buff.push(input);

        // return sum of all prime taps up to num
        let range = (self.size.clamp(0.0, 1.0) * 289.0) as usize;
        let mut accum = 0.0;
        for i in 0..range {
            let idx = HO_PRIMES[i];
            let coeff = SPARSE_COEFFS[i];
            accum += self.buff[idx] * coeff;
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