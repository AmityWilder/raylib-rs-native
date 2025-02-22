/**********************************************************************************************
*
*   raylib configuration flags
*
*   This file defines all the configuration flags for the different raylib modules
*
*   LICENSE: zlib/libpng
*
*   Copyright (c) 2018-2024 Ahmad Fatoum & Ramon Santamaria (@raysan5)
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

//------------------------------------------------------------------------------------
// Module selection - Some modules could be avoided
// Mandatory modules: rcore, rlgl, utils
//------------------------------------------------------------------------------------

// rcore: Configuration values
//------------------------------------------------------------------------------------
/// Maximum file paths capacity
pub const MAX_FILEPATH_CAPACITY: usize = 8192;
/// Maximum length for filepaths (Linux PATH_MAX default value)
pub const MAX_FILEPATH_LENGTH: usize = 4096;

/// Maximum number of keyboard keys supported
pub const MAX_KEYBOARD_KEYS: usize = 512;
/// Maximum number of mouse buttons supported
pub const MAX_MOUSE_BUTTONS: usize = 8;
/// Maximum number of gamepads supported
pub const MAX_GAMEPADS: usize = 4;
/// Maximum number of axis supported (per gamepad)
pub const MAX_GAMEPAD_AXIS: usize = 8;
/// Maximum number of buttons supported (per gamepad)
pub const MAX_GAMEPAD_BUTTONS: usize = 32;
/// Maximum vibration time in seconds
pub const MAX_GAMEPAD_VIBRATION_TIME: f32 = 2.0;
/// Maximum number of touch points supported
pub const MAX_TOUCH_POINTS: usize = 8;
/// Maximum number of keys in the key input queue
pub const MAX_KEY_PRESSED_QUEUE: usize = 16;
/// Maximum number of characters in the char input queue
pub const MAX_CHAR_PRESSED_QUEUE: usize = 16;

/// Max size allocated for decompression in MB
pub const MAX_DECOMPRESSION_SIZE: usize = 64;

/// Maximum number of automation events to record
pub const MAX_AUTOMATION_EVENTS: usize = 16384;

//------------------------------------------------------------------------------------
// Module: rlgl - Configuration values
//------------------------------------------------------------------------------------

/// Default internal render batch elements limits
pub const RL_DEFAULT_BATCH_BUFFER_ELEMENTS: usize = 4096;
/// Default number of batch buffers (multi-buffering)
pub const RL_DEFAULT_BATCH_BUFFERS: usize = 1;
/// Default number of batch draw calls (by state changes: mode, texture)
pub const RL_DEFAULT_BATCH_DRAWCALLS: usize = 256;
/// Maximum number of textures units that can be activated on batch drawing (SetShaderValueTexture())
pub const RL_DEFAULT_BATCH_MAX_TEXTURE_UNITS: usize = 4;

/// Maximum size of internal Matrix stack
pub const RL_MAX_MATRIX_STACK_SIZE: usize = 32;

/// Maximum number of shader locations supported
pub const RL_MAX_SHADER_LOCATIONS: usize = 32;

/// Default projection matrix near cull distance
pub const RL_CULL_DISTANCE_NEAR: f32 = 0.01;
/// Default projection matrix far cull distance
pub const RL_CULL_DISTANCE_FAR: f32 = 1000.0;

// Default shader vertex attribute locations
pub const RL_DEFAULT_SHADER_ATTRIB_LOCATION_POSITION: usize = 0;
pub const RL_DEFAULT_SHADER_ATTRIB_LOCATION_TEXCOORD: usize = 1;
pub const RL_DEFAULT_SHADER_ATTRIB_LOCATION_NORMAL: usize = 2;
pub const RL_DEFAULT_SHADER_ATTRIB_LOCATION_COLOR: usize = 3;
pub const RL_DEFAULT_SHADER_ATTRIB_LOCATION_TANGENT: usize = 4;
pub const RL_DEFAULT_SHADER_ATTRIB_LOCATION_TEXCOORD2: usize = 5;
pub const RL_DEFAULT_SHADER_ATTRIB_LOCATION_INDICES: usize = 6;
#[cfg(feature = "support_mesh_gpu_skinning")]
pub const RL_DEFAULT_SHADER_ATTRIB_LOCATION_BONEIDS: usize = 7;
#[cfg(feature = "support_mesh_gpu_skinning")]
pub const RL_DEFAULT_SHADER_ATTRIB_LOCATION_BONEWEIGHTS: usize = 8;

// Default shader vertex attribute names to set location points
// NOTE: When a new shader is loaded, the following locations are tried to be set for convenience

/// Bound by default to shader location: RL_DEFAULT_SHADER_ATTRIB_LOCATION_POSITION
pub const RL_DEFAULT_SHADER_ATTRIB_NAME_POSITION: &'static str = "vertexPosition";
/// Bound by default to shader location: RL_DEFAULT_SHADER_ATTRIB_LOCATION_TEXCOORD
pub const RL_DEFAULT_SHADER_ATTRIB_NAME_TEXCOORD: &'static str = "vertexTexCoord";
/// Bound by default to shader location: RL_DEFAULT_SHADER_ATTRIB_LOCATION_NORMAL
pub const RL_DEFAULT_SHADER_ATTRIB_NAME_NORMAL: &'static str = "vertexNormal";
/// Bound by default to shader location: RL_DEFAULT_SHADER_ATTRIB_LOCATION_COLOR
pub const RL_DEFAULT_SHADER_ATTRIB_NAME_COLOR: &'static str = "vertexColor";
/// Bound by default to shader location: RL_DEFAULT_SHADER_ATTRIB_LOCATION_TANGENT
pub const RL_DEFAULT_SHADER_ATTRIB_NAME_TANGENT: &'static str = "vertexTangent";
/// Bound by default to shader location: RL_DEFAULT_SHADER_ATTRIB_LOCATION_TEXCOORD2
pub const RL_DEFAULT_SHADER_ATTRIB_NAME_TEXCOORD2: &'static str = "vertexTexCoord2";

/// model-view-projection matrix
pub const RL_DEFAULT_SHADER_UNIFORM_NAME_MVP: &'static str = "mvp";
/// view matrix
pub const RL_DEFAULT_SHADER_UNIFORM_NAME_VIEW: &'static str = "matView";
/// projection matrix
pub const RL_DEFAULT_SHADER_UNIFORM_NAME_PROJECTION: &'static str = "matProjection";
/// model matrix
pub const RL_DEFAULT_SHADER_UNIFORM_NAME_MODEL: &'static str = "matModel";
/// normal matrix (transpose(inverse(matModelView))
pub const RL_DEFAULT_SHADER_UNIFORM_NAME_NORMAL: &'static str = "matNormal";
/// color diffuse (base tint color, multiplied by texture color)
pub const RL_DEFAULT_SHADER_UNIFORM_NAME_COLOR: &'static str = "colDiffuse";
/// texture0 (texture slot active 0)
pub const RL_DEFAULT_SHADER_SAMPLER2D_NAME_TEXTURE0: &'static str = "texture0";
/// texture1 (texture slot active 1)
pub const RL_DEFAULT_SHADER_SAMPLER2D_NAME_TEXTURE1: &'static str = "texture1";
/// texture2 (texture slot active 2)
pub const RL_DEFAULT_SHADER_SAMPLER2D_NAME_TEXTURE2: &'static str = "texture2";


//------------------------------------------------------------------------------------
// Module: rshapes - Configuration Flags
//------------------------------------------------------------------------------------

// rshapes: Configuration values
//------------------------------------------------------------------------------------
/// Spline segments subdivisions
pub const SPLINE_SEGMENT_DIVISIONS: usize = 24;

//------------------------------------------------------------------------------------
// Module: rtextures - Configuration Flags
//------------------------------------------------------------------------------------

//------------------------------------------------------------------------------------
// Module: rtext - Configuration Flags
//------------------------------------------------------------------------------------

// rtext: Configuration values
//------------------------------------------------------------------------------------
/// Size of internal static buffers used on some functions:
/// TextFormat(), TextSubtext(), TextToUpper(), TextToLower(), TextToPascal(), TextSplit()
pub const MAX_TEXT_BUFFER_LENGTH: usize = 1024;
/// Maximum number of substrings to split: TextSplit()
pub const MAX_TEXTSPLIT_COUNT: usize = 128;

//------------------------------------------------------------------------------------
// Module: rmodels - Configuration Flags
//------------------------------------------------------------------------------------

// rmodels: Configuration values
//------------------------------------------------------------------------------------
/// Maximum number of shader maps supported
pub const MAX_MATERIAL_MAPS: usize = 12;

/// Maximum vertex buffers (VBO) per mesh
pub const MAX_MESH_VERTEX_BUFFERS: usize = if cfg!(feature = "support_mesh_gpu_skinning") { 9 } else { 7 };

//------------------------------------------------------------------------------------
// Module: raudio - Configuration Flags
//------------------------------------------------------------------------------------

// raudio: Configuration values
//------------------------------------------------------------------------------------
/// Device output format (miniaudio: float-32bit)
// pub const AUDIO_DEVICE_FORMAT: ma_format = ma_format::f32; // TODO
/// Device output channels: stereo
pub const AUDIO_DEVICE_CHANNELS: usize = 2;
/// Device sample rate (device default)
pub const AUDIO_DEVICE_SAMPLE_RATE: usize = 0;

/// Maximum number of audio pool channels
pub const MAX_AUDIO_BUFFER_POOL_CHANNELS: usize = 16;

//------------------------------------------------------------------------------------
// Module: utils - Configuration Flags
//------------------------------------------------------------------------------------

// utils: Configuration values
//------------------------------------------------------------------------------------
/// Max length of one trace-log message
pub const MAX_TRACELOG_MSG_LENGTH: usize = 256;

pub const STBI_REQUIRED: bool = cfg!(feature = "support_clipboard_image");
