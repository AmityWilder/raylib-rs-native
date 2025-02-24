use arrayvec::{ArrayString, ArrayVec};
use bitflags::bitflags;
use crate::{config::*, prelude::*};

/// Keyboard keys (US keyboard layout)
/// NOTE: Use GetKeyPressed() to allow redefining
/// required keys for alternative layouts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyboardKey {
    // Alphanumeric keys

    /** '                          */ Apostrophe      =  39,
    /** ,                          */ Comma           =  44,
    /** -                          */ Minus           =  45,
    /** .                          */ Period          =  46,
    /** /                          */ Slash           =  47,
    /** 0                          */ Zero            =  48,
    /** 1                          */ One             =  49,
    /** 2                          */ Two             =  50,
    /** 3                          */ Three           =  51,
    /** 4                          */ Four            =  52,
    /** 5                          */ Five            =  53,
    /** 6                          */ Six             =  54,
    /** 7                          */ Seven           =  55,
    /** 8                          */ Eight           =  56,
    /** 9                          */ Nine            =  57,
    /** ;                          */ Semicolon       =  59,
    /** =                          */ Equal           =  61,
    /** A | a                      */ A               =  65,
    /** B | b                      */ B               =  66,
    /** C | c                      */ C               =  67,
    /** D | d                      */ D               =  68,
    /** E | e                      */ E               =  69,
    /** F | f                      */ F               =  70,
    /** G | g                      */ G               =  71,
    /** H | h                      */ H               =  72,
    /** I | i                      */ I               =  73,
    /** J | j                      */ J               =  74,
    /** K | k                      */ K               =  75,
    /** L | l                      */ L               =  76,
    /** M | m                      */ M               =  77,
    /** N | n                      */ N               =  78,
    /** O | o                      */ O               =  79,
    /** P | p                      */ P               =  80,
    /** Q | q                      */ Q               =  81,
    /** R | r                      */ R               =  82,
    /** S | s                      */ S               =  83,
    /** T | t                      */ T               =  84,
    /** U | u                      */ U               =  85,
    /** V | v                      */ V               =  86,
    /** W | w                      */ W               =  87,
    /** X | x                      */ X               =  88,
    /** Y | y                      */ Y               =  89,
    /** Z | z                      */ Z               =  90,
    /** [                          */ LeftBracket     =  91,
    /** \\                         */ Backslash       =  92,
    /** ]                          */ RightBracket    =  93,
    /** `                          */ Grave           =  96,

    // Function keys

    /** Space                      */ Space           =  32,
    /** Esc                        */ Escape          = 256,
    /** Enter                      */ Enter           = 257,
    /** Tab                        */ Tab             = 258,
    /** Backspace                  */ Backspace       = 259,
    /** Ins                        */ Insert          = 260,
    /** Del                        */ Delete          = 261,
    /** Cursor right               */ Right           = 262,
    /** Cursor left                */ Left            = 263,
    /** Cursor down                */ Down            = 264,
    /** Cursor up                  */ Up              = 265,
    /** Page up                    */ PageUp          = 266,
    /** Page down                  */ PageDown        = 267,
    /** Home                       */ Home            = 268,
    /** End                        */ End             = 269,
    /** Caps lock                  */ CapsLock        = 280,
    /** Scroll down                */ ScrollLock      = 281,
    /** Num lock                   */ NumLock         = 282,
    /** Print screen               */ PrintScreen     = 283,
    /** Pause                      */ Pause           = 284,
    /** F1                         */ F1              = 290,
    /** F2                         */ F2              = 291,
    /** F3                         */ F3              = 292,
    /** F4                         */ F4              = 293,
    /** F5                         */ F5              = 294,
    /** F6                         */ F6              = 295,
    /** F7                         */ F7              = 296,
    /** F8                         */ F8              = 297,
    /** F9                         */ F9              = 298,
    /** F10                        */ F10             = 299,
    /** F11                        */ F11             = 300,
    /** F12                        */ F12             = 301,
    /** Shift left                 */ LeftShift       = 340,
    /** Control left               */ LeftControl     = 341,
    /** Alt left                   */ LeftAlt         = 342,
    /** Super left                 */ LeftSuper       = 343,
    /** Shift right                */ RightShift      = 344,
    /** Control right              */ RightControl    = 345,
    /** Alt right                  */ RightAlt        = 346,
    /** Super right                */ RightSuper      = 347,
    /** KB menu                    */ KbMenu          = 348,

    // Keypad keys

    /** Keypad 0                   */ Kp0             = 320,
    /** Keypad 1                   */ Kp1             = 321,
    /** Keypad 2                   */ Kp2             = 322,
    /** Keypad 3                   */ Kp3             = 323,
    /** Keypad 4                   */ Kp4             = 324,
    /** Keypad 5                   */ Kp5             = 325,
    /** Keypad 6                   */ Kp6             = 326,
    /** Keypad 7                   */ Kp7             = 327,
    /** Keypad 8                   */ Kp8             = 328,
    /** Keypad 9                   */ Kp9             = 329,
    /** Keypad .                   */ KpDecimal       = 330,
    /** Keypad /                   */ KpDivide        = 331,
    /** Keypad *                   */ KpMultiply      = 332,
    /** Keypad -                   */ KpSubtract      = 333,
    /** Keypad +                   */ KpAdd           = 334,
    /** Keypad Enter               */ KpEnter         = 335,
    /** Keypad =                   */ KpEqual         = 336,

    // Android key buttons

    /** Android back button        */ Back            =   4,
    /** Android menu button        */ Menu            =   5,
    /** Android volume up button   */ VolumeUp        =  24,
    /** Android volume down button */ VolumeDown      =  25,
}
const _: () = assert!(std::mem::size_of::<KeyboardKey>() == std::mem::size_of::<Option<KeyboardKey>>());

/// Mouse buttons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    /// Mouse button left
    Left,
    /// Mouse button right
    Right,
    /// Mouse button middle (pressed wheel)
    Middle,
    /// Mouse button side (advanced mouse device)
    Side,
    /// Mouse button extra (advanced mouse device)
    Extra,
    /// Mouse button forward (advanced mouse device)
    Forward,
    /// Mouse button back (advanced mouse device)
    Back,
}

// Mouse cursor
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MouseCursor {
    /// Default pointer shape
    #[default]
    Default,
    /// Arrow shape
    Arrow,
    /// Text writing cursor shape
    Ibeam,
    /// Cross shape
    Crosshair,
    /// Pointing hand cursor
    PointingHand,
    /// Horizontal resize/move arrow shape
    ResizeEw,
    /// Vertical resize/move arrow shape
    ResizeNs,
    /// Top-left to bottom-right diagonal resize/move arrow shape
    ResizeNwse,
    /// The top-right to bottom-left diagonal resize/move arrow shape
    ResizeNesw,
    /// The omnidirectional resize/move cursor shape
    ResizeAll,
    /// The operation-not-allowed shape
    NotAllowed,
}

pub type GamepadID = usize;

/// Gamepad buttons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadButton {
    /// Gamepad left DPAD up button
    LeftFaceUp,
    /// Gamepad left DPAD right button
    LeftFaceRight,
    /// Gamepad left DPAD down button
    LeftFaceDown,
    /// Gamepad left DPAD left button
    LeftFaceLeft,
    /// Gamepad right button up (i.e. PS3: Triangle, Xbox: Y)
    RightFaceUp,
    /// Gamepad right button right (i.e. PS3: Circle, Xbox: B)
    RightFaceRight,
    /// Gamepad right button down (i.e. PS3: Cross, Xbox: A)
    RightFaceDown,
    /// Gamepad right button left (i.e. PS3: Square, Xbox: X)
    RightFaceLeft,
    /// Gamepad top/back trigger left (first), it could be a trailing button
    LeftTrigger1,
    /// Gamepad top/back trigger left (second), it could be a trailing button
    LeftTrigger2,
    /// Gamepad top/back trigger right (first), it could be a trailing button
    RightTrigger1,
    /// Gamepad top/back trigger right (second), it could be a trailing button
    RightTrigger2,
    /// Gamepad center buttons, left one (i.e. PS3: Select)
    MiddleLeft,
    /// Gamepad center buttons, middle one (i.e. PS3: PS, Xbox: XBOX)
    Middle,
    /// Gamepad center buttons, right one (i.e. PS3: Start)
    MiddleRight,
    /// Gamepad joystick pressed button left
    LeftThumb,
    /// Gamepad joystick pressed button right
    RightThumb,
}

/// Gamepad axis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadAxis {
    /// Gamepad left stick X axis
    LeftX,
    /// Gamepad left stick Y axis
    LeftY,
    /// Gamepad right stick X axis
    RightX,
    /// Gamepad right stick Y axis
    RightY,
    /// Gamepad back trigger left, pressure level: [1..-1]
    LeftTrigger,
    /// Gamepad back trigger right, pressure level: [1..-1]
    RightTrigger,
}
const _: () = assert!(std::mem::size_of::<GamepadAxis>() == std::mem::size_of::<Option<GamepadAxis>>());

bitflags! {
    /// Gesture
    /// NOTE: Provided as bit-wise flags to enable only desired gestures
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct Gesture: usize {
        /// Tap gesture
        const Tap        =   1;
        /// Double tap gesture
        const Doubletap  =   2;
        /// Hold gesture
        const Hold       =   4;
        /// Drag gesture
        const Drag       =   8;
        /// Swipe right gesture
        const SwipeRight =  16;
        /// Swipe left gesture
        const SwipeLeft  =  32;
        /// Swipe up gesture
        const SwipeUp    =  64;
        /// Swipe down gesture
        const SwipeDown  = 128;
        /// Pinch in gesture
        const PinchIn    = 256;
        /// Pinch out gesture
        const PinchOut   = 512;
    }
}

#[derive(Debug)]
pub struct Keyboard {
    /// Default exit key
    pub(crate) exit_key: Option<KeyboardKey>,
    /// Registers current frame key state
    pub(crate) current_key_state: [u8; MAX_KEYBOARD_KEYS],
    /// Registers previous frame key state
    pub(crate) previous_key_state: [u8; MAX_KEYBOARD_KEYS],

    /// NOTE: Since key press logic involves comparing prev vs cur key state, we need to handle key repeats specially
    /// Registers key repeats for current frame
    pub(crate) key_repeat_in_frame: [u8; MAX_KEYBOARD_KEYS],

    /// Input keys queue
    pub(crate) key_pressed_queue: ArrayVec<Option<KeyboardKey>, MAX_KEY_PRESSED_QUEUE>,

    /// Input characters queue (unicode)
    pub(crate) char_pressed_queue: ArrayVec<char, MAX_CHAR_PRESSED_QUEUE>,
}

impl Default for Keyboard {
    fn default() -> Self {
        Self {
            exit_key: Default::default(),
            current_key_state: [Default::default(); MAX_KEYBOARD_KEYS],
            previous_key_state: [Default::default(); MAX_KEYBOARD_KEYS],
            key_repeat_in_frame: [Default::default(); MAX_KEYBOARD_KEYS],
            key_pressed_queue: Default::default(),
            char_pressed_queue: Default::default(),
        }
    }
}

impl Keyboard {
    /// Maximum number of keyboard keys supported
    pub const MAX_KEYS: usize = MAX_KEYBOARD_KEYS;
    /// Maximum number of keys in the key input queue
    pub const MAX_KEY_PRESSED_QUEUE: usize = MAX_KEY_PRESSED_QUEUE;
    /// Maximum number of characters in the char input queue
    pub const MAX_CHAR_PRESSED_QUEUE: usize = MAX_CHAR_PRESSED_QUEUE;
}

#[derive(Debug, Default)]
pub struct Mouse {
    /// Mouse offset
    pub(crate) offset: Offset2,
    /// Mouse scaling
    pub(crate) scale: Vector2,
    /// Mouse position on screen
    pub(crate) current_position: Vector2,
    /// Previous mouse position
    pub(crate) previous_position: Vector2,

    /// Tracks current mouse cursor
    pub(crate) cursor: MouseCursor,
    /// Track if cursor is hidden
    pub(crate) is_cursor_hidden: bool,
    /// Tracks if cursor is inside client area
    pub(crate) is_cursor_on_screen: bool,

    /// Registers current mouse button state
    pub(crate) current_button_state: [u8; MAX_MOUSE_BUTTONS],
    /// Registers previous mouse button state
    pub(crate) previous_button_state: [u8; MAX_MOUSE_BUTTONS],
    /// Registers current mouse wheel variation
    pub(crate) current_wheel_move: Vector2,
    /// Registers previous mouse wheel variation
    pub(crate) previous_wheel_move: Vector2,
}

impl Mouse {
    /// Maximum number of mouse buttons supported
    pub const MAX_BUTTONS: usize = MAX_MOUSE_BUTTONS;
}

#[derive(Debug, Default)]
pub struct TouchPoint {
    /// Point identifiers
    pub(crate) point_id: u32,
    /// Touch position on screen
    pub(crate) position: Vector2,
    /// Registers current touch state
    pub(crate) current_touch_state: char,
    /// Registers previous touch state
    pub(crate) previous_touch_state: char,
}

#[derive(Debug, Default)]
pub struct Touch {
    pub(crate) items: ArrayVec<TouchPoint, MAX_TOUCH_POINTS>,
}

impl Touch {
    /// Maximum number of touch points supported
    pub const MAX: usize = MAX_TOUCH_POINTS;
}

/// Maximum number of bytes in a gamepad name
pub const MAX_GAMEPAD_NAME_LEN: usize = 64;

#[derive(Debug, Default)]
pub struct Gamepad {
    /// Flag to know if gamepad is ready
    pub(crate) ready: bool,
    /// Register number of available gamepad axis
    pub(crate) axis_count: u32,
    /// Gamepad name holder
    pub(crate) name: ArrayString<MAX_GAMEPAD_NAME_LEN>,
    /// Current gamepad buttons state
    pub(crate) current_button_state: [u8; MAX_GAMEPAD_BUTTONS],
    /// Previous gamepad buttons state
    pub(crate) previous_button_state: [u8; MAX_GAMEPAD_BUTTONS],
    /// Gamepad axis state
    pub(crate) axis_state: [f32; MAX_GAMEPAD_AXIS], // NOT dynamic
}

impl Gamepads {
    /// Maximum number of axis supported (per gamepad)
    pub const MAX_AXIS: usize = MAX_GAMEPAD_AXIS;
    /// Maximum number of buttons supported (per gamepad)
    pub const MAX_BUTTONS: usize = MAX_GAMEPAD_BUTTONS;
    /// Maximum vibration time in seconds
    pub const MAX_VIBRATION_TIME: f32 = MAX_GAMEPAD_VIBRATION_TIME;
}

#[derive(Debug, Default)]
pub struct Gamepads {
    /// Register last gamepad button pressed
    pub(crate) last_button_pressed: Option<GamepadButton>,
    /// Gamepad array
    pub(crate) items: ArrayVec<Gamepad, MAX_GAMEPADS>,
}

impl Gamepads {
    /// Maximum number of gamepads supported
    pub const MAX: usize = MAX_GAMEPADS;
}

#[derive(Debug, Default)]
pub struct Input {
    pub keyboard: Keyboard,
    pub mouse: Mouse,
    pub touch: Touch,
    pub gamepad: Gamepads,
}
