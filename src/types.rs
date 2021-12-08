use std::cmp::Ordering;
use std::num::FpCategory;
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Rem, Sub};
use std::slice::Iter;
use num::{Float, Num, NumCast, One, ToPrimitive, Zero};
use num::traits::NumOps;

/// Bus type stores several N instances of the same Float type T. Busses implement Float themselves
/// and can thus be used in Process traits to process several floats in parallel (i.e. for stereo
/// or surround signals)
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq)]
pub struct Bus<T: Float, const N: usize> {
    pub channels: Vec<T>,
}


// === IMPLEMENTATIONS FOR BUS ===

impl<T: Float, const N: usize> From<&[T]> for Bus<T, N>
{
    fn from(slice: &[T]) -> Self {
        assert_eq!(slice.len(), N);
        Self{
            channels: Vec::from(slice)
        }
    }
}
#[test]
fn test_bus_from_slice() {
    let slice: &[f64] = &[0.0; 8];
    let _bus: Bus<f64, 8> = Bus::from(slice);
}





/*
impl<T: Float, const N: usize> Num for Bus<T, N> {
    type FromStrRadixErr = ();
    /// Unimplemented
    fn from_str_radix(_: &str, _: u32) -> Result<Self, Self::FromStrRadixErr> {
        Result::Err(())
    }
}

 */




impl<T: Float, const N: usize> Zero for Bus<T, N> {
    fn zero() -> Self {
        let slice: &[T] = &[T::zero(); N];
        Self::from(slice)
    }

    fn is_zero(&self) -> bool {
        for elem in self.channels {
            if !elem.is_zero() {
                return false;
            }
        }
        true
    }
}


// TODO: Zero for generic Bus


impl<T: Float, const N: usize> Add<Self> for Bus<T, N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Bus<T, N> {
        Bus::<T, N> {
            channels: self.channels
                .iter()
                .zip(rhs.channels.iter())
                .map(|(&a, &b)| a + b)
                .collect()
        }
    }
}

impl<T: Float, const N: usize> One for Bus<T, N> {
    fn one() -> Self {
        let slice: &[T] = &[T::one(); N];
        Self::from(slice)
    }
}

impl<T: Float, const N: usize> Mul<Self> for Bus<T, N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Bus<T, N> {
        Bus::<T, N> {
            channels: self.channels
                .iter()
                .zip(rhs.channels.iter())
                .map(|(&a, &b)| a * b)
                .collect()
        }
    }
}






/*
impl<T: Float> NumOps for Bus<T, N> {}
*/


/*
impl<T: Float> Add<Self, Output=Self> for Bus<T, N> {
    type Output = ();

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T: Float> Sub<Self, Output=Self> for Bus<T, N> {
    type Output = ();

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T: Float> Mul<Self, Output=Self> for Bus<T, N> {
    type Output = ();

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T: Float> Div<Self, Output=Self> for Bus<T, N> {
    type Output = ();

    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T: Float> Rem<Self, Output=Self> for Bus<T, N> {
    type Output = ();

    fn rem(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T: Float> Copy for Bus<T, N> {}

impl<T: Float> Clone for Bus<T, N> {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl<T: Float> NumCast for Bus<T, N> {
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        todo!()
    }
}

impl<T: Float> ToPrimitive for Bus<T, N> {
    fn to_i64(&self) -> Option<i64> {
        todo!()
    }

    fn to_u64(&self) -> Option<u64> {
        todo!()
    }
}

impl<T: Float> PartialOrd for Bus<T, N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!()
    }
}

impl<T: Float> Neg<Output=Self> for Bus<T, N> {
    type Output = ();

    fn neg(self) -> Self::Output {
        todo!()
    }
}

impl<T: Float, const N: usize> Float for Bus<T, N> {
    fn nan() -> Self {
        todo!()
    }

    fn infinity() -> Self {
        todo!()
    }

    fn neg_infinity() -> Self {
        todo!()
    }

    fn neg_zero() -> Self {
        todo!()
    }

    fn min_value() -> Self {
        todo!()
    }

    fn min_positive_value() -> Self {
        todo!()
    }

    fn max_value() -> Self {
        todo!()
    }

    fn is_nan(self) -> bool {
        todo!()
    }

    fn is_infinite(self) -> bool {
        todo!()
    }

    fn is_finite(self) -> bool {
        todo!()
    }

    fn is_normal(self) -> bool {
        todo!()
    }

    fn classify(self) -> FpCategory {
        todo!()
    }

    fn floor(self) -> Self {
        todo!()
    }

    fn ceil(self) -> Self {
        todo!()
    }

    fn round(self) -> Self {
        todo!()
    }

    fn trunc(self) -> Self {
        todo!()
    }

    fn fract(self) -> Self {
        todo!()
    }

    fn abs(self) -> Self {
        todo!()
    }

    fn signum(self) -> Self {
        todo!()
    }

    fn is_sign_positive(self) -> bool {
        todo!()
    }

    fn is_sign_negative(self) -> bool {
        todo!()
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        todo!()
    }

    fn recip(self) -> Self {
        todo!()
    }

    fn powi(self, n: i32) -> Self {
        todo!()
    }

    fn powf(self, n: Self) -> Self {
        todo!()
    }

    fn sqrt(self) -> Self {
        todo!()
    }

    fn exp(self) -> Self {
        todo!()
    }

    fn exp2(self) -> Self {
        todo!()
    }

    fn ln(self) -> Self {
        todo!()
    }

    fn log(self, base: Self) -> Self {
        todo!()
    }

    fn log2(self) -> Self {
        todo!()
    }

    fn log10(self) -> Self {
        todo!()
    }

    fn max(self, other: Self) -> Self {
        todo!()
    }

    fn min(self, other: Self) -> Self {
        todo!()
    }

    fn abs_sub(self, other: Self) -> Self {
        todo!()
    }

    fn cbrt(self) -> Self {
        todo!()
    }

    fn hypot(self, other: Self) -> Self {
        todo!()
    }

    fn sin(self) -> Self {
        todo!()
    }

    fn cos(self) -> Self {
        todo!()
    }

    fn tan(self) -> Self {
        todo!()
    }

    fn asin(self) -> Self {
        todo!()
    }

    fn acos(self) -> Self {
        todo!()
    }

    fn atan(self) -> Self {
        todo!()
    }

    fn atan2(self, other: Self) -> Self {
        todo!()
    }

    fn sin_cos(self) -> (Self, Self) {
        todo!()
    }

    fn exp_m1(self) -> Self {
        todo!()
    }

    fn ln_1p(self) -> Self {
        todo!()
    }

    fn sinh(self) -> Self {
        todo!()
    }

    fn cosh(self) -> Self {
        todo!()
    }

    fn tanh(self) -> Self {
        todo!()
    }

    fn asinh(self) -> Self {
        todo!()
    }

    fn acosh(self) -> Self {
        todo!()
    }

    fn atanh(self) -> Self {
        todo!()
    }

    fn integer_decode(self) -> (u64, i16, i8) {
        todo!()
    }
}
*/
