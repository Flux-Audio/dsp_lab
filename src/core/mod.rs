//! This module contains basic processes.

// pub mod integrators;         TODO:
// pub mod derivatives;         TODO:
// pub mod lin_filters;         // TODO: linear filters
// pub mod non_lin_filters;     // TODO: non-linear filters, like slew limiters, rolling median
// pub mod oscillators;         TODO:
// pub mod envelopes;           TODO:
// pub mod chaos;               TODO:
// pub mod delay;               // TODO: delay line with interpolation
// pub mod fft;                 TODO:

use crate::traits::Process;
use num::Float;

/// This process does nothing, except passing the values supplied by step to the
/// output unchanged. It is mostly for debugging and testing purposes. It can
/// also serve as a template for developing custom processes.
pub struct EmptyProcess {}

impl<T> Process<T> for EmptyProcess 
where T: Float
{
    fn step(&mut self, input: T) -> T { input }
}