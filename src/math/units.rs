//! Helpers for communicating the expected usage of float parameters

use std::ops::{Neg, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use super::Wrap;

pub trait Unit
where
    Self:
        Sized + Clone + Copy +
        Wrap +
        PartialEq + PartialOrd +
        Neg<Output = Self> +
        Add<Output = Self> + Sub<Output = Self> + AddAssign<Self> + SubAssign<Self> +
        Mul<f32, Output = Self> + MulAssign<f32> + Div<f32, Output = Self> + DivAssign<f32>
{
    type One: Singular;

    fn clamp(self, min: Self, max: Self) -> Self;
}

pub trait Singular {
    type Plural: Unit;
}

/// Indicates that the parameter is expected as a ratio of x units of `T` per y units of `U`
pub struct Ratio<T: Unit, U: Unit>(pub T, pub U::One);

pub trait Angular: Sized + Unit {
    /// 0 | 0 degrees
    const ZERO: Self;
    /// 1/16 turn
    const FRAC_1_16: Self;
    /// 1/12 turn | 30 degrees
    const FRAC_1_12: Self;
    /// 1/8 turn | 45 degrees
    const FRAC_1_8: Self;
    /// 1/4 turn | 90 degrees
    const FRAC_1_4: Self;
    /// 1/2 turn | 180 degrees
    const FRAC_1_2: Self;
    /// 1 turn | 360 degrees
    const FULL: Self;

    /// Test whether the angle is between 0 and 360 degrees
    fn is_positive_normal(self) -> bool {
        Self::ZERO <= self && self <= Self::FULL
    }

    /// Test whether the angle is between -360 and 0 degrees
    fn is_negative_normal(self) -> bool {
        Self::ZERO <= self && self <= Self::FULL
    }

    /// Test whether the angle is between -180 and +180 degrees
    fn is_signed_normal(self) -> bool {
        -Self::FRAC_1_2 <= self && self <= Self::FRAC_1_2
    }

    /// Add while staying within -360 and +360 degrees
    fn wrapping_add(self, rhs: Self) -> Self {
        self.add(rhs).wrap(-Self::FULL, Self::FULL)
    }
}

macro_rules! define_unit {
    (
        $(#[$unit_meta:meta])*
        $unit:ident

        $(#[$singluar_meta:meta])*
        $singular:ident
    ) => {
        $(#[$singluar_meta])*
        pub struct $singular;

        impl Singular for $singular {
            type Plural = $unit;
        }

        $(#[$unit_meta])*
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
        pub struct $unit(pub f32);

        impl Unit for $unit {
            type One = $singular;

            #[inline]
            fn clamp(self, min: Self, max: Self) -> Self {
                Self(self.0.clamp(min.0, max.0))
            }
        }

        impl $unit {
            #[inline]
            pub fn min(self, other: Self) -> Self {
                Self(self.0.min(other.0))
            }
            #[inline]
            pub fn max(self, other: Self) -> Self {
                Self(self.0.max(other.0))
            }
            #[inline]
            pub fn clamp(self, min: Self, max: Self) -> Self {
                Self(self.0.clamp(min.0, max.0))
            }
            #[inline]
            pub const fn per<T: Unit>(self, unit: T::One) -> Ratio<Self, T> {
                Ratio(self, unit)
            }
        }

        impl Neg for $unit {
            type Output = Self;
            #[inline]
            fn neg(self) -> Self::Output { Self(self.0.neg()) }
        }
        impl Add for $unit {
            type Output = Self;
            #[inline]
            fn add(self, rhs: Self) -> Self::Output { Self(self.0.add(rhs.0)) }
        }
        impl AddAssign for $unit {
            #[inline]
            fn add_assign(&mut self, rhs: Self) { self.0.add_assign(rhs.0) }
        }
        impl Sub for $unit {
            type Output = Self;
            #[inline]
            fn sub(self, rhs: Self) -> Self::Output { Self(self.0.sub(rhs.0)) }
        }
        impl SubAssign for $unit {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) { self.0.sub_assign(rhs.0) }
        }
        impl Mul<f32> for $unit {
            type Output = Self;
            #[inline]
            fn mul(self, rhs: f32) -> Self::Output { Self(self.0.mul(rhs)) }
        }
        impl MulAssign<f32> for $unit {
            #[inline]
            fn mul_assign(&mut self, rhs: f32) { self.0.mul_assign(rhs) }
        }
        impl Div<f32> for $unit {
            type Output = Self;
            #[inline]
            fn div(self, rhs: f32) -> Self::Output { Self(self.0.div(rhs)) }
        }
        impl DivAssign<f32> for $unit {
            #[inline]
            fn div_assign(&mut self, rhs: f32) { self.0.div_assign(rhs) }
        }
        impl Mul<$unit> for f32 {
            type Output = $unit;
            #[inline]
            fn mul(self, rhs: $unit) -> Self::Output { $unit(self.mul(rhs.0)) }
        }
        impl MulAssign<$unit> for f32 {
            #[inline]
            fn mul_assign(&mut self, rhs: $unit) { self.mul_assign(rhs.0) }
        }
        impl Div<$unit> for f32 {
            type Output = $unit;
            #[inline]
            fn div(self, rhs: $unit) -> Self::Output { $unit(self.div(rhs.0)) }
        }
        impl DivAssign<$unit> for f32 {
            #[inline]
            fn div_assign(&mut self, rhs: $unit) { self.div_assign(rhs.0) }
        }
        impl Wrap for $unit {
            #[inline]
            fn wrap(self, min: Self, max: Self) -> Self { Self(self.0.wrap(min.0, max.0)) }
        }
    };
}


define_unit!(
    /// Indicates that the parameter is expected in radians `[0..2pi]`
    Radians
    /// One radian
    Radian
);

impl Angular for Radians {
    const ZERO:      Self = Self(0.0);
    const FRAC_1_16: Self = Self(std::f32::consts::FRAC_PI_8);
    const FRAC_1_12: Self = Self(std::f32::consts::FRAC_PI_6);
    const FRAC_1_8:  Self = Self(std::f32::consts::FRAC_PI_2);
    const FRAC_1_4:  Self = Self(std::f32::consts::FRAC_PI_2);
    const FRAC_1_2:  Self = Self(std::f32::consts::PI);
    const FULL:      Self = Self(std::f32::consts::TAU);
}

impl Radians {
    #[inline] #[must_use] pub fn sin(self) -> f32 { self.0.sin() }
    #[inline] #[must_use] pub fn cos(self) -> f32 { self.0.cos() }
    #[inline] #[must_use] pub fn tan(self) -> f32 { self.0.tan() }
    #[inline] #[must_use] pub fn sin_cos(self) -> (f32, f32) { self.0.sin_cos() }
}

define_unit!(
    /// Indicates that the parameter is expected in degrees `[0..360]`
    Degrees
    /// One degree
    Degree
);

impl Angular for Degrees {
    const ZERO:      Self = Self(  0.0);
    const FRAC_1_16: Self = Self( 22.5);
    const FRAC_1_12: Self = Self( 30.0);
    const FRAC_1_8:  Self = Self( 45.0);
    const FRAC_1_4:  Self = Self( 90.0);
    const FRAC_1_2:  Self = Self(180.0);
    const FULL:      Self = Self(360.0);
}

define_unit!(
    /// Indicates that the parameter is expected as `[0..1]`
    Percent
    /// 100%
    Percentage
);

define_unit!(
    /// Indicates that the parameter is expected in seconds
    Seconds
    /// One second
    Second
);
