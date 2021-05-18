use crate::traits::Process;
use crate::core::lin_filter::{Diff, LeakyInt};
use crate::utils::math::var_clip;

pub struct SlewClip1 {
    diff: Diff,
    int: LeakyInt,
    sr_scale: f64,
    dt_scale: f64,
    pub hardness: f64,
    pub drive: f64,
}

impl SlewClip1 {
    pub fn new() -> Self {
        Self {
            diff: Diff::new(),
            int:  LeakyInt::new(0.02, 0.0),
            sr_scale: 1.0,
            dt_scale: 1.0,
            hardness: 0.5,
            drive: 0.0,
        }
    }

    pub fn set_sr(&mut self, sr: f64) {
        self.sr_scale = sr / 44100.0;
        self.dt_scale = 44100.0 / sr.clamp(1.0, std::f64::MAX);
    }
}

impl Process<f64> for SlewClip1 {
    fn step(&mut self, input: f64) -> f64 {
        let post_gain = 1.0 - self.drive;
        let pre_gain  = 1.0 / post_gain.clamp(1e-30, 1.0);

        let dx = self.diff.step(input) * self.sr_scale;
        let dx_sat = var_clip(dx * pre_gain, self.hardness);
        self.int.step(dx_sat * post_gain) * self.dt_scale
    }
}


pub struct SlewClip2 {
    diff1: Diff,
    diff2: Diff,
    int1: LeakyInt,
    int2: LeakyInt,
    sr_scale: f64,
    dt_scale: f64,
    pub hardness: f64,
    pub drive: f64,
}

impl SlewClip2 {
    pub fn new() -> Self {
        Self {
            diff1: Diff::new(),
            diff2: Diff::new(),
            int1:  LeakyInt::new(0.02, 0.0),
            int2:  LeakyInt::new(0.02, 0.0),
            sr_scale: 1.0,
            dt_scale: 1.0,
            hardness: 0.5,
            drive: 0.0,
        }
    }

    pub fn set_sr(&mut self, sr: f64) {
        self.sr_scale = sr / 44100.0;
        self.dt_scale = 44100.0 / sr.clamp(1.0, std::f64::MAX);
    }
}

impl Process<f64> for SlewClip2 {
    fn step(&mut self, input: f64) -> f64 {
        let post_gain = 1.0 - self.drive;
        let pre_gain  = 1.0 / post_gain.clamp(1e-30, 1.0);

        let mut dx = self.diff1.step(input) * self.sr_scale;
        dx         = self.diff2.step(dx)    * self.sr_scale;
        let dx_sat = var_clip(dx * pre_gain, self.hardness);
        let y      = self.int1.step(dx_sat * post_gain) * self.dt_scale;
        self.int2.step(y) * self.dt_scale
    }
}

