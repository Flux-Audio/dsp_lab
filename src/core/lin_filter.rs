//! Linear filters.
//! 
//! + 1-pole high-pass and low-pass topologies  TODO:
//! + 2-pole filters, based on an SVF core
//! + Generic FIR filters   TODO:

use crate::traits::Process;

// 2-pole state variable filter. Implements lowpass, highpass, notch and
// bandpass filters with shared state. Is used internally by filter processes.
struct SvfCore {
    pub lp: f64,
    pub hp: f64,
    pub bs: f64,
    pub bp: f64,
    pub cutoff: f64,
    pub res:    f64,
    pub sr:     f64,
}

impl SvfCore {
    /// Initialize filter state variables.
    pub fn new() -> Self {
        Self {
            lp: 0.0,
            hp: 0.0,
            bs: 0.0,
            bp: 0.0,
            cutoff: 0.0,
            res:    0.0,
            sr:     0.0,
        }
    }

    // Compute lowpass, highpass, notch and bandpass filtering of input with
    // variable resonance and cutoff.
    pub fn filter(&mut self, input: f64) {
        // Pre-process
        let f = 2.0 * (std::f64::consts::PI * self.cutoff / self.sr).sin();
        let q = (1.0 - self.res) * 2.0;

        // Filtering
        let lp = self.bp * f + self.lp;
        let hp = input - lp - q * self.bp;
        let bs = hp + lp;
        let bp = hp * f + self.bp;

        // Update state:
        self.lp = lp;
        self.hp = hp;
        self.bs = bs; 
        self.bp = bp;
    }
}


/// 2-pole SVF low-pass filter
/// TODO: test this
pub struct SVFLowPass {
    core: SvfCore,
    pub cutoff: f64,
    pub res: f64,
}

impl Process<f64> for SVFLowPass {
    fn step(&mut self, input: f64) -> f64 {
        self.core.cutoff = self.cutoff;
        self.core.res = self.res;
        self.core.filter(input);
        return self.core.lp;
    }
}

impl SVFLowPass {
    pub fn new() -> Self {
        Self {
            core: SvfCore::new(),
            cutoff: 0.0,
            res: 0.0,
        }
    }
}


/// 2-pole SVF high-pass filter
/// TODO: test this
pub struct SVFHighPass {
    core: SvfCore,
    pub cutoff: f64,
    pub res: f64,
}

impl Process<f64> for SVFHighPass {
    fn step(&mut self, input: f64) -> f64 {
        self.core.cutoff = self.cutoff;
        self.core.res = self.res;
        self.core.filter(input);
        return self.core.hp;
    }
}

impl SVFHighPass {
    pub fn new() -> Self {
        Self {
            core: SvfCore::new(),
            cutoff: 0.0,
            res: 0.0,
        }
    }
}


/// 2-pole SVF band-pass filter
/// TODO: test this
pub struct SVFBandPass {
    core: SvfCore,
    pub cutoff: f64,
    pub res: f64,
}

impl Process<f64> for SVFBandPass {
    fn step(&mut self, input: f64) -> f64 {
        self.core.cutoff = self.cutoff;
        self.core.res = self.res;
        self.core.filter(input);
        return self.core.bp;
    }
}

impl SVFBandPass {
    pub fn new() -> Self {
        Self {
            core: SvfCore::new(),
            cutoff: 0.0,
            res: 0.0,
        }
    }
}


/// 2-pole SVF band-stop filter
/// TODO: test this
pub struct SVFBandStop {
    core: SvfCore,
    pub cutoff: f64,
    pub res: f64,
}

impl Process<f64> for SVFBandStop {
    fn step(&mut self, input: f64) -> f64 {
        self.core.cutoff = self.cutoff;
        self.core.res = self.res;
        self.core.filter(input);
        return self.core.bs;
    }
}

impl SVFBandStop {
    pub fn new() -> Self {
        Self {
            core: SvfCore::new(),
            cutoff: 0.0,
            res: 0.0,
        }
    }
}


/*
TODO: this is outdated

/// DC offset blocking filter.
pub struct BlockDC {
    x_z1: f32,
    y_z1: f32,
}

impl BlockDC {
    /// Initialize filter state variables.
    pub fn new() -> Self {
        Self {
            x_z1: 0.0,
            y_z1: 0.0,
        }
    }

    /// Weak DC blocker. Use for blocking constant DC in input.
    pub fn filter_weak(&mut self, input: f32) -> f32 {
        self.y_z1 = input - self.x_z1 + 0.995*self.y_z1;
        self.x_z1 = input;
        return self.y_z1;
    }

    /// Medium DC blocker. Use for blocking sub-sonic sound in input.
    pub fn filter_medium(&mut self, input: f32) -> f32 {
        self.y_z1 = input - self.x_z1 + 0.9*self.y_z1;
        self.x_z1 = input;
        return self.y_z1;
    }

    /// Strong DC blocker. Use for blocking DC in feedback loops, i.e. for stabilizing
    /// an unstable feedback loop. For this application you might want to combine
    /// it with a highpass filter at approximately 18kHz.
    /// 
    /// Note also that this filter will remove some of the sub bass, so use only
    /// when strictly necessary.
    pub fn filter_strong(&mut self, input: f32) -> f32 {
        self.y_z1 = input - self.x_z1 + 0.5*self.y_z1;
        self.x_z1 = input;
        return self.y_z1;
    }
}*/