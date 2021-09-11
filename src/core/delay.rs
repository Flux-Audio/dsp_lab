//! Various utilities for implementing delays. Contains DelayLine, a robust clean
//! delay which can efficiently also be used for reverb.

use std::collections::VecDeque;
use crate::utils::math;
use crate::traits::Process;
use crate::core::RawRingBuffer;

const MAX_SIZE: usize = 131072;


/// Efficient and hi-fi multitap delay, for delay and reverb effects.
pub struct DelayLine {
    vector: Box<RawRingBuffer<MAX_SIZE>>,
    sr: f64,
    head_offsets: Vec<f64>,
    head_gains: Vec<f64>,
    interp_mode: InterpMethod,
    mix_mode: MixMethod,
}

/// Used to select interpolation method in the delay line.
/// - Truncate: no interpolation, fastest
/// - NearestNeighbor: select the closest index to the specified time, fast
/// - Linear: you need at least this to remove artifacts when modulating time
///   however it will distort a tiny bit, pretty much as fast (if not faster) than
///   nearest neighbor.
/// - Quadratic: less distortion than linear, slowest
pub enum InterpMethod {
    Truncate,
    NearestNeighbor,
    Linear,
    Quadratic,
}

/// Used to select how volume is scaled when mixing
/// - Sum: no volume scaling, volume will be much higher
/// - Sqrt: divide by the square root of the number of taps, perceptual average
/// - Average: divide by the number of taps, numerical average
pub enum MixMethod {
    Sum,
    Sqrt,
    Average,
}

impl DelayLine {
    /// create a new delay line
    /// # Parameters
    /// - size: size in milliseconds
    /// - sr: sample rate in hertz
    /// - interp: interpolation method
    pub fn new(interp: InterpMethod, mix: MixMethod) -> Self {
        Self {
            vector: Box::new(RawRingBuffer::new()),
            sr: 44100.0,
            head_offsets: Vec::new(),
            head_gains: Vec::new(),
            interp_mode: interp,
            mix_mode: mix,
        }
    }

    pub fn set_sr(&mut self, sr: f64) {
        self.sr = sr;
    }

    /// add a read head
    /// # Parameters
    /// - offset: distance in milliseconds (smaller or equal to delay line size)
    /// - gain: gain at which the delay line is played back
    /// # Returns
    /// - index of the head
    pub fn add_head(&mut self, offset: f64, gain: f64) -> usize {
        let offset = (offset/1000.0 * self.sr).clamp(0.0, MAX_SIZE as f64);
        self.head_offsets.push(offset);
        self.head_gains.push(gain);
        self.head_offsets.len() - 1
    }

    /// remove a read head
    /// # Parameters
    /// - index: index of head to remove
    /// # Returns
    /// - boolean representing wether the head existed in the first place.
    pub fn remove_head(&mut self, index: usize) -> bool {
        if index < self.head_offsets.len() {
            self.head_offsets.remove(index);
            self.head_gains.remove(index);
            true
        } else {
            false
        }
    }

    /// changes the offset of one of the heads.
    /// # Parameters
    /// - index: index of the head to be changed
    /// - offset: new offset for the head
    /// # Returns
    /// - boolean representing wether the chosen head exists.
    /// # Side-effects
    /// The vector of heads is shifted, thus all indexes greater than the one
    /// removed are shifted with it.
    pub fn set_offset(&mut self, index: usize, offset: f64) -> bool {
        let offset = (offset/1000.0 * self.sr).clamp(0.0, MAX_SIZE as f64);
        if index < self.head_offsets.len() {
            self.head_offsets[index] = offset/1000.0 * self.sr;
            true
        } else {
            false
        }
    }
}

impl Process<f64> for DelayLine {
    /// write a new value into the delay line and read from all active read heads
    /// # Parameters
    /// - write: input to write
    /// # Returns
    /// - mixed outputs from active heads
    fn step(&mut self, input: f64) -> f64{
        // Step 1: read previous values from read heads
        let accumulator = self.head_offsets.iter()
            .zip(self.head_gains.iter())
            .map(|(a, b)| { match self.interp_mode {
                    InterpMethod::Truncate => 
                        self.vector[*a as usize] * b,
                    InterpMethod::NearestNeighbor => 
                        self.vector[(*a).round() as usize] * b,
                    InterpMethod::Linear => {
                        let i = ((*a).floor() as usize).clamp(0, MAX_SIZE);
                        let x = *a - i as f64;
                        math::x_fade(self.vector[i], x, self.vector[i + 1]) * b},
                    InterpMethod::Quadratic => {
                        let i = ((*a).floor() as usize).clamp(1, MAX_SIZE);
                        let x = *a - i as f64;
                        math::quad_interp(self.vector[i - 1], self.vector[i], self.vector[i + 1], x) * b},
                }})
            .sum::<f64>() / match self.mix_mode {
                MixMethod::Sum => 1.0,
                MixMethod::Sqrt => (self.head_offsets.len() as f64).sqrt(),
                MixMethod::Average => self.head_offsets.len() as f64,
            };

        // Step 2: write new value and shift deque
        self.vector.push(input);

        return accumulator;
    }
}