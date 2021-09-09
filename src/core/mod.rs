//! This module contains basic processes that are needed in most signal chains.

// pub mod integrators;         TODO:
// pub mod derivatives;         TODO:
pub mod lin_filter;            // linear filters
pub mod non_lin_filters;       // non-linear filters, like slew limiters, rolling median
pub mod osc;
// pub mod envelopes;           TODO:
pub mod chaos;                  // random and noise
pub mod delay;               // TODO: delay line with interpolation
// pub mod fft;                 TODO:
pub mod reverb;                 // reverb primitives

use crate::traits::{Process, Source};
// use crate::core::chaos::RandomToggle;        TODO: uncomment when ready
use num::Float;
use std::os::raw::{c_double, c_int};
use std::ops::Index;

/// This process does nothing, except passing the values supplied by step to the
/// output unchanged. It is mostly for debugging and testing purposes. It can
/// also serve as a template for developing other processes.
pub struct EmptyProcess {}

impl Process<f64> for EmptyProcess {
    fn step(&mut self, input: f64) -> f64 { input }
}

/// This source does nothing, except outputting ones. It is mostly for debugging
/// and testing purposes. It can also serve as a template for developing other
/// sources.
pub struct EmptySource {}

impl Source<f64> for EmptySource {
    fn step(&mut self) -> f64 { 1.0 }
}

/// Crude stack-allocated ring buffer implementation, that maximizes efficiency 
/// over anything else. Great for reverbs, especially on embedded systems. This 
/// is the internal datastructure, a public API `SafeRawRingBuffer` is available, 
/// which does softer error handling but may add overhead in cases where extreme 
/// optimization is a requirement.
pub struct RawRingBuffer<const CAP: usize> {
    buffer: [f64; CAP],
    write_ptr: usize,
}

impl<const CAP: usize> RawRingBuffer<CAP> {
    /// Creates new stack allocated ring buffer, panics if CAP is not a power of
    /// two.
    pub fn new() -> Self {
        // checks if CAP is a power of 2
        assert!((CAP != 0) && ((CAP & (CAP - 1)) == 0));

        Self {
            buffer: [0.0; CAP],
            write_ptr: 0
        }
    }

    /// Pushes a new value onto the buffer, overwriting the oldest value if the
    /// buffer is full.
    pub fn push(&mut self, x: f64) {
        self.buffer[self.write_ptr] = x;

        // increment and wrap pointer, with
        // fast bitwise modulo, possible because we enforce CAP to be a power of 2
        self.write_ptr = (self.write_ptr + 1) & (CAP - 1);
    }

    /// Returns value pointed at by `offs`, alternative to using the subscript
    /// operator to avoid referencing.
    /// Indexing starts at the newest addition to the buffer, higher indexes mean
    /// older values.
    pub fn get(&self, offs: usize) -> f64{
        assert!(offs <= CAP);

        // calculate index as an offset from write_ptr, with wrapping done with
        // fast bitwise modulo, possible because we enforce CAP to be a power of 2
        let idx = (self.write_ptr - offs - 1) & (CAP - 1);
        self.buffer[idx]
    }
}

impl<const CAP: usize> Index<usize> for RawRingBuffer<CAP> {
    type Output = f64;

    /// When indexing, higher index means older values on the buffer. Indexing with
    /// 0 returns the newest item.
    fn index(&self, offs: usize) -> &Self::Output {
        assert!(offs < CAP);

        // calculate index as an offset from write_ptr, with wrapping done with
        // fast bitwise modulo, possible because we enforce CAP to be a power of 2
        let idx = (self.write_ptr + CAP - offs - 1) & (CAP - 1);
        &self.buffer[idx]
    }
}

/// Wrapper for `RawRingBuffer` that doesn't panic if preconditions are not met,
/// but has additional overhead because of `Option`. Should still be fast enough
/// for almost any application.
pub struct SafeRawRingBuffer<const CAP: usize> {
    internal_buffer: RawRingBuffer<CAP>,
}

impl<const CAP: usize> SafeRawRingBuffer<CAP> {
    /// Creates a stack-allocated ring buffer. Returns None if size isn't a power
    /// of 2
    pub fn new() -> Option<Self> {
        if (CAP != 0) && ((CAP & (CAP - 1)) == 0) {
            Some(Self{
                internal_buffer: RawRingBuffer::<CAP>::new()
            })
        } else {
            None
        }
    }

    /// Pushes a new value onto the buffer, overwriting the oldest value if the
    /// buffer is full.
    pub fn push(&mut self, x: f64) { self.internal_buffer.push(x); }

    /// Returns value pointed at by `idx`.
    /// Indexing starts at the newest addition to the buffer, higher indexes mean
    /// older values.
    pub fn get(&self, idx: usize) -> Option<f64> {
        if idx < CAP {
            Some(self.internal_buffer[idx])
        } else {
            None
        }
    }
}

/* TODO: uncomment when ready
/// Denormal-blocking dither.
/// 
/// Replaces numbers close to zero and denormals with random small (around 1e-30) 
/// normal floats.
/// 
/// In most cases using this shouldn't be necessary, however there are some
/// good use cases for this:
/// + Before heavy computations to improve performance, especially on embedded
/// systems
/// + At the beginning of a signal chain in a plugin, to defend from a potential
/// stream of denormals coming from a third party plugin suddenly thrashing
/// the performance of the CPU
/// + Before processing involving a lot of divisions, to defend from potential
/// division by 0 errors, or results exploding to infinity.
/// + Sanitizing the output of some third-party function, which might produce
/// a lot of denormals, infinities or NaN values.
/// + After a computation that is likely to produce a lot of denormals, i.e.
/// IIR filters.
/// 
/// # Caveats
/// Normally f64 has a noise floor of less than -900 dB, whereas after dithering
/// with this process, the noise floor is around -600 dB. This is inaudible in
/// most cases, but inside recursive or iterative processes, this extra noise
/// adds up. Usually a single pass of dithering at the start of a processing chain
/// plus a few additional passes to address problematic sections of code will be
/// more than enough dithering, any more than that and it might be counterproductive.
pub struct DenormalDither {
    rng: RandomToggle,
}


impl DenormalDither {
    pub fn new(seed: u8) -> Self {
        let mut rng = RandomToggle::new(seed);
        // very unlikely to toggle, makes the dither frequency be in the subsonic range
        rng.p_up   = 0.00001;
        rng.p_down = 0.00001;
        Self {
            rng: rng,
        }
    }
}

impl Process<f64> for DenormalDither {
    fn step(&mut self, input: f64) -> f64 {

        // Firstly, flush all denormals to zero
        let output = if !input.is_normal() { 0.0 } else { input };

        // Step the rng,scale it and add it to the signal
        return output + (self.rng.step() - 0.5) * 1e-30;
    }
}

/* TODO:
/// Post-processing dither
/// Applies dithering to the output of a processing chain, this is useful especially
/// in combination with denormal filterning in the chain, as the large gap
/// between 0 and the first non-denormal number is dithered. E.g. in combination
/// with PreDither
/// 
/// Algorithm courtesy of Airwindows:
/// <https://github.com/airwindows/airwindows>
pub struct PostDither {
    rng: *mut DitherRng,
}

impl Process<f64> for PostDither {

    /// Applies dithering to input
    fn step(&mut self, mut input: f64) -> f64 {
        // new pseudo random value of dithering
        let dith;
        unsafe {
            (*self.rng).gen_dither();
            dith = (*self.rng).get_dither();
        }
        
        // find raw float exponent through bit manipulation
        extern "C" {
            fn frexp(x: c_double, exp: *mut c_int) -> c_double;
        }
        let mut exp: c_int = 0;
        unsafe { frexp(input, &mut exp) };

        // add dither
        input += (dith - 0x7fffffff_u32) as f64 
               * 1.1e-44_f64 
               * 2_i32.pow(exp as u32 + 62_u32) as f64;
        
        return input;
    }
}

impl PostDither {
    pub fn new(dith_rng: &mut DitherRng) -> Self {
        Self {
            rng: dith_rng
        }
    }
}
*/*/