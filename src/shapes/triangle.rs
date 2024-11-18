use crate::prelude::*;

pub struct Triangle2D {
    pub points: [Position2; 3],
}

pub struct Triangle3D {
    pub points: [Position3; 3],
}

pub type Triangle = Triangle3D;
