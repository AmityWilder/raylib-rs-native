/// Pixel formats
/// NOTE: Support depends on OpenGL version and platform
pub enum PixelFormat {
    /** 8 bit per pixel (no alpha)         */ UncompressedGrayscale = 1,
    /** 8*2 bpp (2 channels)               */ UncompressedGrayAlpha,
    /** 16 bpp                             */ UncompressedR5G6B5,
    /** 24 bpp                             */ UncompressedR8G8B8,
    /** 16 bpp (1 bit alpha)               */ UncompressedR5G5B5A1,
    /** 16 bpp (4 bit alpha)               */ UncompressedR4G4B4A4,
    /** 32 bpp                             */ UncompressedR8G8B8A8,
    /** 32 bpp (1 channel - float)         */ UncompressedR32,
    /** 32*3 bpp (3 channels - float)      */ UncompressedR32G32A32,
    /** 32*4 bpp (4 channels - float)      */ UncompressedR32G32A32A32,
    /** 16 bpp (1 channel - half float)    */ UncompressedR16,
    /** 16*3 bpp (3 channels - half float) */ UncompressedR16G16B16,
    /** 16*4 bpp (4 channels - half float) */ UncompressedR16G16B16A16,
    /** 4 bpp (no alpha)                   */ CompressedDxt1RGB,
    /** 4 bpp (1 bit alpha)                */ CompressedDxt1RGBA,
    /** 8 bpp                              */ CompressedDxt3RGBA,
    /** 8 bpp                              */ CompressedDxt5RGBA,
    /** 4 bpp                              */ CompressedEtc1RGB,
    /** 4 bpp                              */ CompressedEtc2RGB,
    /** 8 bpp                              */ CompressedEtc2EacRGBA,
    /** 4 bpp                              */ CompressedPvrtRGB,
    /** 4 bpp                              */ CompressedPvrtRGBA,
    /** 8 bpp                              */ CompressedAstc4x4RGBA,
    /** 2 bpp                              */ CompressedAstc8x8RGBA,
}

// Texture parameters: filter mode
// NOTE 1: Filtering considers mipmaps if available in the texture
// NOTE 2: Filter is accordingly set for minification and magnification
pub enum TextureFilter {
    /** No filter, just pixel approximation       */ Point,
    /** Linear filtering                          */ Bilinear,
    /** Trilinear filtering (linear with mipmaps) */ Trilinear,
    /** Anisotropic filtering 4x                  */ Anisotropic4x,
    /** Anisotropic filtering 8x                  */ Anisotropic8x,
    /** Anisotropic filtering 16x                 */ Anisotropic16x,
}

// Texture parameters: wrap mode
pub enum TextureWrap {
    /** Repeats texture in tiled mode                          */ Repeat,
    /** Clamps texture to edge pixel in tiled mode             */ Clamp,
    /** Mirrors and repeats the texture in tiled mode          */ MirrorRepeat,
    /** Mirrors and clamps to border the texture in tiled mode */ MirrorClamp,
}

// Cubemap layouts
pub enum CubemapLayout {
    /** Automatically detect layout type                    */ AutoDetect,
    /** Layout is defined by a vertical line with faces     */ LineVertical,
    /** Layout is defined by a horizontal line with faces   */ LineHorizontal,
    /** Layout is defined by a 3x4 cross with cubemap faces */ CrossThreeByFour,
    /** Layout is defined by a 4x3 cross with cubemap faces */ CrossFourByThree,
}
