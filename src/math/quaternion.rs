use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use crate::prelude::*;

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
    fn from(Vector4 { x, y, z, w }: Vector4) -> Self {
        Self { x, y, z, w }
    }
}

impl From<Quaternion> for Vector4 {
    #[inline]
    fn from(Quaternion { x, y, z, w }: Quaternion) -> Self {
        Self { x, y, z, w }
    }
}

impl Vector for Quaternion {}

impl Quaternion {
    pub const ZERO:     Self = Self { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
    pub const ONE:      Self = Self { x: 1.0, y: 1.0, z: 1.0, w: 1.0 };
    pub const IDENTITY: Self = Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };

    #[inline]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    #[inline]
    pub const fn make(Vector3 { x, y, z }: Vector3, w: f32) -> Self {
        Self { x, y, z, w }
    }

    #[inline]
    pub const fn xyz(self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
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

impl Angle for Quaternion {
    fn angle(self, other: Self) -> Radians {
        todo!()
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
    fn lerp_to(self, target: Self, amount: Percent) -> Self {
        Self {
            x: self.x.lerp_to(target.x, amount),
            y: self.y.lerp_to(target.y, amount),
            z: self.z.lerp_to(target.z, amount),
            w: self.w.lerp_to(target.w, amount),
        }
    }
}

impl Quaternion {
    #[inline]
    #[must_use]
    pub fn nlerp_to(self, target: Self, amount: Percent) -> Normalized<Self> {
        self.lerp_to(target, amount).normalize()
    }

    #[inline]
    #[must_use]
    pub fn slerp_to(self, mut target: Self, amount: Percent) -> Self {
        let mut cos_half_theta = self.dot(target);
        if cos_half_theta < 0.0 {
            target = -target;
            cos_half_theta = -cos_half_theta;
        };

        if cos_half_theta >= 1.0 {
            self
        } else if cos_half_theta > 0.95 {
            self.nlerp_to(target, amount)
        } else {
            let half_theta = cos_half_theta.acos();
            let sin_half_theta = (1.0 - cos_half_theta * cos_half_theta).sqrt();

            if sin_half_theta.abs() < f32::EPSILON {
                self * 0.5 + target * 0.5
            } else {
                let ratio_a = ((1.0 - amount) * half_theta).sin() / sin_half_theta;
                let ratio_b = (       amount  * half_theta).sin() / sin_half_theta;

                self * ratio_a + target * ratio_b
            }
        }
    }


    /// Calculate quaternion cubic spline interpolation using Cubic Hermite Spline algorithm
    /// as described in the GLTF 2.0 specification: https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#interpolation-cubic
    pub fn cubic_hermine_spline(self, out_tangent: Self, next: Self, in_tangent: Self, t: f32) -> Normalized<Self> {
        let t2 = t  * t;
        let t3 = t2 * t;

        let p0 =        self * ( 2.0 * t3 - 3.0 * t2 + 1.0); // $p_0*(2t^3-3t^2+1)$
        let m0 = out_tangent * (       t3 - 2.0 * t2 +   t); // $m_0*(t^3-2t^2+t)$
        let p1 =        next * (-2.0 * t3 + 3.0 * t2      ); // $p_1*(-2t^3+3t^2)$
        let m1 =  in_tangent * (       t3 -       t2      ); // $m_1*(t^3-t^2)$

        (p0 + m0 + p1 + m1).normalize()
    }

    /// Calculate quaternion based on the rotation from one vector to another
    pub fn from_vector3_to_vector3(from: Vector3, to: Vector3) -> Normalized<Self> {
        let cross = from.cross_product(to);

        Self {
            x: cross.x,
            y: cross.y,
            z: cross.z,
            w: 1.0 + from.dot(to),
        }.normalize()
    }

    pub fn from_axis_angle(axis: Vector3, angle: Radians) -> Normalized<Self> {
        let axis = axis.normalize();

        let (sinres, cosres) = (angle * 0.5).sin_cos();

        Self {
            x: axis.x * sinres,
            y: axis.y * sinres,
            z: axis.z * sinres,
            w: cosres,
        }.normalize()
    }

    pub fn to_axis_angle(mut self) -> (Vector3, Radians) {
        if self.w.abs() > 1.0 {
            self = self.normalize();
        }

        let res_angle = self.w.acos() * 2.0;
        let den = (1.0 - self.w * self.w).sqrt();

        let res_axis = if den > f32::EPSILON {
            self.xyz() / den
        } else {
            // This occurs when the angle is zero.
            // Not a problem: just set an arbitrary normalized axis.
            Vector3::UNIT_X
        };

        (res_axis, res_angle)
    }

    pub fn from_euler(pitch: Radians, yaw: Radians, roll: Radians) -> Self {
        let x0 = (pitch * 0.5).cos();
        let x1 = (pitch * 0.5).sin();
        let y0 = (  yaw * 0.5).cos();
        let y1 = (  yaw * 0.5).sin();
        let z0 = ( roll * 0.5).cos();
        let z1 = ( roll * 0.5).sin();

        Self {
            x: x1 * y0 * z0 - x0 * y1 * z1,
            y: x0 * y1 * z0 + x1 * y0 * z1,
            z: x0 * y0 * z1 - x1 * y1 * z0,
            w: x0 * y0 * z0 + x1 * y1 * z1,
        }
    }

    /// (roll, pitch, yaw)
    pub fn to_euler(self) -> (Radians, Radians, Radians) {
        // Roll (x-axis rotation)
        let x0 =       2.0 * (self.w * self.x + self.y * self.z);
        let x1 = 1.0 - 2.0 * (self.x * self.x + self.y * self.y);
        let roll = x0.atan2(x1);

        // Pitch (y-axis rotation)
        let y0 = 2.0 * (self.w * self.y - self.z * self.x).clamp(-1.0, 1.0);
        let pitch = y0.asin();

        // Yaw (z-axis rotation)
        let z0 =       2.0 * (self.w * self.z + self.x * self.y);
        let z1 = 1.0 - 2.0 * (self.y * self.y + self.z * self.z);
        let yaw = z0.atan2(z1);

        (roll, pitch, yaw)
    }
}

impl From<Matrix> for Quaternion {
    /// Convert matrix into quaternion
    fn from(mat: Matrix) -> Self {
        let (biggest_index, four_biggest_squared_minus_1) = [
            mat.0[0][0] + mat.0[1][1] + mat.0[2][2],
            mat.0[0][0] - mat.0[1][1] - mat.0[2][2],
            mat.0[1][1] - mat.0[0][0] - mat.0[2][2],
            mat.0[2][2] - mat.0[0][0] - mat.0[1][1],
        ].into_iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap();

        let biggest_val = (four_biggest_squared_minus_1 + 1.0) * 0.5;
        let mult = 0.25 / biggest_val;

        match biggest_index {
            0 => Self {
                w: biggest_val,
                x: (mat.0[2][1] - mat.0[1][2]) * mult,
                y: (mat.0[0][2] - mat.0[2][0]) * mult,
                z: (mat.0[1][0] - mat.0[0][1]) * mult,
            },
            1 => Self {
                x: biggest_val,
                w: (mat.0[2][1] - mat.0[1][2]) * mult,
                y: (mat.0[1][0] + mat.0[0][1]) * mult,
                z: (mat.0[0][2] + mat.0[2][0]) * mult,
            },
            2 => Self {
                y: biggest_val,
                w: (mat.0[0][2] - mat.0[2][0]) * mult,
                x: (mat.0[1][0] + mat.0[0][1]) * mult,
                z: (mat.0[2][1] + mat.0[1][2]) * mult,
            },
            3 => Self {
                z: biggest_val,
                w: (mat.0[1][0] - mat.0[0][1]) * mult,
                x: (mat.0[0][2] + mat.0[2][0]) * mult,
                y: (mat.0[2][1] + mat.0[1][2]) * mult,
            },
            _ => unreachable!(),
        }
    }
}

impl From<Quaternion> for Matrix {
    /// Convert quaternion into matrix
    fn from(q: Quaternion) -> Self {
        let a2 = q.x * q.x;
        let b2 = q.y * q.y;
        let c2 = q.z * q.z;

        let ac = q.x * q.z;
        let ab = q.x * q.y;
        let bc = q.y * q.z;
        let ad = q.w * q.x;
        let bd = q.w * q.y;
        let cd = q.w * q.z;

        Matrix([
            [1.0 - 2.0 * (b2 + c2),        2.0 * (ab - cd),        2.0 * (ac + bd),  0.0],
            [      2.0 * (ab + cd),  1.0 - 2.0 * (a2 + c2),        2.0 * (bc - ad),  0.0],
            [      2.0 * (ac - bd),        2.0 * (bc + ad),  1.0 - 2.0 * (a2 + b2),  0.0],
            [                  0.0,                    0.0,                    0.0,  1.0],
        ])
    }
}

impl NearEq for Quaternion {
    #[inline]
    fn near_eq(self, other: Self) -> bool {
        self.x.near_eq( other.x) &&
        self.y.near_eq( other.y) &&
        self.z.near_eq( other.z) &&
        self.w.near_eq( other.w) ||
        self.x.near_eq(-other.x) &&
        self.y.near_eq(-other.y) &&
        self.z.near_eq(-other.z) &&
        self.w.near_eq(-other.w)
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
            x: mat.0[0][0] * self.x + mat.0[0][1] * self.y + mat.0[0][2] * self.z + mat.0[0][3] * self.w,
            y: mat.0[1][0] * self.x + mat.0[1][1] * self.y + mat.0[1][2] * self.z + mat.0[1][3] * self.w,
            z: mat.0[2][0] * self.x + mat.0[2][1] * self.y + mat.0[2][2] * self.z + mat.0[2][3] * self.w,
            w: mat.0[3][0] * self.x + mat.0[3][1] * self.y + mat.0[3][2] * self.z + mat.0[3][3] * self.w,
        }
    }
}
