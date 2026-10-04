//! Timestamped append-only file logger shared by loader and tools.
//!
//! One line per event, UTC wall-clock time, no dependencies. Safe to call
//! from any thread; before [`init`] (or if the file cannot be opened) all
//! macros are silent no-ops. Never call from `DllMain`: file I/O there can
//! deadlock the loader.

// The one unsafe block reads the system clock through the Win32 binding.
#![allow(unsafe_code)]

use crate::mem;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

static LOG: OnceLock<Logger> = OnceLock::new();

/// Append-only log handle. Used through the global [`init`]/[`info`] fns.
pub struct Logger {
    inner: Mutex<LoggerInner>,
}

struct LoggerInner {
    file: Option<File>,
    path: PathBuf,
}

impl Logger {
    fn new(path: PathBuf) -> Self {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .ok();
        Logger {
            inner: Mutex::new(LoggerInner { file, path }),
        }
    }

    /// Path this logger appends to.
    pub fn path(&self) -> PathBuf {
        self.inner.lock().unwrap().path.clone()
    }

    /// Append one stamped line. Lock or I/O failure drops the line silently.
    pub fn write(&self, level: &str, msg: &str) {
        let stamp = utc_stamp();
        let line = format!("[{stamp}] [{level}] {msg}\n");
        if let Ok(mut inner) = self.inner.lock()
            && let Some(f) = inner.file.as_mut()
        {
            let _ = f.write_all(line.as_bytes());
            let _ = f.flush();
        }
    }
}

fn utc_stamp() -> String {
    unsafe {
        let mut t = mem::SystemTime::default();
        mem::GetSystemTime(&raw mut t);
        format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
            t.year, t.month, t.day, t.hour, t.minute, t.second, t.millis
        )
    }
}

/// Open the global log file. First call wins; later calls are ignored.
pub fn init(path: PathBuf) {
    LOG.get_or_init(|| Logger::new(path));
}

/// Write one line at `level` to the global log (no-op before [`init`]).
pub fn log(level: &str, msg: &str) {
    if let Some(l) = LOG.get() {
        l.write(level, msg);
    }
}

/// Write one `info` line to the global log.
pub fn info(msg: &str) {
    log("info", msg);
}

/// Write one `warn` line to the global log.
pub fn warn(msg: &str) {
    log("warn", msg);
}

/// Write one `error` line to the global log.
pub fn error(msg: &str) {
    log("error", msg);
}

/// Path of the global log file, if [`init`] has run.
pub fn log_path() -> Option<PathBuf> {
    LOG.get().map(Logger::path)
}
