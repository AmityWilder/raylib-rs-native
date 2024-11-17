use std::ops::{Add, Index, IndexMut, Mul, Sub};
use super::{vector::{DotProduct, Normalize, Vector3}, Radians};

/// Matrix, 4x4 components, column major, OpenGL style, right-handed
///
/// Array of rows; each row is an array of columns
#[derive(Debug, Clone, PartialEq, Default)]
#[must_use]
pub struct Matrix(pub [[f32; 4]; 4]);

impl Index<usize> for Matrix {
    type Output = [f32; 4];

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Matrix {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Matrix {
    pub const IDENTITY: Self = Self([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn perspective(fovy: f64, aspect: f64, near_plane: f64, far_plane: f64) -> Self {
        let top   = near_plane * (fovy * 0.5).tan();
        let right = top * aspect;
        let bottom = -top;
        let left   = -right;

        Self::frustrum(left, right, bottom, top, near_plane, far_plane)
    }

    /// Get orthographic projection matrix
    #[inline]
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

    #[inline]
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
}

impl Add for Matrix {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        #[allow(non_snake_case)]
        let (A, B) = (self, rhs);
        Self([
            [
                A[0][0] + B[0][0],
                A[0][1] + B[0][1],
                A[0][2] + B[0][2],
                A[0][3] + B[0][3],
            ],
            [
                A[1][0] + B[1][0],
                A[1][1] + B[1][1],
                A[1][2] + B[1][2],
                A[1][3] + B[1][3],
            ],
            [
                A[2][0] + B[2][0],
                A[2][1] + B[2][1],
                A[2][2] + B[2][2],
                A[2][3] + B[2][3],
            ],
            [
                A[3][0] + B[3][0],
                A[3][1] + B[3][1],
                A[3][2] + B[3][2],
                A[3][3] + B[3][3],
            ],
        ])
    }
}

impl Sub for Matrix {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        #[allow(non_snake_case)]
        let (A, B) = (self, rhs);
        Self([
            [
                A[0][0] - B[0][0],
                A[0][1] - B[0][1],
                A[0][2] - B[0][2],
                A[0][3] - B[0][3],
            ],
            [
                A[1][0] - B[1][0],
                A[1][1] - B[1][1],
                A[1][2] - B[1][2],
                A[1][3] - B[1][3],
            ],
            [
                A[2][0] - B[2][0],
                A[2][1] - B[2][1],
                A[2][2] - B[2][2],
                A[2][3] - B[2][3],
            ],
            [
                A[3][0] - B[3][0],
                A[3][1] - B[3][1],
                A[3][2] - B[3][2],
                A[3][3] - B[3][3],
            ],
        ])
    }
}

impl Mul for Matrix {
    type Output = Self;

    /// NOTE: When multiplying matrices... the order matters!
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        #[allow(non_snake_case)]
        let (A, B) = (self, rhs);
        Self([
            [
                A[0][0] * B[0][0] + A[1][0] * B[0][1] + A[2][0] * B[0][2] + A[3][0] * B[0][3],
                A[0][0] * B[1][0] + A[1][0] * B[1][1] + A[2][0] * B[1][2] + A[3][0] * B[1][3],
                A[0][0] * B[2][0] + A[1][0] * B[2][1] + A[2][0] * B[2][2] + A[3][0] * B[2][3],
                A[0][0] * B[3][0] + A[1][0] * B[3][1] + A[2][0] * B[3][2] + A[3][0] * B[3][3],
            ], [
                A[0][1] * B[0][0] + A[1][1] * B[0][1] + A[2][1] * B[0][2] + A[3][1] * B[0][3],
                A[0][1] * B[1][0] + A[1][1] * B[1][1] + A[2][1] * B[1][2] + A[3][1] * B[1][3],
                A[0][1] * B[2][0] + A[1][1] * B[2][1] + A[2][1] * B[2][2] + A[3][1] * B[2][3],
                A[0][1] * B[3][0] + A[1][1] * B[3][1] + A[2][1] * B[3][2] + A[3][1] * B[3][3],
            ], [
                A[0][2] * B[0][0] + A[1][2] * B[0][1] + A[2][2] * B[0][2] + A[3][2] * B[0][3],
                A[0][2] * B[1][0] + A[1][2] * B[1][1] + A[2][2] * B[1][2] + A[3][2] * B[1][3],
                A[0][2] * B[2][0] + A[1][2] * B[2][1] + A[2][2] * B[2][2] + A[3][2] * B[2][3],
                A[0][2] * B[3][0] + A[1][2] * B[3][1] + A[2][2] * B[3][2] + A[3][2] * B[3][3],
            ], [
                A[0][3] * B[0][0] + A[1][3] * B[0][1] + A[2][3] * B[0][2] + A[3][3] * B[0][3],
                A[0][3] * B[1][0] + A[1][3] * B[1][1] + A[2][3] * B[1][2] + A[3][3] * B[1][3],
                A[0][3] * B[2][0] + A[1][3] * B[2][1] + A[2][3] * B[2][2] + A[3][3] * B[2][3],
                A[0][3] * B[3][0] + A[1][3] * B[3][1] + A[2][3] * B[3][2] + A[3][3] * B[3][3],
            ],
        ])
    }
}

impl From<Matrix> for [f32; 16] {
    #[inline]
    fn from(value: Matrix) -> Self {
        [
            value[0][0],
            value[1][0],
            value[2][0],
            value[3][0],

            value[0][1],
            value[1][1],
            value[2][1],
            value[3][1],

            value[0][2],
            value[1][2],
            value[2][2],
            value[3][2],

            value[0][3],
            value[1][3],
            value[2][3],
            value[3][3],
        ]
    }
}
