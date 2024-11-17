use crate::math::vector::Vector3;

pub enum CameraProjection {
    Perspective,
    Orthographic,
}

pub struct Camera3D {
    pub position: Vector3,
    /// Camera target it looks-at
    pub target: Vector3,
    /// Camera up vector (rotation over its axis)
    pub up: Vector3,
    /// Camera field-of-view aperture in Y (degrees) in perspective, used as near plane width in orthographic
    pub fovy: f32,
    pub projection: CameraProjection,
}

pub type Camera = Camera3D;
