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
            int:  LeakyInt::new(0.03, 0.0),
            sr_scale: 1.0,
            dt_scale: 1.0,
            hardness: 0.5,
            drive: 0.0,
        }
    }

    pub fn set_sr(&mut self, sr: f64) {
        self.sr_scale = sr / 44100.0;
        self.dt_scale = 44100.0 / sr;
    }
}

impl Process<f64> for SlewClip1 {
    fn step(&mut self, input: f64) -> f64 {
        let post_gain = 1.0 - self.drive;
        let pre_gain  = 1.0 / post_gain;

        let dx = self.diff.step(input) * self.sr_scale;
        let dx_sat = var_clip(dx * pre_gain, self.hardness);
        self.int.step(dx_sat * post_gain) * self.dt_scale
    }
}

