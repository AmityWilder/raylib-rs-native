use std::io::Write;
use crate::*;

mod tracelog_statics {
    use crate::*;
    use std::sync::{atomic::{AtomicU8, Ordering}, Mutex, MutexGuard};

    /// Minimum log type level
    static LOG_TYPE_LEVEL: AtomicU8 = AtomicU8::new(TraceLogLevel::Info as u8);

    pub fn log_type_level() -> TraceLogLevel {
        match LOG_TYPE_LEVEL.load(Ordering::Relaxed) {
            0 => TraceLogLevel::All,
            1 => TraceLogLevel::Trace,
            2 => TraceLogLevel::Debug,
            3 => TraceLogLevel::Info,
            4 => TraceLogLevel::Warning,
            5 => TraceLogLevel::Error,
            6 => TraceLogLevel::Fatal,
            7 => TraceLogLevel::None,
            _ => unreachable!("UB has occurred and LOG_TYPE_LEVEL has been assigned with an invalid state"),
        }
    }

    pub fn set_log_type_level(level: TraceLogLevel) {
        let level = level as u8;
        assert!((0..=7).contains(&level), "UB has occurred and a TraceLogLevel has been passed to set_log_type_level in an invalid state");
        LOG_TYPE_LEVEL.store(level, Ordering::Relaxed);
    }

    /// Logging: Redirect trace log messages
    pub type TraceLogCallback = Box<dyn FnMut(TraceLogType, std::fmt::Arguments<'_>) + 'static + Sync + Send>;

    /// TraceLog callback function pointer
    static TRACE_LOG: Mutex<Option<TraceLogCallback>> = Mutex::new(None);

    pub(super) fn trace_log_fn() -> MutexGuard<'static, Option<TraceLogCallback>> {
        match TRACE_LOG.lock() {
            Ok(lock) => lock,
            Err(e) => {
                let mut lock = e.into_inner();
                *lock = None;
                TRACE_LOG.clear_poison();
                #[cfg(feature = "support_tracelog")]
                print!("INFO: TRACELOG: Poisoned tracelog callback function removed"); // uses println to avoid unchecked recursion
                lock
            }
        }
    }

    pub fn set_trace_log_fn<F: FnMut(TraceLogType, std::fmt::Arguments<'_>) + 'static + Sync + Send>(callback: F) -> Option<TraceLogCallback> {
        let old = trace_log_fn().replace(Box::new(callback));
        tracelog!(Info, "TRACELOG: Updated tracelog callback function");
        old
    }

    pub fn clear_trace_log_fn() -> Option<TraceLogCallback> {
        let old = trace_log_fn().take();
        tracelog!(Info, "TRACELOG: Removed tracelog callback function");
        old
    }
}
pub use tracelog_statics::*;

#[doc(hidden)]
pub fn trace_log(log_type: TraceLogType, args: std::fmt::Arguments<'_>) {
    #[cfg(feature = "support_tracelog")] {
        // Message has level below current threshold, don't emit
        if log_type < log_type_level() { return; }

        if let Some(callback) = trace_log_fn().as_mut() {
            callback(log_type, args);
            return;
        }

        let prefix = match log_type {
            TraceLogType::Trace   => &b"TRACE: "[..],
            TraceLogType::Debug   => &b"DEBUG: "[..],
            TraceLogType::Info    => &b"INFO: "[..],
            TraceLogType::Warning => &b"WARNING: "[..],
            TraceLogType::Error   => &b"ERROR: "[..],
            TraceLogType::Fatal   => &b"FATAL: "[..],
        };

        {
            let mut stdout = std::io::stdout().lock();
            _ = stdout.write_all(prefix);
            _ = stdout.write_fmt(args);
            _ = stdout.write(b"\n");
            _ = stdout.flush();
        }

        if log_type == TraceLogType::Fatal { panic!() }  // If fatal logging, exit program
    }
}

/// Show trace log messages (Info, Warning, Error, Debug)
#[macro_export]
macro_rules! tracelog {
    ($level:ident, $($args:tt)+) => {
        $crate::utils::trace_log($crate::TraceLogType::$level, format_args!($($args)+))
    };

    ($level:expr, $($args:tt)+) => {
        $crate::utils::trace_log($level, format_args!($($args)+))
    };
}
