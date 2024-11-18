use crate::prelude::*;

/// `GlyphInfo`, font characters glyphs info
pub struct GlyphInfo {
    /// Character value (Unicode)
    pub value: char,
    /// Character offset X when drawing
    pub offset_x: i32,
    /// Character offset Y when drawing
    pub offset_y: i32,
    /// Character advance position X
    pub advance_x: i32,
    /// Character image data
    pub image: Image,
}

/// Font, font texture and `GlyphInfo` array data
pub struct Font {
    /// Base size (default chars height)
    pub base_size: i32,
    /// Padding around the glyph characters
    pub glyph_padding: i32,
    /// Texture atlas containing the glyphs
    pub texture: Texture2D,
    // Glyphs info & rectangles in texture for the glyphs
    pub glyphs_recs: Vec<(GlyphInfo, Rectangle)>,
}
