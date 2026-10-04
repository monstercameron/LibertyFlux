//! Streaming file access: asynchronous reads designed for the engine's
//! streaming system.
//!
//! # Model
//!
//! The streaming system opens a file once (an archive, usually) and then
//! asks for byte ranges of it. Each range is one [`ReadRequest`]. Submitting
//! a request returns a [`RequestId`] at once; the read itself happens later,
//! on whatever thread or OS facility the backend uses, and its result comes
//! back as a [`Completion`]. Nothing about a read is ever returned from
//! [`Files::submit`] itself, so the completion path is exercised on every
//! read (the recompilation projects hid bugs by serving async reads
//! synchronously; see the pitfalls table in `plan.md`).
//!
//! # Request lifecycle
//!
//! A request is in exactly one [`RequestStatus`] at a time:
//!
//! 1. `Queued`: accepted, waiting for a reader. Its priority can still be
//!    changed ([`Files::set_priority`]) and it can still be cancelled with a
//!    guarantee that no completion will ever appear ([`Files::cancel`]).
//! 2. `Running`: a reader has taken it. It can no longer be stopped; its
//!    completion will be delivered.
//! 3. `Finished`: its completion is waiting to be collected with
//!    [`Files::poll_completions`] or [`Files::take_completion`]. Collecting it
//!    forgets the request: [`Files::status`] then returns `None`.
//!
//! A request submitted with [`Delivery::Callback`] skips step 3: its callback
//! receives the completion, on a backend thread, and the request is
//! forgotten as soon as the callback returns.
//!
//! # Ordering
//!
//! Queued requests are started highest [`Priority`] first and, within one
//! priority, in submission order. A backend with several readers may finish
//! them out of order; nothing promises completion order.
//!
//! # Block-aligned reads
//!
//! Every backend has a [`Files::block_size`]: the granularity of its fastest
//! reads (an unbuffered OS read needs sector-aligned offsets and lengths; the
//! game's archives store payloads in 2048-byte blocks). A request marked
//! [`Alignment::Block`] must have an offset and a length that are multiples
//! of the block size, or [`Files::submit`] rejects it with
//! [`SubmitError::Misaligned`]. Use [`align_range`] to widen an arbitrary range
//! to blocks. Backends enforce the contract even when they could serve the
//! read anyway, so engine code stays correct on a backend that cannot.
//!
//! # Ranges and end of file
//!
//! A read may run past the end of the file: its data is then short (only the
//! bytes that exist). A read that starts past the end of the file, or starts
//! exactly at the end and asks for at least one byte, completes with
//! [`ReadError::OutOfRange`]. Both are delivered through the completion,
//! never as a submit error, because they are runtime facts, not programming
//! errors. A zero-length read completes with empty data.
//!
//! # Paths
//!
//! Paths are relative to the backend's root (the game folder, normally).
//! Backends reject absolute paths and `..` components when they have a root;
//! see each backend.
//!
//! # Implementations
//!
//! - [`ThreadedFiles`]: portable, std threads and positioned reads; works on
//!   every OS the standard library supports.
//! - [`MemoryFiles`]: deterministic, in memory; reads run only when the test
//!   steps the backend (or waits), so ordering is exact and repeatable.
//! - Future OS backends (an I/O ring on Linux, overlapped I/O on Windows,
//!   dispatch I/O on macOS) implement the same trait in this crate behind
//!   `cfg(target_os)`.

mod core;
mod memory;
mod threaded;

use std::fmt;
use std::io;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

pub use memory::MemoryFiles;
pub use threaded::{DEFAULT_WORKERS, ThreadedFiles};

/// Largest single read a backend accepts, in bytes (1 GiB).
///
/// Larger requests are rejected with [`SubmitError::TooLarge`]; nothing the
/// engine streams comes close.
pub const MAX_REQUEST_LEN: usize = 1 << 30;

/// The block size used when a backend is not told otherwise, in bytes.
///
/// 2048 matches the payload block size of the game's archives, so every
/// archive entry read is block aligned without widening.
pub const DEFAULT_BLOCK_SIZE: u32 = 2048;

/// An open file, as returned by [`Files::open`]. Plain data: copying it does
/// not duplicate the underlying file, and it stays valid until
/// [`Files::close`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FileHandle(pub u32);

/// What [`Files::open`] returns: the handle and the file's size when opened.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpenFile {
    /// Handle for requests against this file.
    pub handle: FileHandle,
    /// Size of the file in bytes at open time.
    pub size: u64,
}

/// Identifies one submitted request. Ids are never reused by a backend
/// instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RequestId(pub u64);

/// How urgent a request is. Higher priorities start first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum Priority {
    /// Speculative prefetch; read only when nothing else waits.
    Background,
    /// Low-priority streaming (distant world sectors).
    Low,
    /// Ordinary streaming.
    #[default]
    Normal,
    /// Needed soon (assets near the camera).
    High,
    /// Needed now: the game is blocked on it (a loading screen, a model the
    /// player is about to see).
    Urgent,
}

/// Alignment contract of a request (see the module docs).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Alignment {
    /// Any offset and length.
    #[default]
    Any,
    /// Offset and length must be multiples of [`Files::block_size`].
    Block,
}

/// Called with the completion of a request submitted with
/// [`Delivery::Callback`].
///
/// It runs on a backend thread ([`ThreadedFiles`]) or inside the call that
/// performed the read ([`MemoryFiles`]). It must be short: a slow callback
/// delays every read behind it. It may submit new requests.
pub type CompletionCallback = Box<dyn FnOnce(Completion) + Send + 'static>;

/// How a request's completion reaches the caller.
#[derive(Default)]
pub enum Delivery {
    /// The completion is queued for [`Files::poll_completions`] /
    /// [`Files::take_completion`].
    #[default]
    Poll,
    /// The callback receives the completion; nothing is queued.
    Callback(CompletionCallback),
}

impl fmt::Debug for Delivery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Delivery::Poll => f.write_str("Poll"),
            Delivery::Callback(_) => f.write_str("Callback(..)"),
        }
    }
}

/// One asynchronous read of `len` bytes at `offset` of `file`.
#[derive(Debug)]
pub struct ReadRequest {
    /// The file to read.
    pub file: FileHandle,
    /// Byte offset of the first byte to read.
    pub offset: u64,
    /// Number of bytes to read (at most [`MAX_REQUEST_LEN`]).
    pub len: usize,
    /// Scheduling priority.
    pub priority: Priority,
    /// Alignment contract.
    pub alignment: Alignment,
    /// Caller data returned untouched in the completion (a streaming slot
    /// index, for example).
    pub tag: u64,
    /// A buffer to reuse for the data. The backend clears it and returns it,
    /// filled, in the completion; its capacity is kept, so a caller that
    /// recycles buffers allocates nothing per read.
    pub buffer: Vec<u8>,
    /// Where the completion goes.
    pub delivery: Delivery,
}

impl ReadRequest {
    /// A normal-priority, unaligned, polled read with no tag.
    #[must_use]
    pub fn new(file: FileHandle, offset: u64, len: usize) -> Self {
        ReadRequest {
            file,
            offset,
            len,
            priority: Priority::Normal,
            alignment: Alignment::Any,
            tag: 0,
            buffer: Vec::new(),
            delivery: Delivery::Poll,
        }
    }

    /// Sets the priority.
    #[must_use]
    pub fn priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }

    /// Requires block alignment (see the module docs).
    #[must_use]
    pub fn block_aligned(mut self) -> Self {
        self.alignment = Alignment::Block;
        self
    }

    /// Sets the caller tag.
    #[must_use]
    pub fn tag(mut self, tag: u64) -> Self {
        self.tag = tag;
        self
    }

    /// Supplies a buffer to reuse.
    #[must_use]
    pub fn buffer(mut self, buffer: Vec<u8>) -> Self {
        self.buffer = buffer;
        self
    }

    /// Delivers the completion to `callback` instead of the poll queue.
    #[must_use]
    pub fn on_complete(mut self, callback: impl FnOnce(Completion) + Send + 'static) -> Self {
        self.delivery = Delivery::Callback(Box::new(callback));
        self
    }
}

/// Why a read failed. Delivered in [`Completion::result`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadError {
    /// The read started at or past the end of the file (an empty read at
    /// exactly the end is not an error).
    OutOfRange {
        /// Requested start offset.
        offset: u64,
        /// File size.
        size: u64,
    },
    /// The OS reported an error.
    Io {
        /// The error kind.
        kind: io::ErrorKind,
        /// The error's text.
        message: String,
    },
}

impl ReadError {
    /// Wraps an I/O error.
    #[must_use]
    pub fn from_io(err: &io::Error) -> Self {
        ReadError::Io {
            kind: err.kind(),
            message: err.to_string(),
        }
    }
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReadError::OutOfRange { offset, size } => {
                write!(
                    f,
                    "read at offset {offset} is past the end of a {size}-byte file"
                )
            }
            ReadError::Io { message, .. } => write!(f, "read failed: {message}"),
        }
    }
}

impl std::error::Error for ReadError {}

/// Why [`Files::submit`] refused a request. These are programming errors;
/// runtime failures arrive in the completion instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubmitError {
    /// The handle was never opened or is already closed.
    UnknownFile(FileHandle),
    /// [`Alignment::Block`] was asked for but the offset or length is not a
    /// multiple of the block size.
    Misaligned {
        /// Requested offset.
        offset: u64,
        /// Requested length.
        len: usize,
        /// The backend's block size.
        block_size: u32,
    },
    /// The length exceeds [`MAX_REQUEST_LEN`].
    TooLarge(usize),
    /// The backend is shutting down.
    ShutDown,
}

impl fmt::Display for SubmitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SubmitError::UnknownFile(h) => write!(f, "unknown file handle {}", h.0),
            SubmitError::Misaligned {
                offset,
                len,
                block_size,
            } => write!(
                f,
                "read of {len} bytes at {offset} is not aligned to {block_size}-byte blocks"
            ),
            SubmitError::TooLarge(len) => write!(f, "read of {len} bytes is too large"),
            SubmitError::ShutDown => f.write_str("the file backend is shutting down"),
        }
    }
}

impl std::error::Error for SubmitError {}

/// The result of one request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Completion {
    /// The request this completes.
    pub id: RequestId,
    /// The file it read.
    pub file: FileHandle,
    /// Requested offset.
    pub offset: u64,
    /// Requested length; the data may be shorter at end of file.
    pub requested: usize,
    /// The request's tag.
    pub tag: u64,
    /// The bytes read (in the request's recycled buffer), or the error.
    pub result: Result<Vec<u8>, ReadError>,
}

/// Where a request is in its lifecycle (see the module docs).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RequestStatus {
    /// Waiting for a reader; can be cancelled or re-prioritised.
    Queued,
    /// Being read; will complete.
    Running,
    /// Completion waiting to be collected.
    Finished,
}

/// What [`Files::cancel`] achieved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CancelOutcome {
    /// The request was still queued and is gone: no completion and no
    /// callback will ever appear for it.
    Cancelled,
    /// The request is being read; its completion will still be delivered.
    Running,
    /// The request already finished; its completion is waiting to be
    /// collected (cancel does not discard it).
    Finished,
    /// No such request: never submitted, already collected, or already
    /// delivered to its callback.
    Unknown,
}

/// The bytes of a whole file, shared and immutable.
///
/// Cloning is cheap (a reference count). A future OS backend may hand out a
/// memory mapping behind the same type; today both implementations hold
/// the bytes on the heap.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileBytes(Arc<[u8]>);

impl FileBytes {
    /// Wraps bytes already in memory.
    #[must_use]
    pub fn new(bytes: Arc<[u8]>) -> Self {
        FileBytes(bytes)
    }

    /// The file's bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Length in bytes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// True for an empty file.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl AsRef<[u8]> for FileBytes {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

/// Widens a byte range to whole blocks.
///
/// Returns `(aligned_offset, aligned_len, lead)`: read `aligned_len` bytes at
/// `aligned_offset`, then the wanted bytes start `lead` bytes into the data.
/// `block_size` must be non-zero. Returns `None` if the widened range would
/// overflow.
#[must_use]
pub fn align_range(offset: u64, len: usize, block_size: u32) -> Option<(u64, usize, usize)> {
    let block = u64::from(block_size.max(1));
    let lead = offset % block;
    let aligned_offset = offset - lead;
    let end = offset.checked_add(u64::try_from(len).ok()?)?;
    let aligned_end = end.checked_next_multiple_of(block)?;
    let aligned_len = usize::try_from(aligned_end - aligned_offset).ok()?;
    Some((aligned_offset, aligned_len, usize::try_from(lead).ok()?))
}

/// Streaming file access. See the module docs for the full contract.
///
/// Every method takes `&self`: implementations are shared between the
/// streaming thread, loaders and the main loop.
pub trait Files: Send + Sync {
    /// Opens a file for reading.
    ///
    /// # Errors
    ///
    /// Returns the OS error when the file cannot be opened, and
    /// [`io::ErrorKind::InvalidInput`] for a path the backend refuses.
    fn open(&self, path: &Path) -> io::Result<OpenFile>;

    /// Closes a file. Requests on it that are still queued are cancelled
    /// (no completion); running ones complete normally. Returns `false` for
    /// an unknown handle.
    fn close(&self, file: FileHandle) -> bool;

    /// The block size for [`Alignment::Block`] requests, in bytes; a power
    /// of two.
    fn block_size(&self) -> u32;

    /// Queues a read. Returns at once; the result arrives as a completion.
    ///
    /// # Errors
    ///
    /// Returns [`SubmitError`] for an unknown handle, a misaligned block
    /// request, an oversized request, or a backend that is shutting down.
    fn submit(&self, request: ReadRequest) -> Result<RequestId, SubmitError>;

    /// Cancels a request if it has not started.
    fn cancel(&self, id: RequestId) -> CancelOutcome;

    /// Changes the priority of a queued request. Returns `false` if the
    /// request is not queued (running, finished or unknown).
    fn set_priority(&self, id: RequestId, priority: Priority) -> bool;

    /// The request's status, or `None` once it is collected, cancelled,
    /// delivered to its callback, or was never submitted.
    fn status(&self, id: RequestId) -> Option<RequestStatus>;

    /// Moves every waiting completion into `out` (oldest first) and returns
    /// how many were moved. Never blocks.
    fn poll_completions(&self, out: &mut Vec<Completion>) -> usize;

    /// Removes and returns one request's completion if it is finished.
    /// Never blocks.
    fn take_completion(&self, id: RequestId) -> Option<Completion>;

    /// Blocks until at least one completion is waiting or `timeout`
    /// passes. Returns `true` if a completion is waiting.
    fn wait_any(&self, timeout: Duration) -> bool;

    /// Blocks until the request finishes or `timeout` passes, and returns
    /// its status then (`Finished` on success; `Queued` or `Running` on
    /// timeout; `None` for an unknown or callback-delivered request).
    fn wait(&self, id: RequestId, timeout: Duration) -> Option<RequestStatus>;

    /// Reads a whole file synchronously. This is the convenience path for
    /// small configuration files and tools; streamed assets go through
    /// [`Files::submit`].
    ///
    /// # Errors
    ///
    /// Returns the OS error, or [`io::ErrorKind::InvalidInput`] for a path
    /// the backend refuses.
    fn read_whole(&self, path: &Path) -> io::Result<FileBytes>;

    /// Submits a request and waits for it: a blocking read built only from
    /// the asynchronous methods, so it travels the same completion path.
    /// Any [`Delivery`] on the request is replaced by [`Delivery::Poll`].
    ///
    /// # Errors
    ///
    /// Returns the read's error; a refused submit or a timeout is reported
    /// as [`ReadError::Io`] with kind `InvalidInput` or `TimedOut`.
    fn read_blocking(
        &self,
        mut request: ReadRequest,
        timeout: Duration,
    ) -> Result<Vec<u8>, ReadError> {
        request.delivery = Delivery::Poll;
        let id = self.submit(request).map_err(|e| ReadError::Io {
            kind: io::ErrorKind::InvalidInput,
            message: e.to_string(),
        })?;
        if self.wait(id, timeout) == Some(RequestStatus::Finished) {
            return match self.take_completion(id) {
                Some(done) => done.result,
                None => Err(ReadError::Io {
                    kind: io::ErrorKind::NotFound,
                    message: "completion was collected by another caller".into(),
                }),
            };
        }
        // Do not leave a stale request behind on timeout.
        let _ = self.cancel(id);
        Err(ReadError::Io {
            kind: io::ErrorKind::TimedOut,
            message: "read did not finish in time".into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn align_range_widens_to_blocks() {
        assert_eq!(align_range(0, 2048, 2048), Some((0, 2048, 0)));
        assert_eq!(align_range(100, 10, 2048), Some((0, 2048, 100)));
        assert_eq!(align_range(2040, 16, 2048), Some((0, 4096, 2040)));
        assert_eq!(align_range(4096, 0, 2048), Some((4096, 0, 0)));
        assert_eq!(align_range(u64::MAX - 1, 10, 2048), None);
    }

    #[test]
    fn priorities_order_low_to_high() {
        assert!(Priority::Background < Priority::Low);
        assert!(Priority::Low < Priority::Normal);
        assert!(Priority::Normal < Priority::High);
        assert!(Priority::High < Priority::Urgent);
        assert_eq!(Priority::default(), Priority::Normal);
    }

    #[test]
    fn request_builder_sets_fields() {
        let r = ReadRequest::new(FileHandle(3), 4096, 16)
            .priority(Priority::High)
            .block_aligned()
            .tag(77)
            .buffer(Vec::with_capacity(64));
        assert_eq!(r.file, FileHandle(3));
        assert_eq!(r.priority, Priority::High);
        assert_eq!(r.alignment, Alignment::Block);
        assert_eq!(r.tag, 77);
        assert!(r.buffer.capacity() >= 64);
        assert!(matches!(r.delivery, Delivery::Poll));
        let r = r.on_complete(|_| {});
        assert_eq!(format!("{:?}", r.delivery), "Callback(..)");
    }
}
