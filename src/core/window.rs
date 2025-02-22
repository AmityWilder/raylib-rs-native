use std::{os::raw::c_void, path::Path};
use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ConfigFlags(pub u32);

#[allow(non_upper_case_globals)]
impl ConfigFlags {
    /// Set to try enabling V-Sync on GPU
    pub const VsyncHint:              Self = Self(0x00000040);
    /// Set to run program in fullscreen
    pub const FullscreenMode:         Self = Self(0x00000002);
    /// Set to allow resizable window
    pub const WindowResizable:        Self = Self(0x00000004);
    /// Set to disable window decoration (frame and buttons)
    pub const WindowUndecorated:      Self = Self(0x00000008);
    /// Set to hide window
    pub const WindowHidden:           Self = Self(0x00000080);
    /// Set to minimize window (iconify)
    pub const WindowMinimized:        Self = Self(0x00000200);
    /// Set to maximize window (expanded to monitor)
    pub const WindowMaximized:        Self = Self(0x00000400);
    /// Set to window non focused
    pub const WindowUnfocused:        Self = Self(0x00000800);
    /// Set to window always on top
    pub const WindowTopmost:          Self = Self(0x00001000);
    /// Set to allow windows running while minimized
    pub const WindowAlwaysRun:        Self = Self(0x00000100);
    /// Set to allow transparent framebuffer
    pub const WindowTransparent:      Self = Self(0x00000010);
    /// Set to support HighDPI
    pub const WindowHighdpi:          Self = Self(0x00002000);
    /// Set to support mouse passthrough, only supported when FLAG_WINDOW_UNDECORATED
    pub const WindowMousePassthrough: Self = Self(0x00004000);
    /// Set to run program in borderless windowed mode
    pub const BorderlessWindowedMode: Self = Self(0x00008000);
    /// Set to try enabling MSAA 4X
    pub const MSAA4xHint:             Self = Self(0x00000020);
    /// Set to try enabling interlaced video format (for V3D)
    pub const InterlacedHint:         Self = Self(0x00010000);

    pub const fn has(self, flags: Self) -> bool {
        self.0 & flags.0 != 0
    }
}

impl PartialEq<u32> for ConfigFlags {
    fn eq(&self, other: &u32) -> bool {
        self.0.eq(other)
    }
}

impl PartialOrd<u32> for ConfigFlags {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl std::ops::Not for ConfigFlags {
    type Output = Self;
    fn not(self) -> Self::Output { Self(self.0.not()) }
}

impl std::ops::BitAnd for ConfigFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output { Self(self.0.bitand(rhs.0)) }
}

impl std::ops::BitOr for ConfigFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output { Self(self.0.bitor(rhs.0)) }
}

impl std::ops::BitXor for ConfigFlags {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output { Self(self.0.bitxor(rhs.0)) }
}

impl std::ops::BitAndAssign for ConfigFlags {
    fn bitand_assign(&mut self, rhs: Self) { self.0.bitand_assign(rhs.0) }
}

impl std::ops::BitOrAssign for ConfigFlags {
    fn bitor_assign(&mut self, rhs: Self) { self.0.bitor_assign(rhs.0) }
}

impl std::ops::BitXorAssign for ConfigFlags {
    fn bitxor_assign(&mut self, rhs: Self) { self.0.bitxor_assign(rhs.0) }
}

pub type MonitorID = usize;

#[must_use]
#[derive(Debug, Default)]
pub struct Window {
    /// Window text title const pointer
    pub title: String,
    /// Configuration flags (bit based), keeps window state
    pub flags: ConfigFlags,
    /// Check if window has been initialized successfully
    pub ready: bool,
    /// Check if fullscreen mode is enabled
    pub fullscreen: bool,
    /// Check if window set for closing
    pub should_close: bool,
    /// Check if window has been resized last frame
    pub resized_last_frame: bool,
    /// Wait for events before ending frame
    pub event_waiting: bool,
    /// Using FBO (RenderTexture) for rendering instead of default framebuffer
    pub using_fbo: bool,

    /// Window position (required on fullscreen toggle)
    pub position: Point,
    /// Window previous position (required on borderless windowed toggle)
    pub previous_position: Point,
    /// Display width and height (monitor, device-screen, LCD, ...)
    pub display: Size,
    /// Screen width and height (used render area)
    pub screen: Size,
    /// Screen previous width and height (required on borderless windowed toggle)
    pub previous_screen: Size,
    /// Current render width and height (depends on active fbo)
    pub current_fbo: Size,
    /// Framebuffer width and height (render area, including black bars if required)
    pub render: Size,
    /// Offset from render area (must be divided by 2)
    pub render_offset: Point,
    /// Screen minimum width and height (for resizable window)
    pub screen_min: Size,
    /// Screen maximum width and height (for resizable window)
    pub screen_max: Size,
    /// Matrix to scale screen (framebuffer rendering)
    pub screen_scale: Matrix,

    /// Store dropped files paths pointers (provided by GLFW)
    pub drop_filepaths: Vec<Box<Path>>,
}

impl Window {
    /// Check if application should close (KEY_ESCAPE pressed or windows close icon clicked)
    pub fn should_close(&self) -> bool {
        todo!()
    }

    /// Check if window has been initialized successfully
    pub fn is_ready(&self) -> bool {
        todo!()
    }

    /// Check if window is currently fullscreen
    pub fn is_fullscreen(&self) -> bool {
        todo!()
    }

    /// Check if window is currently hidden
    pub fn is_hidden(&self) -> bool {
        todo!()
    }

    /// Check if window is currently minimized
    pub fn is_minimized(&self) -> bool {
        todo!()
    }

    /// Check if window is currently maximized
    pub fn is_maximized(&self) -> bool {
        todo!()
    }

    /// Check if window is currently focused
    pub fn is_focused(&self) -> bool {
        todo!()
    }

    /// Check if window has been resized last frame
    pub fn is_resized(&self) -> bool {
        todo!()
    }

    /// Check if one specific window flag is enabled
    pub fn is_state(&self, flag: ConfigFlags) -> bool {
        todo!()
    }

    /// Set window configuration state using flags
    pub fn set_state(&mut self, flags: ConfigFlags) {
        todo!()
    }

    /// Clear window configuration state flags
    pub fn clear_state(&mut self, flags: ConfigFlags) {
        todo!()
    }

    /// Toggle window state: fullscreen/windowed, resizes monitor to match window resolution
    pub fn toggle_fullscreen(&mut self) {
        todo!()
    }

    /// Toggle window state: borderless windowed, resizes window to match monitor resolution
    pub fn toggle_borderless_windowed(&mut self) {
        todo!()
    }

    /// Set window state: maximized, if resizable
    pub fn maximize(&mut self) {
        todo!()
    }

    /// Set window state: minimized, if resizable
    pub fn minimize(&mut self) {
        todo!()
    }

    /// Set window state: not minimized/maximized
    pub fn restore(&mut self) {
        todo!()
    }

    /// Set icon for window (single image, RGBA 32bit)
    pub fn set_icon(&mut self, image: &Image) {
        todo!()
    }

    /// Set icon for window (multiple images, RGBA 32bit)
    pub fn set_icons(&mut self, images: &[Image]) {
        todo!()
    }

    /// Set title for window
    pub fn set_title(&mut self, title: &str) {
        todo!()
    }

    /// Set window position on screen
    pub fn set_position(&mut self, x: i32, y: i32) {
        todo!()
    }

    /// Set monitor for the current window
    pub fn set_monitor(&mut self, monitor: MonitorID) {
        todo!()
    }

    /// Set window minimum dimensions (for FLAG_WINDOW_RESIZABLE)
    pub fn set_min_size(&mut self, width: usize, height: usize) {
        todo!()
    }

    /// Set window maximum dimensions (for FLAG_WINDOW_RESIZABLE)
    pub fn set_max_size(&mut self, width: usize, height: usize) {
        todo!()
    }

    /// Set window dimensions
    pub fn set_size(&mut self, width: usize, height: usize) {
        todo!()
    }

    /// Set window opacity [0.0f..1.0f]
    pub fn set_opacity(&mut self, opacity: f32) {
        todo!()
    }

    /// Set window focused
    pub fn set_focused(&mut self) {
        todo!()
    }

    /// Get native window handle
    pub fn handle(&self) -> *mut c_void {
        todo!()
    }

    /// Get current screen width
    pub fn screen_width(&self) -> usize {
        todo!()
    }

    /// Get current screen height
    pub fn screen_height(&self) -> usize {
        todo!()
    }

    /// Get current render width (it considers HiDPI)
    pub fn render_width(&self) -> usize {
        todo!()
    }

    /// Get current render height (it considers HiDPI)
    pub fn render_height(&self) -> usize {
        todo!()
    }

    /// Get number of connected monitors
    pub fn monitor_count(&self) -> usize {
        todo!()
    }

    /// Get current monitor where window is placed
    pub fn current_monitor(&self) -> MonitorID {
        todo!()
    }

    /// Get specified monitor position
    pub fn monitor_position(&self, monitor: MonitorID) -> Position2 {
        todo!()
    }

    /// Get specified monitor width (current video mode used by monitor)
    pub fn monitor_width(&self, monitor: MonitorID) -> usize {
        todo!()
    }

    /// Get specified monitor height (current video mode used by monitor)
    pub fn monitor_height(&self, monitor: MonitorID) -> usize {
        todo!()
    }

    /// Get specified monitor physical width in millimetres
    pub fn monitor_physical_width(&self, monitor: MonitorID) -> usize {
        todo!()
    }

    /// Get specified monitor physical height in millimetres
    pub fn monitor_physical_height(&self, monitor: MonitorID) -> usize {
        todo!()
    }

    /// Get specified monitor refresh rate
    pub fn monitor_refresh_rate(&self, monitor: MonitorID) -> usize {
        todo!()
    }

    /// Get window position XY on monitor
    pub fn window_position(&self) -> Position2 {
        todo!()
    }

    /// Get window scale DPI factor
    pub fn window_scale_dpi(&self) -> Vector2 {
        todo!()
    }

    /// Get the human-readable, UTF-8 encoded name of the specified monitor
    pub fn monitor_name(&self, monitor: MonitorID) -> &str {
        todo!()
    }

    /// Set clipboard text content
    pub fn set_clipboard_text(&mut self, text: &str) {
        todo!()
    }

    /// Get clipboard text content
    pub fn clipboard_text(&self) -> &str {
        todo!()
    }

    /// Get clipboard image content
    pub fn clipboard_image(&self) -> &Image {
        todo!()
    }

    /// Enable waiting for events on EndDrawing(), no automatic event polling
    pub fn enable_event_waiting(&mut self) {
        todo!()
    }

    /// Disable waiting for events on EndDrawing(), automatic events polling
    pub fn disable_event_waiting(&mut self) {
        todo!()
    }

    // Cursor-related functions

    /// Shows cursor
    pub fn show_cursor(&mut self) {
        todo!()
    }

    /// Hides cursor
    pub fn hide_cursor(&mut self) {
        todo!()
    }

    /// Check if cursor is not visible
    pub fn is_cursor_hidden(&self) -> bool {
        todo!()
    }

    /// Enables cursor (unlock cursor)
    pub fn enable_cursor(&mut self) {
        todo!()
    }

    /// Disables cursor (lock cursor)
    pub fn disable_cursor(&mut self) {
        todo!()
    }

    /// Check if cursor is on the screen
    pub fn is_cursor_on_screen(&self) -> bool {
        todo!()
    }

}
