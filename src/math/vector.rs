use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use super::{matrix::Matrix, Distance, LerpTo, Magnitude, NearEq};

pub trait DotProduct {
    #[must_use]
    fn dot(self, rhs: Self) -> f32;
}

pub trait MatrixTransform {
    #[must_use]
    fn transform(self, mat: Matrix) -> Self;
}

/// Vector normalization
pub trait Normalize {
    #[must_use]
    fn normalize(self) -> Self;
}

// Helper for providing vector-wide implementations
pub trait Vector:
    Sized + Copy +
    Neg<Output = Self> +
    Add<Output = Self> + AddAssign + Add<f32, Output = Self> + AddAssign<f32> +
    Sub<Output = Self> + SubAssign + Sub<f32, Output = Self> + SubAssign<f32> +
    Mul<Output = Self> + MulAssign + Mul<f32, Output = Self> + MulAssign<f32> +
    Div<Output = Self> + DivAssign + Div<f32, Output = Self> + DivAssign<f32> +
    DotProduct + MatrixTransform
{}

impl<T: Vector> Normalize for T {
    #[inline]
    fn normalize(self) -> Self {
        self / self.magnitude()
    }
}

impl<T: Vector> Magnitude for T {
    #[inline]
    fn magnitude(self) -> f32 {
        self.magnitude_sqr().sqrt()
    }

    #[inline]
    fn magnitude_sqr(self) -> f32 {
        self.dot(self)
    }
}

impl<T: Vector> Distance for T {
    #[inline]
    fn distance(self, other: Self) -> f32 {
        (self - other).magnitude()
    }

    #[inline]
    fn distance_sqr(self, other: Self) -> f32 {
        (self - other).magnitude_sqr()
    }
}

//////////////////////////////////////////////////
// Vector2
//////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[must_use]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector for Vector2 {}

impl Vector2 {
    pub const ZERO:   Self = Self { x: 0.0, y: 0.0 };
    pub const ONE:    Self = Self { x: 1.0, y: 1.0 };
    pub const UNIT_X: Self = Self { x: 1.0, y: 0.0 };
    pub const UNIT_Y: Self = Self { x: 0.0, y: 1.0 };

    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Neg for Vector2 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Add for Vector2 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vector2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Sub for Vector2 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vector2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl Mul for Vector2 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl MulAssign for Vector2 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl Div for Vector2 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl DivAssign for Vector2 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs
    }
}

impl Add<f32> for Vector2 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl AddAssign<f32> for Vector2 {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs
    }
}

impl Sub<f32> for Vector2 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl SubAssign<f32> for Vector2 {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        *self = *self - rhs
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<f32> for Vector2 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs
    }
}

impl Div<f32> for Vector2 {
    type Output = Self;

    /// Multiplies by the reciprocal
    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        let inv = 1.0 / rhs;
        Self {
            x: self.x * inv,
            y: self.y * inv,
        }
    }
}

impl DivAssign<f32> for Vector2 {
    /// Multiplies by the reciprocal
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl Add<Vector2> for f32 {
    type Output = Vector2;

    #[inline]
    fn add(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self + rhs.x,
            y: self + rhs.y,
        }
    }
}

impl Sub<Vector2> for f32 {
    type Output = Vector2;

    #[inline]
    fn sub(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self - rhs.x,
            y: self - rhs.y,
        }
    }
}

impl Mul<Vector2> for f32 {
    type Output = Vector2;

    #[inline]
    fn mul(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl Div<Vector2> for f32 {
    type Output = Vector2;

    #[inline]
    fn div(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self / rhs.x,
            y: self / rhs.y,
        }
    }
}

impl LerpTo for Vector2 {
    #[inline]
    fn lerp_to(self, target: Self, amount: f32) -> Self {
        Self {
            x: self.x.lerp_to(target.x, amount),
            y: self.y.lerp_to(target.y, amount),
        }
    }
}

impl NearEq for Vector2 {
    #[inline]
    fn near_eq(self, other: Self) -> bool {
        self.x.near_eq(other.x) &&
        self.y.near_eq(other.y)
    }
}

impl DotProduct for Vector2 {
    #[inline]
    fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x +
        self.y * rhs.y
    }
}

impl MatrixTransform for Vector2 {
    #[inline]
    fn transform(self, mat: Matrix) -> Self {
        Self {
            x: mat[0][0] * self.x + mat[0][1] * self.y + mat[0][2] * 0.0 + mat[0][3],
            y: mat[1][0] * self.x + mat[1][1] * self.y + mat[1][2] * 0.0 + mat[1][3],
        }
    }
}

//////////////////////////////////////////////////
// Vector3
//////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq)]
#[must_use]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector for Vector3 {}

impl Vector3 {
    pub const ZERO:   Self = Self { x: 0.0, y: 0.0, z: 0.0 };
    pub const ONE:    Self = Self { x: 1.0, y: 1.0, z: 1.0 };
    pub const UNIT_X: Self = Self { x: 1.0, y: 0.0, z: 0.0 };
    pub const UNIT_Y: Self = Self { x: 0.0, y: 1.0, z: 0.0 };
    pub const UNIT_Z: Self = Self { x: 0.0, y: 0.0, z: 1.0 };

    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn cross_product(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Neg for Vector3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vector3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vector3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Sub for Vector3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vector3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl Mul for Vector3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl MulAssign for Vector3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl Div for Vector3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl DivAssign for Vector3 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs
    }
}

impl Add<f32> for Vector3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl AddAssign<f32> for Vector3 {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs
    }
}

impl Sub<f32> for Vector3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl SubAssign<f32> for Vector3 {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        *self = *self - rhs
    }
}

impl Mul<f32> for Vector3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f32> for Vector3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs
    }
}

impl Div<f32> for Vector3 {
    type Output = Self;

    /// Multiplies by the reciprocal
    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        let inv = 1.0 / rhs;
        Self {
            x: self.x * inv,
            y: self.y * inv,
            z: self.z * inv,
        }
    }
}

impl DivAssign<f32> for Vector3 {
    /// Multiplies by the reciprocal
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl Add<Vector3> for f32 {
    type Output = Vector3;

    #[inline]
    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self + rhs.x,
            y: self + rhs.y,
            z: self + rhs.z,
        }
    }
}

impl Sub<Vector3> for f32 {
    type Output = Vector3;

    #[inline]
    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self - rhs.x,
            y: self - rhs.y,
            z: self - rhs.z,
        }
    }
}

impl Mul<Vector3> for f32 {
    type Output = Vector3;

    #[inline]
    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Div<Vector3> for f32 {
    type Output = Vector3;

    #[inline]
    fn div(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
        }
    }
}

impl LerpTo for Vector3 {
    #[inline]
    fn lerp_to(self, target: Self, amount: f32) -> Self {
        Self {
            x: self.x.lerp_to(target.x, amount),
            y: self.y.lerp_to(target.y, amount),
            z: self.z.lerp_to(target.z, amount),
        }
    }
}

impl NearEq for Vector3 {
    #[inline]
    fn near_eq(self, other: Self) -> bool {
        self.x.near_eq(other.x) &&
        self.y.near_eq(other.y) &&
        self.z.near_eq(other.z)
    }
}

impl DotProduct for Vector3 {
    #[inline]
    fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x +
        self.y * rhs.y +
        self.z * rhs.z
    }
}

impl MatrixTransform for Vector3 {
    #[inline]
    fn transform(self, mat: Matrix) -> Self {
        Self {
            x: mat[0][0] * self.x + mat[0][1] * self.y + mat[0][2] * self.z + mat[0][3],
            y: mat[1][0] * self.x + mat[1][1] * self.y + mat[1][2] * self.z + mat[1][3],
            z: mat[2][0] * self.x + mat[2][1] * self.y + mat[2][2] * self.z + mat[2][3],
        }
    }
}

//////////////////////////////////////////////////
// Vector4
//////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq)]
#[must_use]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector for Vector4 {}

impl Vector4 {
    pub const ZERO:   Self = Self { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
    pub const ONE:    Self = Self { x: 1.0, y: 1.0, z: 1.0, w: 1.0 };
    pub const UNIT_X: Self = Self { x: 1.0, y: 0.0, z: 0.0, w: 0.0 };
    pub const UNIT_Y: Self = Self { x: 0.0, y: 1.0, z: 0.0, w: 0.0 };
    pub const UNIT_Z: Self = Self { x: 0.0, y: 0.0, z: 1.0, w: 0.0 };
    pub const UNIT_W: Self = Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };

    #[inline]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

impl Neg for Vector4 {
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

impl Add for Vector4 {
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

impl AddAssign for Vector4 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Sub for Vector4 {
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

impl SubAssign for Vector4 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl Mul for Vector4 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}

impl MulAssign for Vector4 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl Div for Vector4 {
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

impl DivAssign for Vector4 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs
    }
}

impl Add<f32> for Vector4 {
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

impl AddAssign<f32> for Vector4 {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs
    }
}

impl Sub<f32> for Vector4 {
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

impl SubAssign<f32> for Vector4 {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        *self = *self - rhs
    }
}

impl Mul<f32> for Vector4 {
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

impl MulAssign<f32> for Vector4 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs
    }
}

impl Div<f32> for Vector4 {
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

impl DivAssign<f32> for Vector4 {
    /// Multiplies by the reciprocal
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl Add<Vector4> for f32 {
    type Output = Vector4;

    #[inline]
    fn add(self, rhs: Vector4) -> Self::Output {
        Vector4 {
            x: self + rhs.x,
            y: self + rhs.y,
            z: self + rhs.z,
            w: self + rhs.w,
        }
    }
}

impl Sub<Vector4> for f32 {
    type Output = Vector4;

    #[inline]
    fn sub(self, rhs: Vector4) -> Self::Output {
        Vector4 {
            x: self - rhs.x,
            y: self - rhs.y,
            z: self - rhs.z,
            w: self - rhs.w,
        }
    }
}

impl Mul<Vector4> for f32 {
    type Output = Vector4;

    #[inline]
    fn mul(self, rhs: Vector4) -> Self::Output {
        Vector4 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
            w: self * rhs.w,
        }
    }
}

impl Div<Vector4> for f32 {
    type Output = Vector4;

    #[inline]
    fn div(self, rhs: Vector4) -> Self::Output {
        Vector4 {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
            w: self / rhs.w,
        }
    }
}

impl LerpTo for Vector4 {
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

impl NearEq for Vector4 {
    #[inline]
    fn near_eq(self, other: Self) -> bool {
        self.x.near_eq(other.x) &&
        self.y.near_eq(other.y) &&
        self.z.near_eq(other.z) &&
        self.w.near_eq(other.w)
    }
}

impl DotProduct for Vector4 {
    #[inline]
    fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x +
        self.y * rhs.y +
        self.z * rhs.z +
        self.w * rhs.w
    }
}

impl MatrixTransform for Vector4 {
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
