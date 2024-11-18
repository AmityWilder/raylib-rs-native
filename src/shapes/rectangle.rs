use crate::prelude::*;

/// Rectangle, 4 components
pub struct Rectangle {
    /// Rectangle top-left corner position x
    pub x: f32,
    /// Rectangle top-left corner position y
    pub y: f32,
    /// Rectangle width
    pub width: f32,
    /// Rectangle height
    pub height: f32,
}

impl Rectangle {
    #[inline]
    #[must_use]
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    #[inline]
    #[must_use]
    pub const fn x_min(&self) -> f32 {
        self.x
    }

    #[inline]
    #[must_use]
    pub fn x_max(&self) -> f32 {
        self.x + self.width
    }

    #[inline]
    #[must_use]
    pub const fn y_min(&self) -> f32 {
        self.y
    }

    #[inline]
    #[must_use]
    pub fn y_max(&self) -> f32 {
        self.y + self.height
    }

    #[inline]
    #[must_use]
    pub fn center_x(&self) -> f32 {
        self.x + self.width * 0.5
    }

    #[inline]
    #[must_use]
    pub fn center_y(&self) -> f32 {
        self.y + self.height * 0.5
    }

    #[inline]
    pub fn center(&self) -> Position2 {
        Position2 {
            x: self.center_x(),
            y: self.center_y(),
        }
    }
}
