/*!********************************************************************************************
*
*   rcore_desktop_sdl - Functions to manage window, graphics device and inputs
*
*   PLATFORM: DESKTOP: SDL
*       - Windows (Win32, Win64)
*       - Linux (X11/Wayland desktop mode)
*       - Others (not tested)
*
*   LIMITATIONS:
*       - Limitation 01
*       - Limitation 02
*
*   POSSIBLE IMPROVEMENTS:
*       - Improvement 01
*       - Improvement 02
*
*   ADDITIONAL NOTES:
*       - TRACELOG() function is located in raylib [utils] module
*
*   CONFIGURATION:
*       #define RCORE_PLATFORM_CUSTOM_FLAG
*           Custom flag for rcore on target platform -not used-
*
*   DEPENDENCIES:
*       - SDL 2 or SLD 3 (main library): Windowing and inputs management
*       - gestures: Gestures system for touch-ready devices (or simulated from mouse inputs)
*
*
*   LICENSE: zlib/libpng
*
*   Copyright (c) 2013-2024 Ramon Santamaria (@raysan5) and contributors
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

use std::num::TryFromIntError;
use sdl3::{gamepad::Gamepad as SdlGamepad, mouse::{Cursor as SdlCursor, SystemCursor}, video::{GLContext, Window as SdlWindow, WindowBuildError}, Error as SdlError, IntegerOrSdlError, Sdl, VideoSubsystem};
use crate::{config::MAX_GAMEPADS, prelude::{ConfigFlags, Core, GamepadAxis, Image, KeyboardKey, Vector2}, tracelog};

/// Size of the clipboard buffer used on GetClipboardText()
pub const MAX_CLIPBOARD_BUFFER_LENGTH: usize = 1024;

/// Platform specific data
pub struct Platform {
    sdl_context: Sdl,

    video_subsystem: VideoSubsystem,
    window: SdlWindow,
    gl_context: GLContext,

    gamepad: [Option<SdlGamepad>; MAX_GAMEPADS],
    cursor: Option<SdlCursor>,
    cursor_relative: bool,
}

pub const SCANCODE_MAPPED_NUM: usize = 232;
static MAP_SCANCODE_TO_KEY: [Option<KeyboardKey>; SCANCODE_MAPPED_NUM] = [
    None,                             // SDL_SCANCODE_UNKNOWN
    None,
    None,
    None,
    Some(KeyboardKey::A),             // SDL_SCANCODE_A
    Some(KeyboardKey::B),             // SDL_SCANCODE_B
    Some(KeyboardKey::C),             // SDL_SCANCODE_C
    Some(KeyboardKey::D),             // SDL_SCANCODE_D
    Some(KeyboardKey::E),             // SDL_SCANCODE_E
    Some(KeyboardKey::F),             // SDL_SCANCODE_F
    Some(KeyboardKey::G),             // SDL_SCANCODE_G
    Some(KeyboardKey::H),             // SDL_SCANCODE_H
    Some(KeyboardKey::I),             // SDL_SCANCODE_I
    Some(KeyboardKey::J),             // SDL_SCANCODE_J
    Some(KeyboardKey::K),             // SDL_SCANCODE_K
    Some(KeyboardKey::L),             // SDL_SCANCODE_L
    Some(KeyboardKey::M),             // SDL_SCANCODE_M
    Some(KeyboardKey::N),             // SDL_SCANCODE_N
    Some(KeyboardKey::O),             // SDL_SCANCODE_O
    Some(KeyboardKey::P),             // SDL_SCANCODE_P
    Some(KeyboardKey::Q),             // SDL_SCANCODE_Q
    Some(KeyboardKey::R),             // SDL_SCANCODE_R
    Some(KeyboardKey::S),             // SDL_SCANCODE_S
    Some(KeyboardKey::T),             // SDL_SCANCODE_T
    Some(KeyboardKey::U),             // SDL_SCANCODE_U
    Some(KeyboardKey::V),             // SDL_SCANCODE_V
    Some(KeyboardKey::W),             // SDL_SCANCODE_W
    Some(KeyboardKey::X),             // SDL_SCANCODE_X
    Some(KeyboardKey::Y),             // SDL_SCANCODE_Y
    Some(KeyboardKey::Z),             // SDL_SCANCODE_Z
    Some(KeyboardKey::One),           // SDL_SCANCODE_1
    Some(KeyboardKey::Two),           // SDL_SCANCODE_2
    Some(KeyboardKey::Three),         // SDL_SCANCODE_3
    Some(KeyboardKey::Four),          // SDL_SCANCODE_4
    Some(KeyboardKey::Five),          // SDL_SCANCODE_5
    Some(KeyboardKey::Six),           // SDL_SCANCODE_6
    Some(KeyboardKey::Seven),         // SDL_SCANCODE_7
    Some(KeyboardKey::Eight),         // SDL_SCANCODE_8
    Some(KeyboardKey::Nine),          // SDL_SCANCODE_9
    Some(KeyboardKey::Zero),          // SDL_SCANCODE_0
    Some(KeyboardKey::Enter),         // SDL_SCANCODE_RETURN
    Some(KeyboardKey::Escape),        // SDL_SCANCODE_ESCAPE
    Some(KeyboardKey::Backspace),     // SDL_SCANCODE_BACKSPACE
    Some(KeyboardKey::Tab),           // SDL_SCANCODE_TAB
    Some(KeyboardKey::Space),         // SDL_SCANCODE_SPACE
    Some(KeyboardKey::Minus),         // SDL_SCANCODE_MINUS
    Some(KeyboardKey::Equal),         // SDL_SCANCODE_EQUALS
    Some(KeyboardKey::LeftBracket),   // SDL_SCANCODE_LEFTBRACKET
    Some(KeyboardKey::RightBracket),  // SDL_SCANCODE_RIGHTBRACKET
    Some(KeyboardKey::Backslash),     // SDL_SCANCODE_BACKSLASH
    None,                             // SDL_SCANCODE_NONUSHASH
    Some(KeyboardKey::Semicolon),     // SDL_SCANCODE_SEMICOLON
    Some(KeyboardKey::Apostrophe),    // SDL_SCANCODE_APOSTROPHE
    Some(KeyboardKey::Grave),         // SDL_SCANCODE_GRAVE
    Some(KeyboardKey::Comma),         // SDL_SCANCODE_COMMA
    Some(KeyboardKey::Period),        // SDL_SCANCODE_PERIOD
    Some(KeyboardKey::Slash),         // SDL_SCANCODE_SLASH
    Some(KeyboardKey::CapsLock),      // SDL_SCANCODE_CAPSLOCK
    Some(KeyboardKey::F1),            // SDL_SCANCODE_F1
    Some(KeyboardKey::F2),            // SDL_SCANCODE_F2
    Some(KeyboardKey::F3),            // SDL_SCANCODE_F3
    Some(KeyboardKey::F4),            // SDL_SCANCODE_F4
    Some(KeyboardKey::F5),            // SDL_SCANCODE_F5
    Some(KeyboardKey::F6),            // SDL_SCANCODE_F6
    Some(KeyboardKey::F7),            // SDL_SCANCODE_F7
    Some(KeyboardKey::F8),            // SDL_SCANCODE_F8
    Some(KeyboardKey::F9),            // SDL_SCANCODE_F9
    Some(KeyboardKey::F10),           // SDL_SCANCODE_F10
    Some(KeyboardKey::F11),           // SDL_SCANCODE_F11
    Some(KeyboardKey::F12),           // SDL_SCANCODE_F12
    Some(KeyboardKey::PrintScreen),   // SDL_SCANCODE_PRINTSCREEN
    Some(KeyboardKey::ScrollLock),    // SDL_SCANCODE_SCROLLLOCK
    Some(KeyboardKey::Pause),         // SDL_SCANCODE_PAUSE
    Some(KeyboardKey::Insert),        // SDL_SCANCODE_INSERT
    Some(KeyboardKey::Home),          // SDL_SCANCODE_HOME
    Some(KeyboardKey::PageUp),        // SDL_SCANCODE_PAGEUP
    Some(KeyboardKey::Delete),        // SDL_SCANCODE_DELETE
    Some(KeyboardKey::End),           // SDL_SCANCODE_END
    Some(KeyboardKey::PageDown),      // SDL_SCANCODE_PAGEDOWN
    Some(KeyboardKey::Right),         // SDL_SCANCODE_RIGHT
    Some(KeyboardKey::Left),          // SDL_SCANCODE_LEFT
    Some(KeyboardKey::Down),          // SDL_SCANCODE_DOWN
    Some(KeyboardKey::Up),            // SDL_SCANCODE_UP
    Some(KeyboardKey::NumLock),       // SDL_SCANCODE_NUMLOCKCLEAR
    Some(KeyboardKey::KpDivide),      // SDL_SCANCODE_KP_DIVIDE
    Some(KeyboardKey::KpMultiply),    // SDL_SCANCODE_KP_MULTIPLY
    Some(KeyboardKey::KpSubtract),    // SDL_SCANCODE_KP_MINUS
    Some(KeyboardKey::KpAdd),         // SDL_SCANCODE_KP_PLUS
    Some(KeyboardKey::KpEnter),       // SDL_SCANCODE_KP_ENTER
    Some(KeyboardKey::Kp1),           // SDL_SCANCODE_KP_1
    Some(KeyboardKey::Kp2),           // SDL_SCANCODE_KP_2
    Some(KeyboardKey::Kp3),           // SDL_SCANCODE_KP_3
    Some(KeyboardKey::Kp4),           // SDL_SCANCODE_KP_4
    Some(KeyboardKey::Kp5),           // SDL_SCANCODE_KP_5
    Some(KeyboardKey::Kp6),           // SDL_SCANCODE_KP_6
    Some(KeyboardKey::Kp7),           // SDL_SCANCODE_KP_7
    Some(KeyboardKey::Kp8),           // SDL_SCANCODE_KP_8
    Some(KeyboardKey::Kp9),           // SDL_SCANCODE_KP_9
    Some(KeyboardKey::Kp0),           // SDL_SCANCODE_KP_0
    Some(KeyboardKey::KpDecimal),     // SDL_SCANCODE_KP_PERIOD
    None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None,
    None, None, None, None,
    Some(KeyboardKey::LeftControl),   //SDL_SCANCODE_LCTRL
    Some(KeyboardKey::LeftShift),     //SDL_SCANCODE_LSHIFT
    Some(KeyboardKey::LeftAlt),       //SDL_SCANCODE_LALT
    Some(KeyboardKey::LeftSuper),     //SDL_SCANCODE_LGUI
    Some(KeyboardKey::RightControl),  //SDL_SCANCODE_RCTRL
    Some(KeyboardKey::RightShift),    //SDL_SCANCODE_RSHIFT
    Some(KeyboardKey::RightAlt),      //SDL_SCANCODE_RALT
    Some(KeyboardKey::RightSuper),    //SDL_SCANCODE_RGUI
];

static CURSORS_LUT: [SystemCursor; 11] = [
    SystemCursor::Arrow,       // 0  MOUSE_CURSOR_DEFAULT
    SystemCursor::Arrow,       // 1  MOUSE_CURSOR_ARROW
    SystemCursor::IBeam,       // 2  MOUSE_CURSOR_IBEAM
    SystemCursor::Crosshair,   // 3  MOUSE_CURSOR_CROSSHAIR
    SystemCursor::Hand,        // 4  MOUSE_CURSOR_POINTING_HAND
    SystemCursor::SizeWE,      // 5  MOUSE_CURSOR_RESIZE_EW
    SystemCursor::SizeNS,      // 6  MOUSE_CURSOR_RESIZE_NS
    SystemCursor::SizeNWSE,    // 7  MOUSE_CURSOR_RESIZE_NWSE
    SystemCursor::SizeNESW,    // 8  MOUSE_CURSOR_RESIZE_NESW
    SystemCursor::SizeAll,     // 9  MOUSE_CURSOR_RESIZE_ALL
    SystemCursor::No,          // 10 MOUSE_CURSOR_NOT_ALLOWED
    //SystemCursor::Wait,      // No equivalent implemented on MouseCursor enum on raylib.h
    //SystemCursor::WaitArrow, // No equivalent implemented on MouseCursor enum on raylib.h
];

#[derive(Debug)]
pub enum InitPlatformError {
    WindowBuildError(WindowBuildError),
    SdlError(SdlError),
    TryFromIntError(TryFromIntError),
    IntegerOrSdlError(IntegerOrSdlError),
}

impl std::fmt::Display for InitPlatformError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WindowBuildError(e) => e.fmt(f),
            Self::SdlError(e) => e.fmt(f),
            Self::TryFromIntError(e) => e.fmt(f),
            Self::IntegerOrSdlError(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for InitPlatformError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::WindowBuildError(e) => Some(e),
            Self::SdlError(e) => Some(e),
            Self::TryFromIntError(e) => Some(e),
            Self::IntegerOrSdlError(e) => Some(e),
        }
    }
}

impl From<WindowBuildError> for InitPlatformError {
    fn from(value: WindowBuildError) -> Self {
        Self::WindowBuildError(value)
    }
}
impl From<SdlError> for InitPlatformError {
    fn from(value: SdlError) -> Self {
        Self::SdlError(value)
    }
}
impl From<TryFromIntError> for InitPlatformError {
    fn from(value: TryFromIntError) -> Self {
        Self::TryFromIntError(value)
    }
}
impl From<IntegerOrSdlError> for InitPlatformError {
    fn from(value: IntegerOrSdlError) -> Self {
        Self::IntegerOrSdlError(value)
    }
}

impl Platform {
    /// Initialize platform: graphics, inputs and more
    pub(crate) fn init(core: &mut Core) -> Result<Self, InitPlatformError> {
        use sdl3::sys::video::*;

        let sdl_context = sdl3::init()
            .inspect_err(|_| tracelog!(Warning, "SDL: Failed to initialize SDL"))?;

        // Initialize graphic device: display/window and graphic context
        //----------------------------------------------------------------------------
        let mut flags: u32 = 0;
        flags |= SDL_WINDOW_OPENGL as u32;
        flags |= SDL_WINDOW_INPUT_FOCUS as u32;
        flags |= SDL_WINDOW_MOUSE_FOCUS as u32;
        flags |= SDL_WINDOW_MOUSE_CAPTURE as u32;

        let video_subsystem = sdl_context.video()
            .map_err(|e| WindowBuildError::SdlError(e))?;

        if core.window.flags.contains(ConfigFlags::FullscreenMode) {
            core.window.fullscreen = true;
            flags |= SDL_WINDOW_FULLSCREEN as u32;
        }

        if core.window.flags.contains(ConfigFlags::WindowHidden) { flags |= SDL_WINDOW_HIDDEN as u32; }
        if core.window.flags.contains(ConfigFlags::WindowUndecorated) { flags |= SDL_WINDOW_BORDERLESS as u32; }
        if core.window.flags.contains(ConfigFlags::WindowResizable) { flags |= SDL_WINDOW_RESIZABLE as u32; }
        if core.window.flags.contains(ConfigFlags::WindowMinimized) { flags |= SDL_WINDOW_MINIMIZED as u32; }
        if core.window.flags.contains(ConfigFlags::WindowMaximized) { flags |= SDL_WINDOW_MAXIMIZED as u32; }

        if core.window.flags.contains(ConfigFlags::WindowUnfocused) {
            flags &= !SDL_WINDOW_INPUT_FOCUS as u32;
            flags &= !SDL_WINDOW_MOUSE_FOCUS as u32;
        }

        if core.window.flags.contains(ConfigFlags::WindowTopmost) { flags |= SDL_WINDOW_ALWAYS_ON_TOP as u32; }
        if core.window.flags.contains(ConfigFlags::WindowMousePassthrough) { flags &= !SDL_WINDOW_MOUSE_CAPTURE as u32; }

        if core.window.flags.contains(ConfigFlags::WindowHighdpi) { flags |= SDL_WINDOW_HIGH_PIXEL_DENSITY as u32; }

        // if core.window.flags.contains(ConfigFlags::WindowTransparent) { flags |= SDL_WINDOW_TRANSPARENT as u32; } // Alternative: SDL_GL_ALPHA_SIZE = 8

        // NOTE: Some OpenGL context attributes must be set before window creation
        {
            let gl_attr = video_subsystem.gl_attr();

            // todo: Check selection OpenGL version

            #[cfg(feature = "rlgl_enable_opengl_debug_context")]
            gl_attr.set_context_flags().debug(); // Enable OpenGL Debug Context

            if core.window.flags.contains(ConfigFlags::MSAA4xHint) {
                gl_attr.set_multisample_buffers(1);
                gl_attr.set_multisample_samples(4);
            }
        }

        // Init window
        let window = video_subsystem.window(&core.window.title, core.window.screen.width, core.window.screen.height)
            .set_window_flags(flags)
            .build()
            .inspect_err(|_| tracelog!(Fatal, "PLATFORM: Failed to initialize graphics device"))?;

        // Init OpenGL context
        let gl_context = window.gl_create_context()
            .inspect_err(|_| tracelog!(Fatal, "PLATFORM: Failed to initialize graphics device"))?;

        core.window.ready = true;

        let display_mode = window.get_display()
            .and_then(|display| display.get_mode())?;

        core.window.display.width = display_mode.w.try_into()?;
        core.window.display.height = display_mode.h.try_into()?;

        core.window.render.width = core.window.screen.width;
        core.window.render.height = core.window.screen.height;
        core.window.current_fbo.width = core.window.render.width;
        core.window.current_fbo.height = core.window.render.height;

        tracelog!(Info, "DISPLAY: Device initialized successfully");
        tracelog!(Info, "    > Display size: {} x {}", core.window.display.width, core.window.display.height);
        tracelog!(Info, "    > Screen size:  {} x {}", core.window.screen.width, core.window.screen.height);
        tracelog!(Info, "    > Render size:  {} x {}", core.window.render.width, core.window.render.height);
        tracelog!(Info, "    > Viewport offsets: {}, {}", core.window.render_offset.x, core.window.render_offset.y);

        video_subsystem.gl_set_swap_interval(if core.window.flags.contains(ConfigFlags::VsyncHint) { 1 } else { 0 })?;

        // Load OpenGL extensions
        // NOTE: GL procedures address loader is required to load extensions
        /* todo: rlLoadExtensions(SDL_GL_GetProcAddress); */

        // Initialize input events system
        //----------------------------------------------------------------------------
        // Initialize gamepads

        let gamepad: [Option<SdlGamepad>; MAX_GAMEPADS] = {
            let gamepad_subsystem = sdl_context.gamepad()?;
            let joystick_subsystem = sdl_context.joystick()?;
            let joysticks_instances = joystick_subsystem.joysticks()?;
            let mut joystick_iter = joysticks_instances.into_iter()
                .map(|joystick_instance| {
                    let id = joystick_instance.id;
                    let gamepad_joystick = gamepad_subsystem.open(id).and_then(|gamepad| joystick_subsystem.open(joystick_instance).map(|joystick| (gamepad, joystick)));
                    match gamepad_joystick {
                        Ok((gamepad, joystick)) => {
                            let core_gamepad = &mut core.input.gamepad.items[id as usize];
                            core_gamepad.ready = true;
                            core_gamepad.axis_count = joystick.num_axes();
                            core_gamepad.axis_state[GamepadAxis::LeftTrigger as usize] = -1.0;
                            core_gamepad.axis_state[GamepadAxis::RightTrigger as usize] = -1.0;
                            core_gamepad.name = gamepad.name().as_str().try_into()
                                .expect(concat!("gamepad name should not exceed ", stringify!(MAX_GAMEPAD_NAME_LEN), " characters"));

                            Some(gamepad)
                        }
                        Err(e) => {
                            tracelog!(Warning, "PLATFORM: Unable to open game controller [ERROR: {e}]");
                            None
                        }
                    }
                })
                .take(gamepad_subsystem.num_gamepads()? as usize);

            std::array::from_fn(|_| joystick_iter.next().flatten())
        };

        // Disable mouse events being interpreted as touch events
        // NOTE: This is wanted because there are SDL_FINGER* events available which provide unique data
        //       Due to the way PollInputEvents() and rgestures.h are currently implemented, setting this won't break SUPPORT_MOUSE_GESTURES
        sdl3::hint::set(sdl3::hint::names::TOUCH_MOUSE_EVENTS, "0");

        /* todo: SDL_EventState(SDL_DROPFILE, SDL_ENABLE); */
        //----------------------------------------------------------------------------

        // Initialize timing system
        //----------------------------------------------------------------------------
        // NOTE: No need to call InitTimer(), let SDL manage it internally
        core.time.previous = get_time(); // Get time as double

        #[cfg(all(target_os = "windows", target_arch = "x86", feature = "support_winmm_highres_timer", not(feature = "support_busy_wait_loop")))]
        sdl3::hint::set(sdl3::hint::names::TIMER_RESOLUTION, "1"); // SDL equivalent of timeBeginPeriod() and timeEndPeriod()
        //----------------------------------------------------------------------------

        // Initialize storage system
        //----------------------------------------------------------------------------
        // Define base path for storage
        core.storage.base_path = sdl3::filesystem::get_base_path().ok(); // Alternative: GetWorkingDirectory();
        //----------------------------------------------------------------------------

        tracelog!(Info, "PLATFORM: DESKTOP (SDL3): Initialized successfully");

        Ok(Platform {
            sdl_context,
            video_subsystem,
            window,
            gl_context,
            gamepad,
            cursor: None,
            cursor_relative: false,
        })
    }
}

impl Drop for Platform {
    /// Close platform
    fn drop(&mut self) {}
}

/// Check if application should close
pub fn window_should_close(core: &Core) -> bool {
    !core.window.ready || core.window.should_close
}

/// Toggle fullscreen mode
pub fn toggle_fullscreen(core: &mut Core, platform: &mut Platform) -> Result<(), SdlError> {
    let new_value = !core.window.flags.contains(ConfigFlags::FullscreenMode);
    platform.window.set_fullscreen(new_value).inspect_err(|_| tracelog!(Warning, "SDL: Failed to find selected monitor"))?;
    core.window.flags.set(ConfigFlags::FullscreenMode, new_value);
    core.window.fullscreen = new_value;
    Ok(())
}

/// Toggle borderless windowed mode
pub fn toggle_borderless_windowed(core: &mut Core, platform: &mut Platform) -> Result<(), SdlError> {
    let new_value = !core.window.flags.contains(ConfigFlags::BorderlessWindowedMode);
    platform.window.set_fullscreen(new_value).inspect_err(|_| tracelog!(Warning, "SDL: Failed to find selected monitor"))?;
    platform.window.set_bordered(new_value).inspect_err(|_| tracelog!(Warning, "SDL: Failed to find selected monitor"))?;
    core.window.flags.set(ConfigFlags::BorderlessWindowedMode, new_value);
    core.window.fullscreen = new_value;
    Ok(())
}

pub fn maximize_window() {
    todo!()
}
pub fn minimize_window() {
    todo!()
}
pub fn restore_window() {
    todo!()
}

pub fn set_window_state(flags: ConfigFlags) {
    todo!()
}
pub fn clear_window_state(flags: ConfigFlags) {
    todo!()
}

pub fn set_window_icon(image: &Image) {
    todo!()
}
pub fn set_window_icons(images: &[Image]) {
    todo!()
}
pub fn set_window_title(title: &str) {
    todo!()
}
pub fn set_window_position(x: u32, y: u32) {
    todo!()
}
pub fn set_window_monitor(monitor: sdl3::sys::video::SDL_DisplayID) {
    todo!()
}
pub fn set_window_min_size(width: u32, height: u32) {
    todo!()
}
pub fn set_window_max_size(width: u32, height: u32) {
    todo!()
}
pub fn set_window_size(width: u32, height: u32) {
    todo!()
}
pub fn set_window_opacity(opacity: f32) {
    todo!()
}
pub fn set_window_focused() {
    todo!()
}
pub fn get_window_handle() -> *mut std::ffi::c_void {
    todo!()
}
pub fn get_window_position() -> Vector2 {
    todo!()
}
pub fn get_window_scale_dpi() -> Vector2 {
    todo!()
}

pub fn get_monitor_count() -> usize {
    todo!()
}
pub fn get_current_monitor() -> sdl3::sys::video::SDL_DisplayID {
    todo!()
}
pub fn get_monitor_width(monitor: sdl3::sys::video::SDL_DisplayID) -> u32 {
    todo!()
}
pub fn get_monitor_height(monitor: sdl3::sys::video::SDL_DisplayID) -> u32 {
    todo!()
}
pub fn get_monitor_physical_width(monitor: sdl3::sys::video::SDL_DisplayID) -> u32 {
    todo!()
}
pub fn get_monitor_physical_height(monitor: sdl3::sys::video::SDL_DisplayID) -> u32 {
    todo!()
}
pub fn get_monitor_refresh_rate(monitor: sdl3::sys::video::SDL_DisplayID) -> u32 {
    todo!()
}
pub fn get_monitor_position(monitor: sdl3::sys::video::SDL_DisplayID) -> Vector2 {
    todo!()
}
pub fn get_monitor_name(monitor: sdl3::sys::video::SDL_DisplayID) -> String {
    todo!()
}

pub fn set_clipboard_text(text: &'static str) {
    todo!()
}
pub fn get_clipboard_text() -> String {
    todo!()
}

pub fn show_cursor() {
    todo!()
}
pub fn hide_cursor() {
    todo!()
}
pub fn enable_cursor() {
    todo!()
}
pub fn disable_cursor() {
    todo!()
}

/// Get elapsed time measure in seconds
fn get_time() -> f64 {
    let ms = sdl3::timer::ticks(); // Elapsed time in milliseconds since SDL_Init()
    let time = ms as f64/1000.0;
    time
}

/// Scancode to keycode mapping
fn convert_scancode_to_key(sdl_scancode: sdl3::keyboard::Scancode) -> Option<KeyboardKey> {
    sdl_scancode.to_i32().try_into().ok().and_then(|code: usize| MAP_SCANCODE_TO_KEY.get(code).copied().flatten())
}
