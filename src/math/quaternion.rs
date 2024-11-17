use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use super::{matrix::Matrix, vector::{DotProduct, MatrixTransform, Vector, Vector4}, LerpTo, Magnitude, NearEq};

#[derive(Debug, Clone, Copy, PartialEq)]
#[must_use]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl From<Vector4> for Quaternion {
    #[inline]
    fn from(value: Vector4) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<Quaternion> for Vector4 {
    #[inline]
    fn from(value: Quaternion) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl Vector for Quaternion {}

impl Quaternion {
    pub const ZERO:   Self = Self { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
    pub const ONE:    Self = Self { x: 1.0, y: 1.0, z: 1.0, w: 1.0 };
    pub const UNIT_X: Self = Self { x: 1.0, y: 0.0, z: 0.0, w: 0.0 };
    pub const UNIT_Y: Self = Self { x: 0.0, y: 1.0, z: 0.0, w: 0.0 };
    pub const UNIT_Z: Self = Self { x: 0.0, y: 0.0, z: 1.0, w: 0.0 };
    pub const UNIT_W: Self = Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };

    pub const IDENTITY: Self = Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };

    #[inline]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    #[inline]
    pub fn invert(self) -> Self {
        let inv_magnitude = 1.0 / self.magnitude();
        Self {
            x: self.x * -inv_magnitude,
            y: self.y * -inv_magnitude,
            z: self.z * -inv_magnitude,
            w: self.w *  inv_magnitude,
        }
    }
}

impl Neg for Quaternion {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Add for Quaternion {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl AddAssign for Quaternion {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Quaternion {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl SubAssign for Quaternion {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul for Quaternion {
    type Output = Self;

    #[inline]
    /// Calculate two-quaternion multiplication
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.w + self.w * rhs.x + self.y * rhs.z - self.z * rhs.y,
            y: self.y * rhs.w + self.w * rhs.y + self.z * rhs.x - self.x * rhs.z,
            z: self.z * rhs.w + self.w * rhs.z + self.x * rhs.y - self.y * rhs.x,
            w: self.w * rhs.w - self.w * rhs.x - self.w * rhs.y - self.z * rhs.z,
        }
    }
}

impl MulAssign for Quaternion {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Div for Quaternion {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w,
        }
    }
}

impl DivAssign for Quaternion {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Add<f32> for Quaternion {
    type Output = Self;

    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
            w: self.w + rhs,
        }
    }
}

impl AddAssign<f32> for Quaternion {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs;
    }
}

impl Sub<f32> for Quaternion {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
            w: self.w - rhs,
        }
    }
}

impl SubAssign<f32> for Quaternion {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        *self = *self - rhs;
    }
}

impl Mul<f32> for Quaternion {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl MulAssign<f32> for Quaternion {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Div<f32> for Quaternion {
    type Output = Self;

    /// Multiplies by the reciprocal
    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        let inv = 1.0 / rhs;
        Self {
            x: self.x * inv,
            y: self.y * inv,
            z: self.z * inv,
            w: self.w * inv,
        }
    }
}

impl DivAssign<f32> for Quaternion {
    /// Multiplies by the reciprocal
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl Add<Quaternion> for f32 {
    type Output = Quaternion;

    #[inline]
    fn add(self, rhs: Quaternion) -> Self::Output {
        Quaternion {
            x: self + rhs.x,
            y: self + rhs.y,
            z: self + rhs.z,
            w: self + rhs.w,
        }
    }
}

impl Sub<Quaternion> for f32 {
    type Output = Quaternion;

    #[inline]
    fn sub(self, rhs: Quaternion) -> Self::Output {
        Quaternion {
            x: self - rhs.x,
            y: self - rhs.y,
            z: self - rhs.z,
            w: self - rhs.w,
        }
    }
}

impl Mul<Quaternion> for f32 {
    type Output = Quaternion;

    #[inline]
    fn mul(self, rhs: Quaternion) -> Self::Output {
        Quaternion {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
            w: self * rhs.w,
        }
    }
}

impl Div<Quaternion> for f32 {
    type Output = Quaternion;

    #[inline]
    fn div(self, rhs: Quaternion) -> Self::Output {
        Quaternion {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
            w: self / rhs.w,
        }
    }
}

impl LerpTo for Quaternion {
    #[inline]
    fn lerp_to(self, target: Self, amount: f32) -> Self {
        Self {
            x: self.x.lerp_to(target.x, amount),
            y: self.y.lerp_to(target.y, amount),
            z: self.z.lerp_to(target.z, amount),
            w: self.w.lerp_to(target.w, amount),
        }
    }
}

impl NearEq for Quaternion {
    #[inline]
    fn near_eq(self, other: Self) -> bool {
        self.x.near_eq(other.x) &&
        self.y.near_eq(other.y) &&
        self.z.near_eq(other.z) &&
        self.w.near_eq(other.w)
    }
}

impl DotProduct for Quaternion {
    #[inline]
    fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x +
        self.y * rhs.y +
        self.z * rhs.z +
        self.w * rhs.w
    }
}

impl MatrixTransform for Quaternion {
    #[inline]
    fn transform(self, mat: Matrix) -> Self {
        Self {
            x: mat[0][0] * self.x + mat[0][1] * self.y + mat[0][2] * self.z + mat[0][3] * self.w,
            y: mat[1][0] * self.x + mat[1][1] * self.y + mat[1][2] * self.z + mat[1][3] * self.w,
            z: mat[2][0] * self.x + mat[2][1] * self.y + mat[2][2] * self.z + mat[2][3] * self.w,
            w: mat[3][0] * self.x + mat[3][1] * self.y + mat[3][2] * self.z + mat[3][3] * self.w,
        }
    }
}
