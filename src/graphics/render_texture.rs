use crate::prelude::*;
use super::GlFrameBufferID;

/// `RenderTexture`, fbo for texture rendering
pub struct RenderTexture {
    /// OpenGL framebuffer object id
    pub id: GlFrameBufferID,
    /// Color buffer attachment texture
    pub texture: Texture,
    /// Depth buffer attachment texture
    pub depth: Texture,
}

/// `RenderTexture2D`, same as `RenderTexture`
pub type RenderTexture2D = RenderTexture;
