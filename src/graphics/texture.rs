use crate::prelude::*;
use super::GlTextureID;

/// Texture, tex data stored in GPU memory (VRAM)
pub struct Texture {
    /// OpenGL texture id
    pub id: GlTextureID,
    /// Texture base width
    pub width: usize,
    /// Texture base height
    pub height: usize,
    /// Mipmap levels, 1 by default
    pub mipmap: usize,
    // Data format
    pub format: PixelFormat,
}

pub type Texture2D = Texture;
pub type TextureCubemap = Texture;
