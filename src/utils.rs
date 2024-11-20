use std::io::Write;

/// Trace log level
/// NOTE: Organized by priority level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TraceLogLevel {
    /// Display all logs
    All,
    /// Trace logging, intended for internal use only
    Trace,
    /// Debug logging, used for internal debugging, it should be disabled on release builds
    Debug,
    /// Info logging, used for program execution info
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

#[macro_export]
macro_rules! tracelog {
    ($level:expr, $fmt:literal $(, $args:expr)* $(,)?) => {
        if cfg!(feature = "support_tracelog") {
            trace_log($level, format_args!($fmt, $($args),*));
        }
    };
}

#[macro_export]
macro_rules! tracelogd {
    ($fmt:literal $(, $args:expr)* $(,)?) => {
        tracelog!(TraceLogLevel::Debug, $fmt, $($args),*)
    };
}

// Callbacks to be implemented by users
pub type TraceLogCallback = dyn FnMut(TraceLogLevel, std::fmt::Arguments);

/// Minimum log type level
static mut LOG_TYPE_LEVEL: TraceLogLevel = TraceLogLevel::Info;
/// Log type that exits
static mut LOG_FATAL: TraceLogLevel = TraceLogLevel::Error;
/// TraceLog callback function pointer
static mut TRACE_LOG: Option<Box<TraceLogCallback>> = None;

/// Set the current threshold (minimum) log level
pub fn set_trace_log_level(log_type: TraceLogLevel) {
    unsafe { LOG_TYPE_LEVEL = log_type; }
}

// Set the exit threshold (minimum) log level
pub fn set_trace_log_fatal(log_type: TraceLogLevel) {
    unsafe { LOG_FATAL = log_type; }
}

/// Set custom trace log
pub fn set_trace_log_callback(callback: impl FnMut(TraceLogLevel, std::fmt::Arguments) + 'static) {
    unsafe { TRACE_LOG = Some(Box::new(callback)); }
}

/// Max length of one trace-log message
pub const MAX_TRACELOG_MSG_LENGTH: usize = 256;

// Show trace log messages (LOG_INFO, LOG_WARNING, LOG_ERROR, LOG_DEBUG)
pub fn trace_log(log_type: TraceLogLevel, args: std::fmt::Arguments) {
    if cfg!(feature = "support_tracelog_debug") {
        // Message has level below current threshold, don't emit
        if log_type < unsafe { LOG_TYPE_LEVEL } {
            println!("below threshold");
            return;
        }

        if let Some(callback) = unsafe { TRACE_LOG.as_mut() } {
            println!("custom callback");
            (callback)(log_type, args);
            return;
        }

        if cfg!(platform_android) {
            todo!()
            // match log_type {
            //     TraceLogLevel::Trace   => __android_log_vprint(ANDROID_LOG_VERBOSE, "raylib", text, args),
            //     TraceLogLevel::Debug   => __android_log_vprint(ANDROID_LOG_DEBUG, "raylib", text, args),
            //     TraceLogLevel::Info    => __android_log_vprint(ANDROID_LOG_INFO, "raylib", text, args),
            //     TraceLogLevel::Warning => __android_log_vprint(ANDROID_LOG_WARN, "raylib", text, args),
            //     TraceLogLevel::Error   => __android_log_vprint(ANDROID_LOG_ERROR, "raylib", text, args),
            //     TraceLogLevel::Fatal   => __android_log_vprint(ANDROID_LOG_FATAL, "raylib", text, args),
            //     _ => (),
            // }
        } else {
            println!("normal");
            let mut stdout = std::io::stdout();
            match log_type {
                TraceLogLevel::Trace   => _ = stdout.write(b"TRACE: "  ),
                TraceLogLevel::Debug   => _ = stdout.write(b"DEBUG: "  ),
                TraceLogLevel::Info    => _ = stdout.write(b"INFO: "   ),
                TraceLogLevel::Warning => _ = stdout.write(b"WARNING: "),
                TraceLogLevel::Error   => _ = stdout.write(b"ERROR: "  ),
                TraceLogLevel::Fatal   => _ = stdout.write(b"FATAL: "  ),
                _ => (),
            };
            _ = stdout.write_fmt(args);
            _ = std::io::stdout().flush();
        }

        // If fatal logging, exit program
        if log_type == unsafe { LOG_FATAL } {
            println!("fatal");
            std::process::exit(1);
        }
    } // SUPPORT_TRACELOG
}
