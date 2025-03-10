/*!********************************************************************************************
*
*   raylib v5.5 - A simple and easy-to-use library to enjoy videogames programming (www.raylib.com)
*
*   FEATURES:
*       - NO external dependencies, all required libraries included with raylib
*       - Multiplatform: Windows, Linux, FreeBSD, OpenBSD, NetBSD, DragonFly,
*                        MacOS, Haiku, Android, Raspberry Pi, DRM native, HTML5.
*       - Written in plain C code (C99) in PascalCase/camelCase notation
*       - Hardware accelerated with OpenGL (1.1, 2.1, 3.3, 4.3, ES2, ES3 - choose at compile)
*       - Unique OpenGL abstraction layer (usable as standalone module): [rlgl]
*       - Multiple Fonts formats supported (TTF, OTF, FNT, BDF, Sprite fonts)
*       - Outstanding texture formats support, including compressed formats (DXT, ETC, ASTC)
*       - Full 3d support for 3d Shapes, Models, Billboards, Heightmaps and more!
*       - Flexible Materials system, supporting classic maps and PBR maps
*       - Animated 3D models supported (skeletal bones animation) (IQM, M3D, GLTF)
*       - Shaders support, including Model shaders and Postprocessing shaders
*       - Powerful math module for Vector, Matrix and Quaternion operations: [raymath]
*       - Audio loading and playing with streaming support (WAV, OGG, MP3, FLAC, QOA, XM, MOD)
*       - VR stereo rendering with configurable HMD device parameters
*       - Bindings to multiple programming languages available!
*
*   NOTES:
*       - One default Font is loaded on InitWindow()->LoadFontDefault() [core, text]
*       - One default Texture2D is loaded on rlglInit(), 1x1 white pixel R8G8B8A8 [rlgl] (OpenGL 3.3 or ES2)
*       - One default Shader is loaded on rlglInit()->rlLoadShaderDefault() [rlgl] (OpenGL 3.3 or ES2)
*       - One default RenderBatch is loaded on rlglInit()->rlLoadRenderBatch() [rlgl] (OpenGL 3.3 or ES2)
*
*   DEPENDENCIES (included):
*       [rcore][GLFW] rglfw (Camilla Löwy - github.com/glfw/glfw) for window/context management and input
*       [rcore][RGFW] rgfw (ColleagueRiley - github.com/ColleagueRiley/RGFW) for window/context management and input
*       [rlgl] glad/glad_gles2 (David Herberth - github.com/Dav1dde/glad) for OpenGL 3.3 extensions loading
*       [raudio] miniaudio (David Reid - github.com/mackron/miniaudio) for audio device/context management
*
*   OPTIONAL DEPENDENCIES (included):
*       [rcore] msf_gif (Miles Fogle) for GIF recording
*       [rcore] sinfl (Micha Mettke) for DEFLATE decompression algorithm
*       [rcore] sdefl (Micha Mettke) for DEFLATE compression algorithm
*       [rcore] rprand (Ramon Snatamaria) for pseudo-random numbers generation
*       [rtextures] qoi (Dominic Szablewski - https://phoboslab.org) for QOI image manage
*       [rtextures] stb_image (Sean Barret) for images loading (BMP, TGA, PNG, JPEG, HDR...)
*       [rtextures] stb_image_write (Sean Barret) for image writing (BMP, TGA, PNG, JPG)
*       [rtextures] stb_image_resize2 (Sean Barret) for image resizing algorithms
*       [rtextures] stb_perlin (Sean Barret) for Perlin Noise image generation
*       [rtext] stb_truetype (Sean Barret) for ttf fonts loading
*       [rtext] stb_rect_pack (Sean Barret) for rectangles packing
*       [rmodels] par_shapes (Philip Rideout) for parametric 3d shapes generation
*       [rmodels] tinyobj_loader_c (Syoyo Fujita) for models loading (OBJ, MTL)
*       [rmodels] cgltf (Johannes Kuhlmann) for models loading (glTF)
*       [rmodels] m3d (bzt) for models loading (M3D, https://bztsrc.gitlab.io/model3d)
*       [rmodels] vox_loader (Johann Nadalutti) for models loading (VOX)
*       [raudio] dr_wav (David Reid) for WAV audio file loading
*       [raudio] dr_flac (David Reid) for FLAC audio file loading
*       [raudio] dr_mp3 (David Reid) for MP3 audio file loading
*       [raudio] stb_vorbis (Sean Barret) for OGG audio loading
*       [raudio] jar_xm (Joshua Reisenauer) for XM audio module loading
*       [raudio] jar_mod (Joshua Reisenauer) for MOD audio module loading
*       [raudio] qoa (Dominic Szablewski - https://phoboslab.org) for QOA audio manage
*
*
*   LICENSE: zlib/libpng
*
*   raylib is licensed under an unmodified zlib/libpng license, which is an OSI-certified,
*   BSD-like license that allows static linking with closed source software:
*
*   Copyright (c) 2013-2024 Ramon Santamaria (@raysan5)
*
*   This software is provided "as-is", without any express or implied warranty. In no event
*   will the authors be held liable for any damages arising from the use of this software.
*
*   Permission is granted to anyone to use this software for any purpose, including commercial
*   applications, and to alter it and redistribute it freely, subject to the following restrictions:
*
*     1. The origin of this software must not be misrepresented; you must not claim that you
*     wrote the original software. If you use this software in a product, an acknowledgment
*     in the product documentation would be appreciated but is not required.
*
*     2. Altered source versions must be plainly marked as such, and must not be misrepresented
*     as being the original software.
*
*     3. This notice may not be removed or altered from any source distribution.
*
**********************************************************************************************/

//! This source code has been heavily altered by Amy Wilder.

#![warn(
    clippy::pedantic,
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::cast_possible_truncation,
)]
#![deny(
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
)]
#![forbid(unsafe_code)]

pub const RAYLIB_VERSION: &str = "5.5";

pub mod config;
mod external;
mod platforms;
pub mod core;
pub mod rlgl;
pub mod utils;
pub mod color;
pub mod math;
pub mod shapes;
pub mod graphics;
pub mod audio;

pub use platforms::rcore_desktop_sdl::*;

pub mod prelude {
    pub use super::{
        core::{
            *,
            window::*,
            input::*,
        },
        utils::*,
        color::*,
        math::{
            *,
            indicators::*,
            matrix::*,
            quaternion::*,
            ray::*,
            transform::*,
            vector::*,
        },
        graphics::{
            *,
            model::{
                *,
                animation::*,
                material::*,
                mesh::*,
            },
            drawing::{
                *,
            },
            camera::*,
            font::*,
            image::*,
            pixel_format::*,
            render_texture::*,
            shader::*,
            texture::*,
        },
        shapes::{
            *,
            circle::*,
            rectangle::*,
            triangle::*,
        },
    };
}

/// Trace log level
/// NOTE: Organized by priority level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum TraceLogType {
    /// Trace logging, intended for internal use only
    Trace = 1,
    /// Debug logging, used for internal debugging, it should be disabled on release builds
    Debug,
    /// Info logging, used for program execution info
    #[default]
    Info,
    /// Warning logging, used on recoverable failures
    Warning,
    /// Error logging, used on unrecoverable failures
    Error,
    /// Fatal logging, used to abort program: exit(EXIT_FAILURE)
    Fatal,
}

impl TryFrom<u8> for TraceLogType {
    type Error = std::num::TryFromIntError;
    fn try_from(value: u8) -> Result<Self, std::num::TryFromIntError> {
        match value {
            1 => Ok(Self::Trace),
            2 => Ok(Self::Debug),
            3 => Ok(Self::Info),
            4 => Ok(Self::Warning),
            5 => Ok(Self::Error),
            6 => Ok(Self::Fatal),
            0 | 7.. => Err(u8::try_from(256u16).unwrap_err()),
        }
    }
}

/// Trace log level
/// NOTE: Organized by priority level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum TraceLogLevel {
    /// Display all logs
    All,
    /// Trace logging, intended for internal use only
    Trace,
    /// Debug logging, used for internal debugging, it should be disabled on release builds
    Debug,
    /// Info logging, used for program execution info
    #[default]
    Info,
    /// Warning logging, used on recoverable failures
    Warning,
    /// Error logging, used on unrecoverable failures
    Error,
    /// Fatal logging, used to abort program: exit(EXIT_FAILURE)
    Fatal,
    /// Disable logging
    None,
}

impl TryFrom<u8> for TraceLogLevel {
    type Error = std::num::TryFromIntError;
    fn try_from(value: u8) -> Result<Self, std::num::TryFromIntError> {
        match value {
            0 => Ok(Self::All),
            1 => Ok(Self::Trace),
            2 => Ok(Self::Debug),
            3 => Ok(Self::Info),
            4 => Ok(Self::Warning),
            5 => Ok(Self::Error),
            6 => Ok(Self::Fatal),
            7 => Ok(Self::None),
            8.. => Err(u8::try_from(256u16).unwrap_err()),
        }
    }
}

impl From<TraceLogType> for TraceLogLevel {
    fn from(value: TraceLogType) -> Self {
        match value {
            TraceLogType::Trace   => Self::Trace,
            TraceLogType::Debug   => Self::Debug,
            TraceLogType::Info    => Self::Info,
            TraceLogType::Warning => Self::Warning,
            TraceLogType::Error   => Self::Error,
            TraceLogType::Fatal   => Self::Fatal,
        }
    }
}

impl PartialEq<TraceLogType> for TraceLogLevel {
    fn eq(&self, other: &TraceLogType) -> bool {
        (*self as u8).eq(&(*other as u8))
    }
}

impl PartialEq<TraceLogLevel> for TraceLogType {
    fn eq(&self, other: &TraceLogLevel) -> bool {
        (*self as u8).eq(&(*other as u8))
    }
}

impl PartialOrd<TraceLogType> for TraceLogLevel {
    fn partial_cmp(&self, other: &TraceLogType) -> Option<std::cmp::Ordering> {
        (*self as u8).partial_cmp(&(*other as u8))
    }
}

impl PartialOrd<TraceLogLevel> for TraceLogType {
    fn partial_cmp(&self, other: &TraceLogLevel) -> Option<std::cmp::Ordering> {
        (*self as u8).partial_cmp(&(*other as u8))
    }
}
