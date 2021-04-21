use crate::traits::Process;
use rand::Rng;

/// Generate white noise (wrapper for rand::thread_rng)
pub struct NoiseWhite {
    rng: rand::thread_rng();
}

impl Process<f64> for NoiseWhite {
    fn step(&mut self, input: f64) -> f64 {
        self.rng.gen::<f64>()
    }
}


/*
// Generate violet noise
pub struct NoiseViolet {
    rng: rand::thread_rng();
    sr: 44100.0;
    nse_p = 0.0;
}

impl Process<f64> for NoiseViolet {
    fn step(&mut self, input: f64) -> f64 {
        self.nse_p = self.rng.gen::<f64>() - self.nse_p;

    }
}
*/