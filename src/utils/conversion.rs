//! Conversion functions, e.g. from decibels to gain, frequency to pitch ...

use std::f64::consts;

/// Turn decibels to gain
pub fn db_to_gain(db: f64) -> f64 { 10.0_f64.powf( db * 0.05 ) }

/// Normalize frequencies to the range [0; pi] for use in filters.
pub fn f_to_omega(f: f64, sr: f64) -> f64 { consts::TAU * f / sr }