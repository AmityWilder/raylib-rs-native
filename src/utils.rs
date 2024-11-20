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

// Callbacks to be implemented by users
pub type TraceLogCallback = dyn FnMut(TraceLogLevel, std::fmt::Arguments);

pub struct TraceLog {
    /// Minimum log type level
    pub log_threshold: TraceLogLevel,
    /// Minimum log type that exits
    pub log_fatal: TraceLogLevel,
    /// TraceLog callback function pointer
    logger: Option<&'static mut TraceLogCallback>,
}

impl Default for TraceLog {
    fn default() -> Self {
        Self {
            log_threshold: TraceLogLevel::Info,
            log_fatal: TraceLogLevel::Error,
            logger: None,
        }
    }
}

impl TraceLog {
    /// Set custom trace log
    pub fn set_trace_log_callback(&mut self, callback: &'static mut impl FnMut(TraceLogLevel, std::fmt::Arguments)) {
        self.logger = Some(callback);
    }

    /// Max length of one trace-log message
    // const MAX_MSG_LENGTH: usize = 256;

    // Show trace log messages (LOG_INFO, LOG_WARNING, LOG_ERROR, LOG_DEBUG)
    pub fn trace_log(&mut self, log_type: TraceLogLevel, args: std::fmt::Arguments) {
        if cfg!(feature = "support_tracelog") {
            // Message has level below current threshold, don't emit
            if log_type < self.log_threshold {
                return;
            }

            if let Some(callback) = self.logger.as_mut() {
                (callback)(log_type, args);
                return;
            }

            if cfg!(target_os="android") {
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
                let mut cout = std::io::stdout();
                match log_type {
                    TraceLogLevel::Trace   => _ = cout.write(b"TRACE: "  ),
                    TraceLogLevel::Debug   => _ = cout.write(b"DEBUG: "  ),
                    TraceLogLevel::Info    => _ = cout.write(b"INFO: "   ),
                    TraceLogLevel::Warning => _ = cout.write(b"WARNING: "),
                    TraceLogLevel::Error   => _ = cout.write(b"ERROR: "  ),
                    TraceLogLevel::Fatal   => _ = cout.write(b"FATAL: "  ),
                    _ => (),
                };
                _ = cout.write_fmt(args);
                _ = cout.write(b"\n");
                _ = cout.flush();
            }

            // If fatal logging, exit program
            if log_type == self.log_fatal {
                std::process::exit(1);
            }
        }
    }
}

#[macro_export]
macro_rules! tracelog {
    ($log:expr, $level:ident, $($args:expr),* $(,)?) => {
        $log.trace_log(TraceLogLevel::$level, format_args!($($args),*));
    };
}

#[macro_export]
macro_rules! tracelogd {
    ($log:expr, $($args:expr),* $(,)?) => {
        tracelog!($log, Debug, $($args),*)
    };
}
