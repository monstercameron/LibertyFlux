//! `lf-platform`: the operating-system interface.
//!
//! README for future lanes:
//! - Every other crate talks to the OS through the traits defined here.
//!   Nothing else in the workspace touches OS APIs directly, and
//!   `cfg(target_os)` (or `cfg(unix)` / `cfg(windows)`) appears nowhere
//!   outside this crate.
//! - The traits are platform-agnostic and object-safe: engine code holds a
//!   `&dyn Files` or `Box<dyn AudioOutput>` and never names a backend.
//! - [`files`] is the streaming file interface: asynchronous reads with
//!   request handles, priorities, cancellation, completion polling or
//!   callbacks, block-aligned reads and a whole-file convenience. It ships
//!   two implementations: [`files::ThreadedFiles`] (portable, std threads,
//!   real asynchronous reads on every OS) and [`files::MemoryFiles`]
//!   (deterministic, in memory, for tests).
//! - [`audio`] is the audio output interface: device enumeration, a
//!   pull-model stream whose callback fills interleaved `f32` frames, and
//!   sample-rate and channel negotiation. It ships a null device and an
//!   in-memory capture device for tests. There is no OS audio backend yet:
//!   it needs a library such as SDL3, which is a download and waits for
//!   approval. [`audio`] documents where it plugs in.
//! - [`Window`], [`Input`], [`Threads`] and [`Time`] are still placeholder
//!   shapes; they change when the windowing backend (SDL3, phase 5) lands.
//! - Async file reads must be real async (see the pitfalls table in
//!   `plan.md`): the std implementation reads on worker threads and every
//!   result travels through the completion path, never a synchronous
//!   shortcut.

pub mod audio;
pub mod files;

pub use audio::{AudioOutput, AudioStream};
pub use files::Files;

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
