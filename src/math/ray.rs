use crate::prelude::*;

pub struct Ray {
    pub position: Vector3,
    pub direction: Normalized<Vector3>,
}

pub struct RayCollision {
    pub is_hit: bool,
    pub distance: Units,
    pub point: Vector3,
    pub normal: Normalized<Vector3>,
}
