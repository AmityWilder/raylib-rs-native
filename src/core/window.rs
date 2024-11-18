use std::{os::raw::c_void, path::Path};

use bitflags::bitflags;
use crate::prelude::*;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ConfigFlags: usize {
        /// Set to try enabling V-Sync on GPU
        const VsyncHint              = 0x00000040;
        /// Set to run program in fullscreen
        const FullscreenMode         = 0x00000002;
        /// Set to allow resizable window
        const WindowResizable        = 0x00000004;
        /// Set to disable window decoration (frame and buttons)
        const WindowUndecorated      = 0x00000008;
        /// Set to hide window
        const WindowHidden           = 0x00000080;
        /// Set to minimize window (iconify)
        const WindowMinimized        = 0x00000200;
        /// Set to maximize window (expanded to monitor)
        const WindowMaximized        = 0x00000400;
        /// Set to window non focused
        const WindowUnfocused        = 0x00000800;
        /// Set to window always on top
        const WindowTopmost          = 0x00001000;
        /// Set to allow windows running while minimized
        const WindowAlwaysRun        = 0x00000100;
        /// Set to allow transparent framebuffer
        const WindowTransparent      = 0x00000010;
        /// Set to support HighDPI
        const WindowHighdpi          = 0x00002000;
        /// Set to support mouse passthrough, only supported when FLAG_WINDOW_UNDECORATED
        const WindowMousePassthrough = 0x00004000;
        /// Set to run program in borderless windowed mode
        const BorderlessWindowedMode = 0x00008000;
        /// Set to try enabling MSAA 4X
        const MSAA4xHint             = 0x00000020;
        /// Set to try enabling interlaced video format (for V3D)
        const InterlacedHint         = 0x00010000;
    }
}

impl Default for ConfigFlags {
    fn default() -> Self {
        Self(Default::default())
    }
}

pub type MonitorID = usize;

#[must_use]
#[derive(Debug, Default)]
pub struct Window<'a, 'b> {
    /// Window text title const pointer
    pub(super) title: &'a str,
    /// Configuration flags (bit based), keeps window state
    pub(super) flags: ConfigFlags,
    /// Check if window has been initialized successfully
    pub(super) ready: bool,
    /// Check if fullscreen mode is enabled
    pub(super) fullscreen: bool,
    /// Check if window set for closing
    pub(super) should_close: bool,
    /// Check if window has been resized last frame
    pub(super) resized_last_frame: bool,
    /// Wait for events before ending frame
    pub(super) event_waiting: bool,
    /// Using FBO (RenderTexture) for rendering instead of default framebuffer
    pub(super) using_fbo: bool,

    /// Window position (required on fullscreen toggle)
    pub(super) position: Point,
    /// Window previous position (required on borderless windowed toggle)
    pub(super) previous_position: Point,
    /// Display width and height (monitor, device-screen, LCD, ...)
    pub(super) display: Size,
    /// Screen width and height (used render area)
    pub(super) screen: Size,
    /// Screen previous width and height (required on borderless windowed toggle)
    pub(super) previous_screen: Size,
    /// Current render width and height (depends on active fbo)
    pub(super) current_fbo: Size,
    /// Framebuffer width and height (render area, including black bars if required)
    pub(super) render: Size,
    /// Offset from render area (must be divided by 2)
    pub(super) render_offset: Point,
    /// Screen minimum width and height (for resizable window)
    pub(super) screen_min: Size,
    /// Screen maximum width and height (for resizable window)
    pub(super) screen_max: Size,
    /// Matrix to scale screen (framebuffer rendering)
    pub(super) screen_scale: Matrix,

    /// Store dropped files paths pointers (provided by GLFW)
    pub(super) drop_filepaths: Vec<&'b Path>,
}

impl<'a, 'b> Window<'a, 'b> {
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
