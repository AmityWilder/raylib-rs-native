use crate::prelude::*;

pub mod mesh;
pub mod material;
pub mod animation;

pub struct BoundingBox {
    pub min: Position3,
    pub max: Position3,
}
