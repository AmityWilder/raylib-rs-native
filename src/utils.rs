use std::{io::Write, sync::{Mutex, MutexGuard, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard}};
use crate::*;

/// Minimum log type level
static LOG_TYPE_LEVEL: RwLock<TraceLogLevel> = RwLock::new(TraceLogLevel::Info);

pub fn log_type_level() -> Result<TraceLogLevel, PoisonError<TraceLogLevel>> {
    match LOG_TYPE_LEVEL.read() {
        Ok(x) => Ok(*x),
        Err(e) => Err(PoisonError::new(*e.into_inner())),
    }
}

pub fn set_log_type_level<'a>(level: TraceLogLevel) -> Result<(), PoisonError<RwLockWriteGuard<'a, TraceLogLevel>>> {
    match LOG_TYPE_LEVEL.write() {
        Ok(mut x) => Ok(*x = level),
        Err(e) => Err(e),
    }
}

/// Logging: Redirect trace log messages
pub type TraceLogCallback = Option<Box<dyn FnMut(TraceLogType, std::fmt::Arguments<'_>) + Send + Sync>>;

/// TraceLog callback function pointer
static TRACE_LOG: Mutex<TraceLogCallback> = Mutex::new(None);

pub fn trace_log_fn<'a>() -> Result<MutexGuard<'a, TraceLogCallback>, PoisonError<MutexGuard<'a, TraceLogCallback>>> {
    TRACE_LOG.lock()
}

pub fn set_trace_log_fn<'a>(callback: TraceLogCallback) -> Result<(), PoisonError<MutexGuard<'a, TraceLogCallback>>> {
    match TRACE_LOG.lock() {
        Ok(mut x) => Ok(*x = callback),
        Err(e) => Err(e),
    }
}

#[doc(hidden)]
pub fn trace_log(log_type: TraceLogType, args: std::fmt::Arguments<'_>) {
    #[cfg(feature = "support_tracelog")] {
        // Message has level below current threshold, don't emit
        if log_type < log_type_level().unwrap() { return; }

        let mut lock = trace_log_fn().unwrap();
        if let Some(callback) = &mut *lock {
            (callback)(log_type, args);
            return;
        }
        drop(lock);

        let prefix = match log_type {
            TraceLogType::Trace   => b"TRACE: "  .as_slice(),
            TraceLogType::Debug   => b"DEBUG: "  .as_slice(),
            TraceLogType::Info    => b"INFO: "   .as_slice(),
            TraceLogType::Warning => b"WARNING: ".as_slice(),
            TraceLogType::Error   => b"ERROR: "  .as_slice(),
            TraceLogType::Fatal   => b"FATAL: "  .as_slice(),
        };

        let mut stdout = std::io::stdout().lock();
        _ = stdout.write_all(prefix);
        _ = stdout.write_fmt(args);
        _ = stdout.write(b"\n");
        _ = stdout.flush();
        drop(stdout);

        if log_type == TraceLogType::Fatal { panic!() }  // If fatal logging, exit program
    }
}

/// Show trace log messages (LOG_INFO, LOG_WARNING, LOG_ERROR, LOG_DEBUG)
#[macro_export]
macro_rules! tracelog {
    ($level:expr, $($args:tt)+) => {
        $crate::utils::trace_log($level, format_args!($($args)+))
    };
}
