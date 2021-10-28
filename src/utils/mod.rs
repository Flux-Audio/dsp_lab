//! These modules contain pure functions, so they don't implement the Process
//! trait.

// pub mod clipping;            TODO:
// pub mod saturation;          TODO:
pub mod math;                // crossfading
pub mod conversion;          // pitch to freq, bpm to hz, pitch to 1v/oct

#[cfg(feature = "no_fpu")]
pub(crate) mod math_impl_no_fpu;
#[cfg(not(feature = "no_fpu"))]
pub(crate) mod math_impl;