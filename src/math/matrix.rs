use std::ops::{Add, Mul, Sub};
use super::{quaternion::Quaternion, vector::{DotProduct, Normalize, Vector3}, Magnitude, NearEq, units::Radians};

/// Matrix, 4x4 components, column major, OpenGL style, right-handed
///
/// Array of rows; each row is an array of columns
#[derive(Debug, Clone, PartialEq, Default)]
#[must_use]
pub struct Matrix(pub [[f32; 4]; 4]);

impl Matrix {
    pub const IDENTITY: Self = Self([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    #[must_use]
    pub fn det(self) -> f32 {
        // Cache the matrix values (speed optimization)
        let [
            a00, a01, a02, a03,
            a10, a11, a12, a13,
            a20, a21, a22, a23,
            a30, a31, a32, a33,
        ] = <[f32; 16]>::from(self);

        (a30 * a21 * a12 * a03) - (a20 * a31 * a12 * a03) - (a30 * a11 * a22 * a03) + (a10 * a31 * a22 * a03) +
        (a20 * a11 * a32 * a03) - (a10 * a21 * a32 * a03) - (a30 * a21 * a02 * a13) + (a20 * a31 * a02 * a13) +
        (a30 * a01 * a22 * a13) - (a00 * a31 * a22 * a13) - (a20 * a01 * a32 * a13) + (a00 * a21 * a32 * a13) +
        (a30 * a11 * a02 * a23) - (a10 * a31 * a02 * a23) - (a30 * a01 * a12 * a23) + (a00 * a31 * a12 * a23) +
        (a10 * a01 * a32 * a23) - (a00 * a11 * a32 * a23) - (a20 * a11 * a02 * a33) + (a10 * a21 * a02 * a33) +
        (a20 * a01 * a12 * a33) - (a00 * a21 * a12 * a33) - (a10 * a01 * a22 * a33) + (a00 * a11 * a22 * a33)
    }

    #[inline]
    #[must_use]
    pub fn trace(self) -> f32 {
        self.0[0][0] + self.0[1][1] + self.0[2][2] + self.0[3][3]
    }

    #[must_use]
    pub const fn transpose(self) -> Self {
        Self([
            [self.0[0][0], self.0[1][0], self.0[2][0], self.0[3][0]],
            [self.0[0][1], self.0[1][1], self.0[2][1], self.0[3][1]],
            [self.0[0][2], self.0[1][2], self.0[2][2], self.0[3][2]],
            [self.0[0][3], self.0[1][3], self.0[2][3], self.0[3][3]],
        ])
    }

    pub fn invert(self) -> Self {
        // Cache the matrix values (speed optimization)
        let [
            a00, a01, a02, a03,
            a10, a11, a12, a13,
            a20, a21, a22, a23,
            a30, a31, a32, a33,
        ] = <[f32; 16]>::from(self);

        let b00 = a00 * a11 - a01 * a10;
        let b01 = a00 * a12 - a02 * a10;
        let b02 = a00 * a13 - a03 * a10;
        let b03 = a01 * a12 - a02 * a11;
        let b04 = a01 * a13 - a03 * a11;
        let b05 = a02 * a13 - a03 * a12;
        let b06 = a20 * a31 - a21 * a30;
        let b07 = a20 * a32 - a22 * a30;
        let b08 = a20 * a33 - a23 * a30;
        let b09 = a21 * a32 - a22 * a31;
        let b10 = a21 * a33 - a23 * a31;
        let b11 = a22 * a33 - a23 * a32;

        let inv_det = 1.0 / (b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06);

        Self([
            [
                ( a11 * b11 - a12 * b10 + a13 * b09) * inv_det,
                (-a10 * b11 + a12 * b08 - a13 * b07) * inv_det,
                ( a10 * b10 - a11 * b08 + a13 * b06) * inv_det,
                (-a10 * b09 + a11 * b07 - a12 * b06) * inv_det,
            ], [
                (-a01 * b11 + a02 * b10 - a03 * b09) * inv_det,
                ( a00 * b11 - a02 * b08 + a03 * b07) * inv_det,
                (-a00 * b10 + a01 * b08 - a03 * b06) * inv_det,
                ( a00 * b09 - a01 * b07 + a02 * b06) * inv_det,
            ], [
                ( a31 * b05 - a32 * b04 + a33 * b03) * inv_det,
                (-a30 * b05 + a32 * b02 - a33 * b01) * inv_det,
                ( a30 * b04 - a31 * b02 + a33 * b00) * inv_det,
                (-a30 * b03 + a31 * b01 - a32 * b00) * inv_det,
            ], [
                (-a21 * b05 + a22 * b04 - a23 * b03) * inv_det,
                ( a20 * b05 - a22 * b02 + a23 * b01) * inv_det,
                (-a20 * b04 + a21 * b02 - a23 * b00) * inv_det,
                ( a20 * b03 - a21 * b01 + a22 * b00) * inv_det,
            ],
        ])
    }

    /// Get translation matrix
    #[inline]
    pub const fn translate(x: f32, y: f32, z: f32) -> Self {
        Self([
            [1.0, 0.0, 0.0,   x],
            [0.0, 1.0, 0.0,   y],
            [0.0, 0.0, 1.0,   z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Get scaling matrix
    #[inline]
    pub const fn scale(x: f32, y: f32, z: f32) -> Self {
        Self([
            [  x, 0.0, 0.0, 0.0],
            [0.0,   y, 0.0, 0.0],
            [0.0, 0.0,   z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Create rotation matrix from axis and angle
    /// NOTE: Angle should be provided in radians
    pub fn rotate(axis: Vector3, angle: Radians) -> Self {
        let Vector3 { x, y, z } = axis.normalize();

        let (sinres, cosres) = angle.sin_cos();
        let t = 1.0 - cosres;

        Self([
            [x * x * t +     cosres,  x * y * t - z * sinres,  x * z * t + y * sinres,  0.0],
            [y * x * t + z * sinres,  y * y * t +     cosres,  y * z * t - x * sinres,  0.0],
            [z * x * t - y * sinres,  z * y * t + x * sinres,  z * z * t +     cosres,  0.0],
            [                   0.0,                     0.0,                     0.0,  1.0],
        ])
    }

    /// Get x-rotation matrix
    /// NOTE: Angle must be provided in radians
    #[inline]
    pub fn rotate_x(angle: Radians) -> Self {
        let (sinres, cosres) = angle.sin_cos();
        Self([
            [1.0,     0.0,      0.0,  0.0],
            [0.0,  cosres,  -sinres,  0.0],
            [0.0,  sinres,   cosres,  0.0],
            [0.0,     0.0,      0.0,  1.0],
        ])
    }

    /// Get y-rotation matrix
    /// NOTE: Angle must be provided in radians
    #[inline]
    pub fn rotate_y(angle: Radians) -> Self {
        let (sinres, cosres) = angle.sin_cos();
        Self([
            [ cosres,  0.0,  sinres,  0.0],
            [    0.0,  1.0,     0.0,  0.0],
            [-sinres,  0.0,  cosres,  0.0],
            [    0.0,  0.0,     0.0,  1.0],
        ])
    }

    /// Get z-rotation matrix
    /// NOTE: Angle must be provided in radians
    #[inline]
    pub fn rotate_z(angle: Radians) -> Self {
        let (sinres, cosres) = angle.sin_cos();
        Self([
            [cosres,  -sinres,  0.0,  0.0],
            [sinres,   cosres,  0.0,  0.0],
            [   0.0,      0.0,  1.0,  0.0],
            [   0.0,      0.0,  0.0,  1.0],
        ])
    }

    /// Get xyz-rotation matrix
    /// NOTE: Angle must be provided in radians
    pub fn rotate_xyz(x: Radians, y: Radians, z: Radians) -> Self {
        let (sin_x, cos_x) = x.sin_cos();
        let (sin_y, cos_y) = y.sin_cos();
        let (sin_z, cos_z) = z.sin_cos();
        Self([
            [                        cos_z * cos_y,                          sin_z * cos_y,         -sin_y,  0.0],
            [cos_z * sin_y * sin_x - sin_z * cos_x,  sin_z * sin_y * sin_x + cos_z * cos_x,  cos_y * sin_x,  0.0],
            [cos_z * sin_y * cos_x + sin_z * sin_x,  sin_z * sin_y * cos_x - cos_z * sin_x,  cos_y * cos_x,  0.0],
            [                                  0.0,                                    0.0,            0.0,  1.0],
        ])
    }

    /// Get perspective projection matrix
    pub fn frustrum(left: f64, right: f64, bottom: f64, top: f64, near_plane: f64, far_plane: f64) -> Self {
        let width  = (    right - left      ) as f32;
        let height = (      top - bottom    ) as f32;
        let depth  = (far_plane - near_plane) as f32;

        let left       = left       as f32;
        let right      = right      as f32;
        let top        = top        as f32;
        let bottom     = bottom     as f32;
        let near_plane = near_plane as f32;
        let  far_plane =  far_plane as f32;

        Self([
            [near_plane * 2.0 / width,                        0.0,   (    right + left      ) /  width,                                      0.0],
            [                     0.0,  near_plane * 2.0 / height,   (      top + bottom    ) / height,                                      0.0],
            [                     0.0,                        0.0,  -(far_plane + near_plane) /  depth,  -(far_plane * near_plane * 2.0) / depth],
            [                     0.0,                        0.0,                                -1.0,                                      0.0],
        ])
    }

    /// Get perspective projection matrix
    pub fn perspective(fovy: f64, aspect: f64, near_plane: f64, far_plane: f64) -> Self {
        let top   = near_plane * (fovy * 0.5).tan();
        let right = top * aspect;
        let bottom = -top;
        let left   = -right;

        Self::frustrum(left, right, bottom, top, near_plane, far_plane)
    }

    /// Get orthographic projection matrix
    pub fn ortho(left: f64, right: f64, bottom: f64, top: f64, near_plane: f64, far_plane: f64) -> Self {
        let width  = (    right - left      ) as f32;
        let height = (      top - bottom    ) as f32;
        let depth  = (far_plane - near_plane) as f32;

        let left       = left       as f32;
        let right      = right      as f32;
        let top        = top        as f32;
        let bottom     = bottom     as f32;
        let near_plane = near_plane as f32;
        let  far_plane =  far_plane as f32;

        Self([
            [2.0 / width,           0.0,           0.0,  -(    right + left      ) /  width],
            [        0.0,  2.0 / height,           0.0,  -(      top + bottom    ) / height],
            [        0.0,           0.0,  -2.0 / depth,  -(far_plane + near_plane) /  depth],
            [        0.0,           0.0,           0.0,                                 1.0],
        ])
    }

    pub fn look_at(eye: Vector3, target: Vector3, up: Vector3) -> Self {
        let vz = eye - target;
        let vx = up.cross_product(vz).normalize();
        let vy = vz.cross_product(vx);

        Self([
            [vx.x, vx.y, vx.z, -vx.dot(eye)],
            [vy.x, vy.y, vy.z, -vy.dot(eye)],
            [vz.x, vz.y, vz.z, -vz.dot(eye)],
            [ 0.0,  0.0,  0.0,          1.0],
        ])
    }

    /// Returns: (translation, rotation, scale)
    pub fn decompose(self) -> (Vector3, Quaternion, Vector3) {
        let translation = Vector3 {
            x: self.0[3][0],
            y: self.0[3][1],
            z: self.0[3][2],
        };

        // Extract upper-left for determinant computation
        let a = self.0[0][0];
        let b = self.0[1][0];
        let c = self.0[2][0];
        let d = self.0[0][1];
        let e = self.0[1][1];
        let f = self.0[2][1];
        let g = self.0[0][2];
        let h = self.0[1][2];
        let i = self.0[2][2];

        // Extract scale
        let det =
            a * (e * i - f * h) +
            b * (f * g - d * i) +
            c * (d * h - e * g);

        let scale = det.signum() * Vector3::new(
            Vector3::new(a, b, c).magnitude(),
            Vector3::new(d, e, f).magnitude(),
            Vector3::new(g, h, i).magnitude(),
        );

        let rotation = if !det.near_eq(0.0) {
            // Remove scale from the matrix if it is not close to zero
            let mut clone = self.clone();
            clone.0[0][0] /= scale.x;
            clone.0[1][0] /= scale.x;
            clone.0[2][0] /= scale.x;
            clone.0[0][1] /= scale.y;
            clone.0[1][1] /= scale.y;
            clone.0[2][1] /= scale.y;
            clone.0[0][2] /= scale.z;
            clone.0[1][2] /= scale.z;
            clone.0[2][2] /= scale.z;
            clone.into()
        } else {
            // Set to identity if close to zero
            Quaternion::IDENTITY
        };

        (translation, rotation, scale)
    }
}

impl Add for Matrix {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self([
            [
                self.0[0][0] + rhs.0[0][0],
                self.0[0][1] + rhs.0[0][1],
                self.0[0][2] + rhs.0[0][2],
                self.0[0][3] + rhs.0[0][3],
            ],
            [
                self.0[1][0] + rhs.0[1][0],
                self.0[1][1] + rhs.0[1][1],
                self.0[1][2] + rhs.0[1][2],
                self.0[1][3] + rhs.0[1][3],
            ],
            [
                self.0[2][0] + rhs.0[2][0],
                self.0[2][1] + rhs.0[2][1],
                self.0[2][2] + rhs.0[2][2],
                self.0[2][3] + rhs.0[2][3],
            ],
            [
                self.0[3][0] + rhs.0[3][0],
                self.0[3][1] + rhs.0[3][1],
                self.0[3][2] + rhs.0[3][2],
                self.0[3][3] + rhs.0[3][3],
            ],
        ])
    }
}

impl Sub for Matrix {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self([
            [
                self.0[0][0] - rhs.0[0][0],
                self.0[0][1] - rhs.0[0][1],
                self.0[0][2] - rhs.0[0][2],
                self.0[0][3] - rhs.0[0][3],
            ],
            [
                self.0[1][0] - rhs.0[1][0],
                self.0[1][1] - rhs.0[1][1],
                self.0[1][2] - rhs.0[1][2],
                self.0[1][3] - rhs.0[1][3],
            ],
            [
                self.0[2][0] - rhs.0[2][0],
                self.0[2][1] - rhs.0[2][1],
                self.0[2][2] - rhs.0[2][2],
                self.0[2][3] - rhs.0[2][3],
            ],
            [
                self.0[3][0] - rhs.0[3][0],
                self.0[3][1] - rhs.0[3][1],
                self.0[3][2] - rhs.0[3][2],
                self.0[3][3] - rhs.0[3][3],
            ],
        ])
    }
}

impl Mul for Matrix {
    type Output = Self;

    /// NOTE: When multiplying matrices... the order matters!
    fn mul(self, rhs: Self) -> Self::Output {
        Self([
            [
                self.0[0][0] * rhs.0[0][0] + self.0[1][0] * rhs.0[0][1] + self.0[2][0] * rhs.0[0][2] + self.0[3][0] * rhs.0[0][3],
                self.0[0][0] * rhs.0[1][0] + self.0[1][0] * rhs.0[1][1] + self.0[2][0] * rhs.0[1][2] + self.0[3][0] * rhs.0[1][3],
                self.0[0][0] * rhs.0[2][0] + self.0[1][0] * rhs.0[2][1] + self.0[2][0] * rhs.0[2][2] + self.0[3][0] * rhs.0[2][3],
                self.0[0][0] * rhs.0[3][0] + self.0[1][0] * rhs.0[3][1] + self.0[2][0] * rhs.0[3][2] + self.0[3][0] * rhs.0[3][3],
            ], [
                self.0[0][1] * rhs.0[0][0] + self.0[1][1] * rhs.0[0][1] + self.0[2][1] * rhs.0[0][2] + self.0[3][1] * rhs.0[0][3],
                self.0[0][1] * rhs.0[1][0] + self.0[1][1] * rhs.0[1][1] + self.0[2][1] * rhs.0[1][2] + self.0[3][1] * rhs.0[1][3],
                self.0[0][1] * rhs.0[2][0] + self.0[1][1] * rhs.0[2][1] + self.0[2][1] * rhs.0[2][2] + self.0[3][1] * rhs.0[2][3],
                self.0[0][1] * rhs.0[3][0] + self.0[1][1] * rhs.0[3][1] + self.0[2][1] * rhs.0[3][2] + self.0[3][1] * rhs.0[3][3],
            ], [
                self.0[0][2] * rhs.0[0][0] + self.0[1][2] * rhs.0[0][1] + self.0[2][2] * rhs.0[0][2] + self.0[3][2] * rhs.0[0][3],
                self.0[0][2] * rhs.0[1][0] + self.0[1][2] * rhs.0[1][1] + self.0[2][2] * rhs.0[1][2] + self.0[3][2] * rhs.0[1][3],
                self.0[0][2] * rhs.0[2][0] + self.0[1][2] * rhs.0[2][1] + self.0[2][2] * rhs.0[2][2] + self.0[3][2] * rhs.0[2][3],
                self.0[0][2] * rhs.0[3][0] + self.0[1][2] * rhs.0[3][1] + self.0[2][2] * rhs.0[3][2] + self.0[3][2] * rhs.0[3][3],
            ], [
                self.0[0][3] * rhs.0[0][0] + self.0[1][3] * rhs.0[0][1] + self.0[2][3] * rhs.0[0][2] + self.0[3][3] * rhs.0[0][3],
                self.0[0][3] * rhs.0[1][0] + self.0[1][3] * rhs.0[1][1] + self.0[2][3] * rhs.0[1][2] + self.0[3][3] * rhs.0[1][3],
                self.0[0][3] * rhs.0[2][0] + self.0[1][3] * rhs.0[2][1] + self.0[2][3] * rhs.0[2][2] + self.0[3][3] * rhs.0[2][3],
                self.0[0][3] * rhs.0[3][0] + self.0[1][3] * rhs.0[3][1] + self.0[2][3] * rhs.0[3][2] + self.0[3][3] * rhs.0[3][3],
            ],
        ])
    }
}

impl From<Matrix> for [f32; 16] {
    #[inline]
    fn from(Matrix(rows): Matrix) -> Self {
        [
            rows[0][0],
            rows[1][0],
            rows[2][0],
            rows[3][0],

            rows[0][1],
            rows[1][1],
            rows[2][1],
            rows[3][1],

            rows[0][2],
            rows[1][2],
            rows[2][2],
            rows[3][2],

            rows[0][3],
            rows[1][3],
            rows[2][3],
            rows[3][3],
        ]
    }
}
