use crate::utils::math::fast_sigmoid;
use crate::traits::Process;

/// Old model for hysteresis, use the others for writing new code. Models magnetic 
/// hysteresis found in transformer cores and magnetic tape.
pub struct HysteresisLegacy {
    x_p: f64,   // previous value of input
    y_p: f64,   // previous value of output
    pub sq:    f64,
    pub coerc: f64,
    // pub fast:  bool,
}

impl Process<f64> for HysteresisLegacy{
    fn step(&mut self, input: f64) -> f64 { 
        let dx: f64 = input - self.x_p;
        self.x_p = input;

        // calmp hysteresis parameters to avoid floating point errors and
        // NaN / infinity values
        self.sq = self.sq.clamp(0.0 , 1.00) * 1.45 - 0.5;

        // crossfade to stateless distortion, for small values of coercitivity
        let k   =  self.coerc.clamp(0.1, 1.0);
        let mix = (self.coerc.clamp(0.0, 0.2) * 5.0).sqrt().sqrt();

        // hysteresis loop equations
        let y_an: f64 = fast_sigmoid(input.abs().powf(1.0/(1.0 - self.sq)))
                      .powf(1.0 - self.sq) * input.signum();

        let y: f64 = self.y_p + (y_an - self.y_p) * dx.abs() / k;
        
        // prevent runaway accumulation by clamping
        self.y_p = (y * mix + y_an * (1.0 - mix)).clamp(-1.25, 1.25);

        return self.y_p;
    }
}

impl HysteresisLegacy{
    pub fn new() -> Self {
        Self{
            x_p: 0.0,
            y_p: 0.0,
            sq:   0.5,
            coerc: 0.5,
        }
    }
}


/// Models magnetic hysteresis found in transformer cores and magnetic tape.
/// 
/// Uses the Jiles-Atherton model of hysteresis, with trapezoid rule for derivatives
/// and `fast_sigmoid` instead of the Langevin function.
/// The derivative of `fast_sigmoid` is used instead of the derivative of the Langevin function.
/// This is equal to `1 - fast_sigmoid(x) * fast_sigmoid(x)`
pub struct MagneticHysteresis {
    pub sr: f64,
    pub a: f64,
    pub c: f64,
    pub k: f64,
    pub s: f64,
    x_z1:  f64,
    dx_z1: f64,
    y_z1:  f64,
}

impl Process<f64> for MagneticHysteresis {
    fn step(&mut self, x: f64) -> f64 {
        // set up Jiles-Atherton variables
        const ALPHA: f64 = 0.0016;      // I got this from Chow's hysteresis implementation
        let q = x * ALPHA*self.y_z1 / self.a;
        let lq = fast_sigmoid(q / 3.0); // Langevin function
        let dlq = 1.0 - lq * lq;        // derivative of Langevin function
        let delta_x = if x > self.x_z1 { 1.0 } else { -1.0 };
        let delta_y = if delta_x * (lq - self.y_z1) > 0.0 { 1.0 } else { 0.0 };

        // trapezoid derivative of x
        let dx = 2.0 * self.sr * (x - self.x_z1) - self.dx_z1;

        // set up solution of Jiles-Atherton with RK4 numeric integration. 
        // NOTE: all the aux variables are to split up the Jiles-Atherton equation 
        // to minimize number of operations. Not very readable, but exactly 
        // equivalent to the original formula, so look it up if it's confusing.
        let aux1 = 1.0 - self.c;
        let aux2 = self.s * lq;
        let aux3 = self.c * self.s / self.a - dlq;
        let aux4 = aux1 * delta_y;
        let aux5 = aux1 * delta_x * self.k;
        let aux6 = 1.0 - ALPHA * aux3;
        let dy = |y| {
            let aux7 = aux2 - y;
            dx * (aux4 * aux7 / (aux5 - ALPHA * aux7) + aux3) / aux6
        };      // dy is a closure, because we need to average 4 different 
                // versions of dy in RK4
        
        // RK4 step
        let dt = 1.0 / self.sr;
        let k1 = dy(self.y_z1);
        let k2 = dy(self.y_z1 + dt*k1/2.0);
        let k3 = dy(self.y_z1 + dt*k2/2.0);
        let k4 = dy(self.y_z1 + dt*k3);
        let y = self.y_z1 + 0.1666666666666667 * dt * (k1 + 2.0*k2 + 2.0*k3 + k4);

        // update state variables
        self.x_z1 = x;
        self.dx_z1 = dx;
        self.y_z1 = y;

        y
    }
}

/// Takes the ideal resistance, characteristics of the
/// resistor's material, ambient temperature and voltage drop across it and returns 
/// an effective resistance.
pub struct GenericResistance {}

/// Takes the ideal capacitance, characteristics of the
/// capacitor's material and construction, ambient temperature and voltage drop
/// across it and returns an effective capacitance.
pub struct GenericCapacitance {}

/// Takes the ideal inductance, characteristics of the inductor's material,
/// ambient temperature and voltage drop across it and returns an effective
/// inductance.
pub struct GenericInductance {}