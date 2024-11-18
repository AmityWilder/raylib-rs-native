use crate::prelude::*;

/// Image, pixel data stored in CPU memory (RAM)
pub struct Image {
    /// Image raw data
    pub data: Vec<u8>,
    /// Image base width
    pub width: usize,
    /// Image base height
    pub height: usize,
    /// Mipmap levels, 1 by default
    pub mipmap: usize,
    /// Data format
    pub format: PixelFormat,
}
