pub mod vector;
pub mod quaternion;
pub mod matrix;

/// Communicates that the parameter is expected in radians
pub type Radians = f32;

/// Communicates that the parameter is expected in degrees
pub type Degrees = f32;

pub trait Magnitude {
    #[must_use]
    fn magnitude(self) -> f32;

    #[must_use]
    fn magnitude_sqr(self) -> f32;
}

impl Magnitude for f32 {
    #[inline]
    fn magnitude(self) -> f32 {
        self.abs()
    }

    #[inline]
    fn magnitude_sqr(self) -> f32 {
        self * self
    }
}

pub trait Distance {
    #[must_use]
    fn distance(self, other: Self) -> f32;

    #[must_use]
    fn distance_sqr(self, other: Self) -> f32;
}

impl Distance for f32 {
    #[inline]
    fn distance(self, other: Self) -> f32 {
        (self - other).magnitude()
    }

    #[inline]
    fn distance_sqr(self, other: Self) -> f32 {
        (self - other).magnitude_sqr()
    }
}

/// Linear interpolate other types using this type
///
/// Implicitly defined for any type which implements `lerp_to`
pub trait Lerp<T = Self> {
    #[must_use]
    fn lerp(self, start: T, end: T) -> T;
}

/// Linear interpolate from self to another target
pub trait LerpTo<Amount = f32> {
    #[must_use]
    fn lerp_to(self, target: Self, amount: Amount) -> Self;
}

impl<T: LerpTo<Amount>, Amount> Lerp<T> for Amount {
    #[inline]
    fn lerp(self, start: T, end: T) -> T {
        start.lerp_to(end, self)
    }
}

impl LerpTo for f32 {
    #[inline]
    fn lerp_to(self, target: Self, amount: f32) -> Self {
        self + amount * (target - self)
    }
}

pub trait NormalizeBetween {
    /// Normalize input value within input range
    #[must_use]
    fn normalize_between(self, start: Self, end: Self) -> Self;
}

impl NormalizeBetween for f32 {
    #[inline]
    fn normalize_between(self, start: Self, end: Self) -> Self {
        (self - start) / (end - start)
    }
}

pub trait Remap<In = Self, Out = Self> {
    type Output;

    /// Remap input value within input range to output range
    fn remap(self, input_start: In, input_end: In, output_start: Out, output_end: Out) -> Self::Output;
}

impl Remap for f32 {
    type Output = Self;

    #[inline]
    fn remap(self, input_start: Self, input_end: Self, output_start: Self, output_end: Self) -> Self::Output {
        output_start + (output_end - output_start) * (self - input_start) / (input_end - input_start)
    }
}

// Check whether two given floats are almost equal
pub trait NearEq {
    fn near_eq(self, other: Self) -> bool;
}

impl NearEq for f32 {
    #[inline]
    fn near_eq(self, other: Self) -> bool {
        (self - other).abs() <= Self::EPSILON * self.abs().max(other.abs()).max(1.0)
    }
}

impl NearEq for f64 {
    #[inline]
    fn near_eq(self, other: Self) -> bool {
        (self - other).abs() <= Self::EPSILON * self.abs().max(other.abs()).max(1.0)
    }
}
