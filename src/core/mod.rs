use std::path::PathBuf;
use crate::{external::msf_gif::MsfGifResult, prelude::*, tracelog, TraceLogType::{self, Info}};
use input::Input;
use window::Window;

pub mod window;
pub mod input;

extern "Rust" {
    fn WindowShouldClose() -> bool;
    fn ToggleFullscreen();
    fn ToggleBorderlessWindowed();
    fn MaximizeWindow();
    fn MinimizeWindow();
    fn RestoreWindow();

    fn SetWindowState(flags: ConfigFlags);
    fn ClearWindowState(flags: ConfigFlags);

    fn SetWindowIcon(image: &Image);
    fn SetWindowIcons(images: &[Image]);
    fn SetWindowTitle(title: &str);
    fn SetWindowPosition(x: u32, y: u32);
    fn SetWindowMonitor(monitor: sdl3::sys::video::SDL_DisplayID);
    fn SetWindowMinSize(width: u32, height: u32);
    fn SetWindowMaxSize(width: u32, height: u32);
    fn SetWindowSize(width: u32, height: u32);
    fn SetWindowOpacity(opacity: f32);
    fn SetWindowFocused();
    fn GetWindowHandle() -> *mut std::ffi::c_void;
    fn GetWindowPosition() -> Vector2;
    fn GetWindowScaleDPI() -> Vector2;

    fn GetMonitorCount() -> usize;
    fn GetCurrentMonitor() -> sdl3::sys::video::SDL_DisplayID;
    fn GetMonitorWidth(monitor: sdl3::sys::video::SDL_DisplayID) -> u32;
    fn GetMonitorHeight(monitor: sdl3::sys::video::SDL_DisplayID) -> u32;
    fn GetMonitorPhysicalWidth(monitor: sdl3::sys::video::SDL_DisplayID) -> u32;
    fn GetMonitorPhysicalHeight(monitor: sdl3::sys::video::SDL_DisplayID) -> u32;
    fn GetMonitorRefreshRate(monitor: sdl3::sys::video::SDL_DisplayID) -> u32;
    fn GetMonitorPosition(monitor: sdl3::sys::video::SDL_DisplayID) -> Vector2;
    fn GetMonitorName(monitor: sdl3::sys::video::SDL_DisplayID) -> String;

    fn SetClipboardText(text: &'static str);
    fn GetClipboardText() -> String;

    fn ShowCursor();
    fn HideCursor();
    fn EnableCursor();
    fn DisableCursor();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

pub enum InputEventType {
    /// param[0]: key
    KeyUp,
    /// param[0]: key
    KeyDown,
    /// param[0]: key
    KeyPressed,
    /// param[0]: key
    KeyReleased,
    /// param[0]: button
    MouseButtonUp,
    /// param[0]: button
    MouseButtonDown,
    /// param[0]: x, param[1]: y
    MousePosition,
    /// param[0]: x delta, param[1]: y delta
    MouseWheelMotion,
    /// param[0]: gamepad
    GamepadConnect,
    /// param[0]: gamepad
    GamepadDisconnect,
    /// param[0]: button
    GamepadButtonUp,
    /// param[0]: button
    GamepadButtonDown,
    /// param[0]: axis, param[1]: delta
    GamepadAxisMotion,
    /// param[0]: id
    TouchUp,
    /// param[0]: id
    TouchDown,
    /// param[0]: x, param[1]: y
    TouchPosition,
    /// param[0]: gesture
    Gesture,
}

pub enum WindowEventType {
    /// no params
    Close,
    /// no params
    Maximize,
    /// no params
    Minimize,
    /// param[0]: width, param[1]: height
    Resize,
}

pub enum CustomEventType {
    /// no params
    TakeScreenshot,
    /// param[0]: fps
    SetTargetFps,
}

/// Automation events type
pub enum AutomationEventType {
    Input(InputEventType),
    Window(WindowEventType),
    Custom(CustomEventType),
}

/// Automation event
pub struct AutomationEvent {
    /// Event frame
    frame: usize,
    /// Event type (AutomationEventType)
    ty: AutomationEventType,
    /// Event parameters (if required)
    params: [i32; 4],
}

#[derive(Debug, Default)]
pub struct Storage {
    /// Base path for data storage
    base_path: PathBuf,
}

#[derive(Debug, Default)]
pub struct Time {
    /// Current time measure
    current: f64,
    /// Previous time measure
    previous: f64,
    /// Time measure for frame update
    update: f64,
    /// Time measure for frame draw
    draw: f64,
    /// Time measure for one frame
    frame: f64,
    /// Desired time for one frame, if 0 not applied
    target: f64,
    /// Base time measure for hi-res timer (PLATFORM_ANDROID, PLATFORM_DRM)
    base: usize,
    /// Frame counter
    frame_counter: usize,
}

/// Core global state context data
pub struct Core {
    pub window: Window,
    pub storage: Storage,
    pub input: Input,
    pub time: Time,
    is_gpu_ready: bool,
    /// Current automation events list, set by user, keep internal pointer
    current_event_list: Option<Vec<AutomationEvent>>,
    /// Recording automation events flag
    automation_event_recording: bool,

    /// Screenshots counter
    #[cfg(feature = "support_screen_capture")]
    screenshot_counter: usize,

    /// GIF frames counter
    #[cfg(feature = "support_gif_recording")]
    gif_frame_counter: u32,
    /// GIF recording state
    #[cfg(feature = "support_gif_recording")]
    gif_recording: bool,
    /// MSGIF context state
    #[cfg(feature = "support_gif_recording")]
    gif_state: MsfGifState,
}

impl Default for Core {
    fn default() -> Self {
        Self {
            tracelog: Default::default(),
            window: Default::default(),
            storage: Default::default(),
            input: Default::default(),
            time: Default::default(),
            is_gpu_ready: false,
            current_event_list: None,
            automation_event_recording: false,

            #[cfg(feature = "support_screen_capture")]
            screenshot_counter: 0,

            #[cfg(feature = "support_gif_recording")]
            gif_frame_counter: 0,
            #[cfg(feature = "support_gif_recording")]
            gif_recording: false,
            #[cfg(feature = "support_gif_recording")]
            gif_state: Default::default(),
        }
    }
}

impl Core {
    /// Initialize window and OpenGL context
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        tracelog!(TraceLogType::Info, "Initializing raylib {}", crate::RAYLIB_VERSION);

        tracelog!(TraceLogType::Info, "Platform backend: DESKTOP (SDL)");

        tracelog!(TraceLogType::Info, "Supported raylib modules:");
        tracelog!(TraceLogType::Info, "    > rcore:..... loaded (mandatory)");
        tracelog!(TraceLogType::Info, "    > rlgl:...... loaded (mandatory)");
        if cfg!(feature = "support_module_rshapes") {
            tracelog!(TraceLogType::Info, "    > rshapes:... loaded (optional)");
        } else {
            tracelog!(TraceLogType::Info, "    > rshapes:... not loaded (optional)");
        }

        if cfg!(feature = "support_module_rtextures") {
            tracelog!(TraceLogType::Info, "    > rtextures:. loaded (optional)");
        } else {
            tracelog!(TraceLogType::Info, "    > rtextures:. not loaded (optional)");
        }

        if cfg!(feature = "support_module_rtext") {
            tracelog!(TraceLogType::Info, "    > rtext:..... loaded (optional)");
        } else {
            tracelog!(TraceLogType::Info, "    > rtext:..... not loaded (optional)");
        }

        if cfg!(feature = "support_module_rmodels") {
            tracelog!(TraceLogType::Info, "    > rmodels:... loaded (optional)");
        } else {
            tracelog!(TraceLogType::Info, "    > rmodels:... not loaded (optional)");
        }

        if cfg!(feature = "support_module_raudio") {
            tracelog!(TraceLogType::Info, "    > raudio:.... loaded (optional)");
        } else {
            tracelog!(TraceLogType::Info, "    > raudio:.... not loaded (optional)");
        }

        let mut core = Self::default();

        // Initialize window data
        core.window.screen.width = width;
        core.window.screen.height = height;
        core.window.event_waiting = false;
        core.window.screen_scale = Matrix::IDENTITY; // No draw scaling required by default
        if !title.is_empty() {
            core.window.title = title.to_string();
        }

        // Initialize global input state
        core.input = Input::default(); // Reset core.input structure to 0
        core.input.keyboard.exit_key = Some(KeyboardKey::Escape);
        core.input.mouse.scale = Vector2::new(1.0, 1.0);
        core.input.mouse.cursor = MouseCursor::Arrow;
        core.input.gamepad.last_button_pressed = None;

        // // Initialize platform
        // //--------------------------------------------------------------
        // InitPlatform();
        // //--------------------------------------------------------------

        // // Initialize rlgl default data (buffers and shaders)
        // // NOTE: core.window.current_fbo.width and core.window.current_fbo.height not used, just stored as globals in rlgl
        // rlglInit(core.window.current_fbo.width, core.window.current_fbo.height);
        // core.is_gpu_ready = true; // Flag to note GPU has been initialized successfully

        // // Setup default viewport
        // SetupViewport(core.window.current_fbo.width, core.window.current_fbo.height);

        // if cfg!(feature = "support_module_rtext") {
        //     if cfg!(support_default_font) {
        //         // Load default font
        //         // WARNING: External function: Module required: rtext
        //         LoadFontDefault();
        //         if cfg!(feature = "support_module_rshapes") {
        //             // Set font white rectangle for shapes drawing, so shapes and text can be batched together
        //             // WARNING: rshapes module is required, if not available, default internal white rectangle is used
        //             let rec = GetFontDefault().recs[95];
        //             if core.window.flags.contains(ConfigFlags::MSAA4xHint) {
        //                 // NOTE: We try to maxime rec padding to avoid pixel bleeding on MSAA filtering
        //                 SetShapesTexture(GetFontDefault().texture, (Rectangle){ rec.x + 2, rec.y + 2, 1, 1 });
        //             } else {
        //                 // NOTE: We set up a 1px padding on char rectangle to avoid pixel bleeding
        //                 SetShapesTexture(GetFontDefault().texture, (Rectangle){ rec.x + 1, rec.y + 1, rec.width - 2, rec.height - 2 });
        //             }
        //         }
        //     }
        // } else {
        //     if cfg!(feature = "support_module_rshapes") {
        //         // Set default texture and rectangle to be used for shapes drawing
        //         // NOTE: rlgl default texture is a 1x1 pixel UNCOMPRESSED_R8G8B8A8
        //         let texture = Texture {
        //             id: rlGetTextureIdDefault(),
        //             width: 1,
        //             height: 1,
        //             mipmap: 1,
        //             format: PixelFormat::UncompressedR8G8B8A8,
        //         };
        //         // WARNING: Module required: rshapes
        //         SetShapesTexture(texture, Rectangle::new(0.0, 0.0, 1.0, 1.0));
        //     }
        // }

        // core.time.frame_counter = 0;
        // core.window.should_close = false;

        // // Initialize random seed
        // SetRandomSeed(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as u32);

        // TRACELOG!(TraceLogType::Info, "SYSTEM: Working Directory: %s", GetWorkingDirectory());

        core
    }
}
