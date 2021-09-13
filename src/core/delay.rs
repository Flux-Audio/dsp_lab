//! Various utilities for implementing delays. Contains DelayLine, a robust clean
//! delay which can efficiently also be used for reverb.

use crate::utils::math;
use crate::traits::Process;
use crate::core::RawRingBuffer;
use crate::shared_enums::{InterpMethod, ScaleMethod};

const MAX_SIZE: usize = 131072;


/// Efficient and hi-fi multitap delay, for delay and reverb effects.
pub struct DelayLine {
    vector: RawRingBuffer<MAX_SIZE>,
    sr: f64,
    head_offsets: Vec<f64>,
    head_gains: Vec<f64>,
    pub interp_mode: InterpMethod,
    pub mix_mode: ScaleMethod,
}

impl DelayLine {
    /// create a new delay line
    /// # Parameters
    /// - size: size in milliseconds
    /// - sr: sample rate in hertz
    /// - interp: interpolation method
    pub fn new() -> Self {
        Self {
            vector: RawRingBuffer::new(),
            sr: 44100.0,
            head_offsets: Vec::new(),
            head_gains: Vec::new(),
            interp_mode: InterpMethod::Linear,
            mix_mode: ScaleMethod::Perceptual,
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
        //let offset = (offset/1000.0 * self.sr).clamp(0.0, MAX_SIZE as f64);
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
        //let offset = (offset/1000.0 * self.sr).clamp(0.0, MAX_SIZE as f64);
        if index < self.head_offsets.len() {
            self.head_offsets[index] = offset;
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
            .map(|(a, b)| { 
                let offset = (a / 1000.0 * self.sr).clamp(0.0, MAX_SIZE as f64);
                match self.interp_mode {
                    InterpMethod::Truncate => 
                        self.vector[offset as usize] * b,
                    InterpMethod::NearestNeighbor => 
                        self.vector[offset.round() as usize] * b,
                    InterpMethod::Linear => {
                        let i = (offset.floor() as usize).clamp(0, MAX_SIZE);
                        let x = offset - i as f64;
                        math::x_fade(self.vector[i], x, self.vector[i + 1]) * b},
                    InterpMethod::Quadratic => {
                        let i = (offset.floor() as usize).clamp(1, MAX_SIZE);
                        let x = offset - i as f64;
                        math::quad_interp(self.vector[i - 1], self.vector[i], self.vector[i + 1], x) * b},
                }})
            .sum::<f64>() / match self.mix_mode {
                ScaleMethod::Off => 1.0,
                ScaleMethod::Perceptual => (self.head_offsets.len() as f64).sqrt(),
                ScaleMethod::Unity => self.head_offsets.len() as f64,
            };

        // Step 2: write new value and shift deque
        self.vector.push(input);

        return accumulator;
    }
}