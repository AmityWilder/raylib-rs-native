use crate::prelude::*;

/// Color, 4 components, R8G8B8A8 (32bit)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[must_use]
pub struct Color {
    /// Color red value
    pub r: u8,
    /// Color green value
    pub g: u8,
    /// Color blue value
    pub b: u8,
    /// Color alpha value
    pub a: u8,
}

/// Constructs an opaque RGB color
#[inline]
pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color { r, g, b, a: 255 }
}

/// Constructs a transparent RGBA color
#[inline]
pub fn rgba(r: u8, g: u8, b: u8, a: f32) -> Color {
    rgb(r, g, b).alpha(a)
}

/// Get a Color from HSV values, hue [0..360], saturation/value [0..1]
#[inline]
pub fn hsv(hue: Degrees, saturation: Percent, value: Percent) -> Color {
    todo!()
}

impl Color {
    pub const LIGHTGRAY: Self = rgb(200, 200, 200); // Light Gray
    pub const GRAY:      Self = rgb(130, 130, 130); // Gray
    pub const DARKGRAY:  Self = rgb( 80,  80,  80); // Dark Gray
    pub const YELLOW:    Self = rgb(253, 249,   0); // Yellow
    pub const GOLD:      Self = rgb(255, 203,   0); // Gold
    pub const ORANGE:    Self = rgb(255, 161,   0); // Orange
    pub const PINK:      Self = rgb(255, 109, 194); // Pink
    pub const RED:       Self = rgb(230,  41,  55); // Red
    pub const MAROON:    Self = rgb(190,  33,  55); // Maroon
    pub const GREEN:     Self = rgb(  0, 228,  48); // Green
    pub const LIME:      Self = rgb(  0, 158,  47); // Lime
    pub const DARKGREEN: Self = rgb(  0, 117,  44); // Dark Green
    pub const SKYBLUE:   Self = rgb(102, 191, 255); // Sky Blue
    pub const BLUE:      Self = rgb(  0, 121, 241); // Blue
    pub const DARKBLUE:  Self = rgb(  0,  82, 172); // Dark Blue
    pub const PURPLE:    Self = rgb(200, 122, 255); // Purple
    pub const VIOLET:    Self = rgb(135,  60, 190); // Violet
    pub const DARKPURPLE:Self = rgb(112,  31, 126); // Dark Purple
    pub const BEIGE:     Self = rgb(211, 176, 131); // Beige
    pub const BROWN:     Self = rgb(127, 106,  79); // Brown
    pub const DARKBROWN: Self = rgb( 76,  63,  47); // Dark Brown

    pub const WHITE:     Self = rgb(255, 255, 255); // White
    pub const BLACK:     Self = rgb(  0,   0,   0); // Black
    pub const MAGENTA:   Self = rgb(255,   0, 255); // Magenta
    pub const RAYWHITE:  Self = rgb(245, 245, 245); // My own White (raylib logo)

    pub const BLANK:     Self = Self::new(0, 0, 0, 0); // Blank (Transparent)

    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Get HSV values for a Color, hue [0..360], saturation/value [0..1]
    pub fn to_hsv(self) -> (Degrees, f32, f32) {
        todo!()
    }

    pub fn alpha(self, a: Percent) -> Self {
        todo!()
    }
}

impl LerpTo for Color {
    fn lerp_to(self, target: Self, amount: Percent) -> Self {
        todo!()
    }
}
