use crate::traits::Process;
//use crate::core::chaos::NoiseWhite;
use super::Hysteresis;

pub struct WriteHead {
    hyst: Hysteresis,
    //nse:  NoiseWhite,
}

impl Process<f64> for WriteHead {
    fn step(&mut self, input: f64) -> f64 {
        return 0.0;
    }
}