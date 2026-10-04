//! `lf-platform`: the operating-system interface.
//!
//! README for future lanes:
//! - This crate defines traits only: [`Window`], [`Input`], [`AudioOutput`],
//!   [`Files`], [`Threads`] and [`Time`]. There is no implementation yet;
//!   per-OS implementations arrive in phase 5 behind these traits.
//! - Every other crate talks to the OS through these traits. Nothing else
//!   in the workspace touches OS APIs directly.
//! - Method signatures below are placeholders to pin the shape (object-safe
//!   traits, plain-data arguments). Expect them to change when the first
//!   implementation lands.
//! - Async file reads must be real async (see plan.md pitfalls); the
//!   [`Files`] trait will grow completion-based reads before phase 5 ends.

use std::io;
use std::path::Path;

/// A game window: title and size only for now.
pub trait Window {
    /// Sets the window title.
    fn set_title(&self, title: &str);
    /// Returns the drawable size in pixels as `(width, height)`.
    fn size(&self) -> (u32, u32);
}

/// An input event with an opaque device-specific code.
///
/// Codes stay opaque until the input crate defines real key/gamepad maps.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputEvent {
    /// The user asked the window to close.
    Quit,
    /// Anything else; the code is device-specific and unstable.
    Unknown(u32),
}

/// Polled input: keyboard, mouse and gamepad events.
pub trait Input {
    /// Returns the next pending event, or `None` when the queue is empty.
    fn poll_event(&self) -> Option<InputEvent>;
}

/// Audio output: accepts plain PCM frames for now.
pub trait AudioOutput {
    /// Submits interleaved stereo frames at the given sample rate.
    fn submit_stereo_f32(&self, frames: &[f32], sample_rate_hz: u32);
}

/// File access. Paths are game-relative unless documented otherwise.
pub trait Files {
    /// Reads a whole file into memory.
    ///
    /// # Errors
    ///
    /// Returns the underlying I/O error when the file cannot be read.
    fn read_file(&self, path: &Path) -> io::Result<Vec<u8>>;
}

/// Thread management.
pub trait Threads {
    /// Spawns a named thread and forgets it; used for worker threads.
    fn spawn_named(&self, name: &str, f: Box<dyn FnOnce() + Send + 'static>);
    /// Returns the number of usable CPU cores.
    fn cpu_count(&self) -> usize;
}

/// Clocks and sleeps.
pub trait Time {
    /// Milliseconds since an unspecified epoch (for deltas, not dates).
    fn now_millis(&self) -> u64;
    /// Sleeps at least `ms` milliseconds.
    fn sleep_millis(&self, ms: u64);
}
