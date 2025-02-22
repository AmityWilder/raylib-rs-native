use crate::prelude::Core;

pub fn WindowShouldClose(core: &Core) -> bool {

}

pub fn ToggleFullscreen();
pub fn ToggleBorderlessWindowed();
pub fn MaximizeWindow();
pub fn MinimizeWindow();
pub fn RestoreWindow();

pub fn SetWindowState(flags: ConfigFlags);
pub fn ClearWindowState(flags: ConfigFlags);

pub fn SetWindowIcon(image: &Image);
pub fn SetWindowIcons(images: &[Image]);
pub fn SetWindowTitle(title: &str);
pub fn SetWindowPosition(x: u32, y: u32);
pub fn SetWindowMonitor(monitor: sdl3::sys::video::SDL_DisplayID);
pub fn SetWindowMinSize(width: u32, height: u32);
pub fn SetWindowMaxSize(width: u32, height: u32);
pub fn SetWindowSize(width: u32, height: u32);
pub fn SetWindowOpacity(opacity: f32);
pub fn SetWindowFocused();
pub fn GetWindowHandle() -> *mut std::ffi::c_void;
pub fn GetWindowPosition() -> Vector2;
pub fn GetWindowScaleDPI() -> Vector2;

pub fn GetMonitorCount() -> usize;
pub fn GetCurrentMonitor() -> sdl3::sys::video::SDL_DisplayID;
pub fn GetMonitorWidth(monitor: sdl3::sys::video::SDL_DisplayID) -> u32;
pub fn GetMonitorHeight(monitor: sdl3::sys::video::SDL_DisplayID) -> u32;
pub fn GetMonitorPhysicalWidth(monitor: sdl3::sys::video::SDL_DisplayID) -> u32;
pub fn GetMonitorPhysicalHeight(monitor: sdl3::sys::video::SDL_DisplayID) -> u32;
pub fn GetMonitorRefreshRate(monitor: sdl3::sys::video::SDL_DisplayID) -> u32;
pub fn GetMonitorPosition(monitor: sdl3::sys::video::SDL_DisplayID) -> Vector2;
pub fn GetMonitorName(monitor: sdl3::sys::video::SDL_DisplayID) -> String;

pub fn SetClipboardText(text: &'static str);
pub fn GetClipboardText() -> String;

pub fn ShowCursor();
pub fn HideCursor();
pub fn EnableCursor();
pub fn DisableCursor();
