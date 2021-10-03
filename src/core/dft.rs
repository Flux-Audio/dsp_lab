use crate::core::{RawRingBufferNoAlloc, RawRingBuffer};
use crate::utils::math::{c_add, c_mul, c_sub, i_exp};
use crate::shared_enums::{WindowMode, OverlapPolicy};
use num::complex::Complex;
use rustfft::Fft;

use std::f64::consts;
use std::sync::Arc;

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
        let diff = ((input - self.input_buf[self.size - 1]), 0.0);
        self.input_buf.push(input);

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

/// Performs forward and backward FFT with matching size and window.
/// 
/// The FFT object should be allocated only once, as it can reuse memory across
/// individual FFT procedures.
/// 
/// Is used in the public API for OlaFft to build chains. A chain node takes a
/// reference to the buffer of the previous node, or the fft_buf of the FftCore
/// if it's the first node, and uses it to update its internal buffer(s) according
/// to some processing algorithm.
/// 
/// A node may also just modify the reference it was given in the case that it
/// doesn't need to keep any local state.
struct FftCore <'a>{

    // the size of the frame, preferribly a power of 2 or sum of
    // few powers of two (i.e. 24, 48, 192, ...)
    size: usize,

    // specifies how many samples are between the start of
    // each overlapping frame. This is calculated from the
    // overlap ratio.
    frame_gap: usize,  
    
    // keeps track of the gap between 0 and the first
    // buffer's index, essentially counts up to frame_gap
    // then is reset, and is used to determine when
    // to start recording into a new buffer.
    first_gap_counter: usize,
    
    // the input buffer, is a matrix, but stored
    // sequentially. Each buffer is 4096 samples long
    // and there are 6 of them.
    // NOTE: for optimization, try size-hinting allocations by zerofilling with an iterator rather than pushing.
    in_buf: Vec<f64>,
    
    // the "stack pointers" for the end of each buffer.
    // they are initialized according to the frame gap,
    // then incremented once per sample until they reach
    // the size of the window, at which point they are
    // reset.
    buf_top: [usize; 6],
    
    // index of which buffer will be used next in the fft
    leading_buf: usize,
    
    // index of the highest buffer in use (depending on overlap)
    highest_buf: usize,
    
    // selects which windowing function to use
    window_mode: WindowMode,
    
    // selects which overlap policy to use
    overlap_policy: OverlapPolicy,

    // stores window coefficients, is half the size of the
    // input buffer because we are exploiting the symmetry
    // of windows to save on computations
    // should have 2048 elements
    // NOTE: for optimization, try size-hinting allocations by zerofilling with an iterator rather than pushing.
    win_buf: Vec<f64>,

    // should have 4096 elements
    // NOTE: RawRingBufferNoAlloc might be faster
    out_buf: Vec<f64>,
    
    // Wether the output buffer has just been written to. As soon as the output
    // buffer is inspected, this should be set to false.
    is_updated: bool,

    // NOTE: since these three buffers are very hot, they are stack allocated.

    // when the FFT is ready to be made, the
    // leading buffer is copied here while
    // simultaneously computing the windowing
    // should have 4096 elements
    in_buf_windowed: Vec<Complex<f64>>,

    // the result of the FFT ends up here, this is also the vector used to store
    // intermediate computations in the FFT effect chain, and it's the input
    // buffer for the IFFT
    // should have 4096 elements
    fft_buf: Vec<Complex<f64>>,

    // stores the output of the IFFT
    // should have 4096 elements
    out_buf_windowed: Vec<Complex<f64>>,

    // rustfft uses this to store temporary data. It is garbage.
    // should have 4096 elements
    garbage_buf: Vec<Complex<f64>>,

    // FFT engines for forward and inverse FFT, store reference to the return
    // value of the FftPlanner's plan_fft_forward() and plan_fft_backward()
    // methods. Note that the same instance of the FftPlanner should be used
    // to instantiate both engines, as this allows for memory reuse.
    fft_engine_fwd: Arc<dyn Fft<Complex<f64>>>,
    fft_engine_bwd: Arc<dyn Fft<Complex<f64>>>,
}

impl<'a> FftCore<'a> {

    // creates a new instance of FftPlanner, and uses it to initialize both
    // forward and backward engines.
    // fn new() -> Self{}

    // sets the size of the windows, which changes a lot of how the indexing is
    // performed internally
    fn set_size(&mut self, size: usize) {
        self.size = size;
        self.apply_config();
    }

    // sets the type of window, which not only selects the window mode, but also
    // affects indexing, as each window has a different overlap setting.
    fn set_win_type(&mut self, win: WindowMode) {
        self.window_mode = win;
        self.apply_config();
    }

    // set the overlap policy, which affects the amount of overlap and thus the
    // indexing.
    fn set_overlap_policy(&mut self, policy: OverlapPolicy) {
        self.overlap_policy = policy;
        self.apply_config();
    }

    // updates the internal state after a configuration change
    fn apply_config(&mut self) {
        let col: isize = match self.overlap_policy {
            OverlapPolicy::Off           => -1,
            OverlapPolicy::Eco           =>  0,
            OverlapPolicy::Default       =>  1,
            OverlapPolicy::FlatAmplitude =>  2,
            OverlapPolicy::FlatPower     =>  3,
        };
        if col == -1 {
            self.frame_gap = self.size;
            return;
        }
        let row = match self.window_mode {
            WindowMode::Box            => 0,
            WindowMode::Triangular     => 1,
            WindowMode::Welch          => 2,
            WindowMode::Hann           => 3,
            WindowMode::BlackmanHarris => 4,
            WindowMode::Nuttal         => 5,
            WindowMode::Kaiser         => 6,
            WindowMode::FlatTop        => 7
        };
        let overlap_size = (OVERLAPS[col as usize + row * 4] * self.size as f64) as usize;
        self.frame_gap = self.size - overlap_size;

        // TODO: write window into window buffer
    }

    

    // returns true if the fft_buf has not been read since it was last computed.
    fn is_updated(&self) -> bool {
        if self.is_updated {
            self.is_updated = false;
            return true;
        } else {
            return false;
        }
    }

    // streams samples into the input buffers, striping it according to the
    // overlap settings
    // fn in_stream() {}

    // rebuilds an output stream with the overlap and add method.
    // fn out_stream() -> f64 {}

    // internal function for forward and backward fft, these are the exit and
    // entry points of the FFT chain.
    // fn fft_forward() -> &'a [Complex<f64>] {}
    // fn fft_backward() -> {}
}

// FFT windowing overlap ratios, based on policy and window type:
const OVERLAPS: [f64; 32] = [
//  eco     ROV     flat A  flat P
    0.0,    0.0,    0.0,    0.0,    // boxcar
    0.5,    0.5,    0.5,    0.8325, // triangle
    0.293,  0.293,  0.71,   0.46,   // welch
    0.5,    0.5,    0.5,    0.6667, // hann
    0.5,    0.661,  0.74,   0.82,   // blackman - harris
    0.5,    0.612,  0.65,   0.78,   // nuttal 3a
    0.5,    0.619,  0.69,   0.79,   // kaiser 3
    0.5,    0.6667, 0.6667, 0.8     // SFT3F (flat-top)
];