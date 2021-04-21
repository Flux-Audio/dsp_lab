//! Conversion functions, e.g. from decibels to gain, frequency to pitch ...

use crate::utils::math::fast_powf;

pub fn db_to_gain(db: f32) -> f32 { fast_powf(10.0, db * 0.05) }