//! Helpers for communicating the expected usage of multipurpose/multistate types

use crate::prelude::*;

/// Indicates the value is expected as a ratio of x units `T` per y units `U`
///
/// No conversion is required if `U` is a unit type
pub struct Ratio<T, U>(pub T, pub U);

impl<T: std::ops::Div<U, Output = T>, U> Ratio<T, U> {
    /// Calculates the change in `T` per one unit of `U`
    pub fn per_unit(self) -> T {
        self.0 / self.1
    }
}

/// Indicates the value is expected as an angle in radians `[0..2pi]`
pub type Radians = f32;

/// Indicates the ratio is x units per radian
pub struct Radian;

/// Indicates the value is expected as an angle in degrees `[0..360]`
pub type Degrees = f32;

/// Indicates the ratio is x units per degree
pub struct Degree;

/// Indicates the value is expected as a percentage `[0..1]`
pub type Percent = f32;

/// Indicates the value is expected as a duration of time in seconds
pub type Seconds = f32;

/// Indicates the ratio is x units per second
pub struct Second;

/// Indicates the value is expected as a measurement in pixels
pub type Pixels = f32;

/// Indicates the ratio is x units per pixel
pub struct Pixel;

/// Indicates the value is expected as a measurement in raylib distance units
pub type Units = f32;

/// Indicates the ratio is x units per raylib distance unit
pub struct Unit;

/// Indicates that the value is expected to be normalized
pub type Normalized<T> = T;

/// Indicates the vector is expected to be a position in 2D space (usually pixels)
pub type Position2 = Vector2;
/// Indicates the vector is expected to be a relative position in 2D space (usually pixels)
pub type Offset2 = Vector2;
/// Indicates the vector is expected to be a normalized direction in 2D space
pub type Direction2 = Normalized<Vector2>;

/// Indicates the vector is expected to be a position in 3D space
pub type Position3 = Vector3;
/// Indicates the vector is expected to be a relative position in 3D space
pub type Offset3 = Vector3;
/// Indicates the vector is expected to be a normalized direction in 3D space
pub type Direction3 = Normalized<Vector3>;

/// Indicates the vector is expected to be a position in 4D space
pub type Position4 = Vector4;
/// Indicates the vector is expected to be a relative position in 4D space
pub type Offset4 = Vector4;
/// Indicates the vector is expected to be a normalized direction in 4D space
pub type Direction4 = Normalized<Vector4>;
