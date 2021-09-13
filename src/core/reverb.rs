//! This module contains reverb primitives.
//! 
//! To be specific, it contains:
//! - FIR diffusers (non-recursive multitap delay banks, fixed tap-spacing, optimized
//!   to minimize resonances, extremely efficient on `opt-level = 3`)
//! - Dynamic FIR diffusers (FIR diffusers with variable spacing of taps, enable
//!   modulation of delay time, slightly less efficient)
//! - Cascaded AP diffusers (fairly low-quality early digital reverb, used in
//!   shroeder reverberators)
//! - Reflections: dynamic sparse delay bank, for early reflections, has modes
//!   vor various types of reverb (room, chamber, hall, plate, spring ...)
//! - Dispersion reflections: sparse delay bank with high-order all-pass dispersion,
//!   simulates dispersion of certain media (like springs and plates)
//! - Spatialized reflections: sparse delay bank with binaural spatialization for
//!   each tap.
//! - Nested AP diffusers (for a more vintage, metallic and shimmery reverb algorithms,
//!   very CPU efficient)
//! - Parallel comb reverberators (also fairly low-quality early digital reverb, used
//!   in shroeder reverberators)
//! - Parallel LBCF reverberators (variation on parallel combs, used in the Freeverb
//!   algorithm)
//! - Spectral diffusers (FFT-based convolution diffusers, are very modern and
//!   flexible, but also quite CPU intensive)
//! - FDN reverberators: use linear algebra magic to implement very dense exponentially
//!   decaying diffusion with low CPU usage. Used extensively by Valhalla DSP reverbs.
//! - Multiband FDN reverberators: FDN reverberators with different decay times
//!   for each frequency band, achieved with different amounts of feedback.
//! **NOTE 1:** for convolution reverbs, use the convolution primitive in the `lin_filter`
//! module.
//! 
//! **NOTE 2:** simple delays are in the `delay` module, or alternatively can be
//! manually implemented using raw ring buffers from the `core` module.
//! 
//! **NOTE 3:** a distinction is made here between "reverberators" and "diffusers",
//! although there isn't an exact "official" distinction. Diffusers have unity
//! gain characteristics, meaning they can be fed-back at unity gain and not
//! explode, reverberators on the other hand do not have this property, as they
//! themselves add to the overall gain as their length increases, with no need
//! for additional feedback.
//! 
//! **NOTE 4:** some simple reverb algorithms are implemented in the `effects` module.

mod tuning;

use crate::traits::Process;
use crate::core::RawRingBuffer;
use crate::core::reverb::tuning::{PRIMES, HO_PRIMES, SPARSE_A, SPARSE_B, SPARSE_C, 
    SPARSE_D, SPARSE_E, SPARSE_F, SPARSE_G, SPARSE_H};
use crate::shared_enums::{Polarization, ScaleMethod};

pub enum TuningVectors {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

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
    pub scale_mode: ScaleMethod,
}

impl DenseFirDiffuser {
    pub fn new() -> Self {
        Self {
            buff: RawRingBuffer::<8192>::new(),
            size: 0.5,
            scale_mode: ScaleMethod::Unity,
        }
    }
}

impl Process<f64> for DenseFirDiffuser {
    fn step(&mut self, input: f64) -> f64 {
        // rotate internal buffer
        self.buff.push(input);

        // return sum of all prime taps up to num
        let mut range = (self.size.clamp(0.0, 1.0) * 1027.0) as usize;
        if range == 0 { range = 1 };    // ensure minimum size
        let mut accum = 0.0;

        for idx in PRIMES.iter().take(range) {
            accum += self.buff[*idx];
        }

        match self.scale_mode {
            ScaleMethod::Off => accum,
            ScaleMethod::Perceptual => accum / (range as f64).sqrt(),
            ScaleMethod::Unity => accum / range as f64
        }
    }
}


pub struct SparseFirDiffuser {
    buff: RawRingBuffer<16384>,
    pub size: f64,
    pub scale_mode: ScaleMethod,
}

impl SparseFirDiffuser {
    pub fn new() -> Self {
        Self {
            buff: RawRingBuffer::<16384>::new(),
            size: 0.5,
            scale_mode: ScaleMethod::Unity,
        }
    }
}

impl Process<f64> for SparseFirDiffuser {
    fn step(&mut self, input: f64) -> f64 {
        // rotate internal buffer
        self.buff.push(input);

        // return sum of all prime taps up to num
        let mut range = (self.size.clamp(0.0, 1.0) * 289.0) as usize;
        if range == 0 { range = 1 };    // ensure minimum size
        let mut accum = 0.0;
        for idx in HO_PRIMES.iter().take(range) {
            accum += self.buff[*idx];
        }
        
        match self.scale_mode {
            ScaleMethod::Off => accum,
            ScaleMethod::Perceptual => accum / (range as f64).sqrt(),
            ScaleMethod::Unity => accum / range as f64
        }
    }
}


pub struct PolarizedFirDiffuser {
    buff: RawRingBuffer<65536>,
    pub size: f64,
    pub positive_tuning: TuningVectors,
    pub negative_tuning: TuningVectors,
    pub polarization: Polarization,
    pub scale_mode: ScaleMethod,
}

impl PolarizedFirDiffuser {
    pub fn new() -> Self {
        Self {
            buff: RawRingBuffer::new(),
            size: 0.5,
            positive_tuning: TuningVectors::A,
            negative_tuning: TuningVectors::B,
            polarization: Polarization::Zero,
            scale_mode: ScaleMethod::Perceptual,
        }
    }
}

impl Process<f64> for PolarizedFirDiffuser {
    fn step(&mut self, input: f64) -> f64 {
        // rotate internal buffer
        self.buff.push(input);

        // return sum of all prime taps up to num, once for positive and once
        // for negative taps.
        let mut range = (self.size.clamp(0.0, 1.0) * 192.0) as usize;
        if range == 0 { range = 1 };    // ensure minimum size
        let positive_taps = match self.positive_tuning {
            TuningVectors::A => SPARSE_A,
            TuningVectors::B => SPARSE_B,
            TuningVectors::C => SPARSE_C,
            TuningVectors::D => SPARSE_D,
            TuningVectors::E => SPARSE_E,
            TuningVectors::F => SPARSE_F,
            TuningVectors::G => SPARSE_G,
            TuningVectors::H => SPARSE_H,
        };
        let negative_taps = match self.negative_tuning {
            TuningVectors::A => SPARSE_A,
            TuningVectors::B => SPARSE_B,
            TuningVectors::C => SPARSE_C,
            TuningVectors::D => SPARSE_D,
            TuningVectors::E => SPARSE_E,
            TuningVectors::F => SPARSE_F,
            TuningVectors::G => SPARSE_G,
            TuningVectors::H => SPARSE_H,
        };
        let mut accum = 0.0;
        for i in 0..range {
            let positive_idx = positive_taps[i];
            let negative_idx = negative_taps[i];
            //let coeff = SPARSE_COEFFS[i];
            accum += self.buff[positive_idx];
            accum -= self.buff[negative_idx];
        }

        accum = match self.polarization {
            Polarization::Unity => accum + input,
            Polarization::Zero => accum,
            Polarization::NegativeUnity => accum - input,
        };
        match self.scale_mode {
            ScaleMethod::Off => accum,
            ScaleMethod::Perceptual => accum / (range as f64).sqrt(),
            ScaleMethod::Unity => accum / (range as f64),
        }
    }
}


pub struct StereoFirDiffuser {
    left_diff:      PolarizedFirDiffuser,
    right_diff:     PolarizedFirDiffuser,
    cross_to_right: PolarizedFirDiffuser,
    cross_to_left:  PolarizedFirDiffuser,
    pub size: f64,
    pub crossover: f64,

    // auxiliary outputs
    pub right_aux:  f64,
    pub left_aux:   f64,
    pub l_to_r_aux: f64,
    pub r_to_l_aux: f64,
}

impl StereoFirDiffuser {
    pub fn new() -> Self {
        let mut ret = Self {
            left_diff:      PolarizedFirDiffuser::new(),
            right_diff:     PolarizedFirDiffuser::new(),
            cross_to_right: PolarizedFirDiffuser::new(),
            cross_to_left:  PolarizedFirDiffuser::new(),
            size: 0.5,
            crossover: 0.2,

            // auxiliary outputs
            right_aux:  0.0,
            left_aux:   0.0,
            l_to_r_aux: 0.0,
            r_to_l_aux: 0.0,
        };
        ret.left_diff.positive_tuning      = TuningVectors::A;
        ret.left_diff.negative_tuning      = TuningVectors::B;
        ret.right_diff.positive_tuning     = TuningVectors::C;
        ret.right_diff.negative_tuning     = TuningVectors::D;
        ret.cross_to_right.positive_tuning = TuningVectors::E;
        ret.cross_to_right.negative_tuning = TuningVectors::F;
        ret.cross_to_left.positive_tuning  = TuningVectors::G;
        ret.cross_to_left.negative_tuning  = TuningVectors::H;
        //ret.cross_to_right.polarization = Polarization::Zero;
        //ret.cross_to_left.polarization  = Polarization::Zero;
        ret
    }

    // TODO: implement a "stereo pair" type that implements the "Float" trait
    // so that the Process trait can be implemented.
    pub fn step(&mut self, input: (f64, f64)) -> (f64, f64) {
        let (left, right) = input;
        let size = self.size;
        self.left_diff.size      = size;
        self.right_diff.size     = size;
        self.cross_to_right.size = size;
        self.cross_to_left.size  = size;

        // step diffusers, store in auxiliary outputs
        self.left_aux   = self.left_diff.step(left);
        self.right_aux  = self.right_diff.step(right);
        self.l_to_r_aux = self.cross_to_right.step(left);
        self.r_to_l_aux = self.cross_to_left.step(right);

        // mixing matrix
        let ret_l = self.left_aux * (1.0 - self.crossover)  + self.crossover * self.r_to_l_aux;
        let ret_r = self.right_aux * (1.0 - self.crossover) + self.crossover * self.l_to_r_aux;

        (ret_l, ret_r)
    }
}





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