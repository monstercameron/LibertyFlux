//! [`MemoryFiles`]: a deterministic in-memory [`Files`] backend for tests.

use std::collections::HashMap;
use std::io;
use std::path::Path;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::time::Duration;

use super::core::{Core, Job, check_rooted};
use super::{
    CancelOutcome, Completion, CompletionCallback, DEFAULT_BLOCK_SIZE, FileBytes, FileHandle,
    Files, OpenFile, Priority, ReadError, ReadRequest, RequestId, RequestStatus, SubmitError,
};

/// The file object behind a handle: the bytes and the normalised path (the
/// key for injected failures).
#[derive(Clone)]
struct MemFile {
    key: String,
    bytes: Arc<[u8]>,
}

struct Inner {
    core: Core<MemFile>,
    files: HashMap<String, Arc<[u8]>>,
    failures: HashMap<String, io::ErrorKind>,
    executed: Vec<RequestId>,
}

/// An in-memory file backend with fully deterministic scheduling.
///
/// Files are added with [`MemoryFiles::insert`]. Reads never happen on their
/// own: a queued request runs only when the test calls
/// [`MemoryFiles::step`] or [`MemoryFiles::run_until_idle`], or when engine
/// code blocks in [`Files::wait`] / [`Files::wait_any`], which run queued
/// requests (in priority order) until what they wait for is ready. The
/// timeouts of those two methods are ignored: nothing here can block.
/// Callbacks run inside the call that performed the read, after the backend
/// lock is released, so they may submit new requests.
///
/// Paths follow the rooted rule (relative, no `..`); backslashes and
/// slashes are treated alike, case is significant.
pub struct MemoryFiles {
    inner: Mutex<Inner>,
}

impl Default for MemoryFiles {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryFiles {
    /// An empty backend with [`DEFAULT_BLOCK_SIZE`].
    #[must_use]
    pub fn new() -> Self {
        Self::with_block_size(DEFAULT_BLOCK_SIZE)
    }

    /// An empty backend with the given block size.
    ///
    /// # Panics
    ///
    /// Panics if `block_size` is not a power of two.
    #[must_use]
    pub fn with_block_size(block_size: u32) -> Self {
        assert!(
            block_size.is_power_of_two(),
            "block size must be a power of two"
        );
        MemoryFiles {
            inner: Mutex::new(Inner {
                core: Core::new(block_size),
                files: HashMap::new(),
                failures: HashMap::new(),
                executed: Vec::new(),
            }),
        }
    }

    fn lock(&self) -> MutexGuard<'_, Inner> {
        self.inner.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Normalises a path to its map key, applying the rooted rule.
    fn key(path: &Path) -> io::Result<String> {
        check_rooted(path)?;
        Ok(path.to_string_lossy().replace('\\', "/"))
    }

    /// Adds or replaces a file. Handles already open keep the old bytes.
    ///
    /// # Errors
    ///
    /// Returns [`io::ErrorKind::InvalidInput`] for a path that breaks the
    /// rooted rule.
    pub fn insert(&self, path: impl AsRef<Path>, bytes: impl Into<Arc<[u8]>>) -> io::Result<()> {
        let key = Self::key(path.as_ref())?;
        self.lock().files.insert(key, bytes.into());
        Ok(())
    }

    /// Makes every later read of `path` (through any handle, open or not
    /// yet open) complete with an I/O error of `kind`, to test error paths.
    ///
    /// # Errors
    ///
    /// Returns [`io::ErrorKind::InvalidInput`] for a path that breaks the
    /// rooted rule.
    pub fn fail_reads(&self, path: impl AsRef<Path>, kind: io::ErrorKind) -> io::Result<()> {
        let key = Self::key(path.as_ref())?;
        self.lock().failures.insert(key, kind);
        Ok(())
    }

    /// Runs the next queued request (highest priority, oldest first).
    /// Returns `false` when nothing was queued.
    pub fn step(&self) -> bool {
        let callback = {
            let mut inner = self.lock();
            let Some(job) = inner.core.take_job() else {
                return false;
            };
            let id = job.id;
            let failure = inner.failures.get(&job.file.key).copied();
            let result = read_job(job, failure);
            inner.executed.push(id);
            inner.core.finish(id, result)
        };
        run_callback(callback);
        true
    }

    /// Runs queued requests until none is left (including any submitted by
    /// callbacks meanwhile). Returns how many ran.
    pub fn run_until_idle(&self) -> usize {
        let mut n = 0;
        while self.step() {
            n += 1;
        }
        n
    }

    /// Ids of every request that has been read, in the order they ran.
    #[must_use]
    pub fn executed(&self) -> Vec<RequestId> {
        self.lock().executed.clone()
    }
}

/// Performs a job's read from memory.
fn read_job(job: Job<MemFile>, failure: Option<io::ErrorKind>) -> Result<Vec<u8>, ReadError> {
    if let Some(kind) = failure {
        return Err(ReadError::from_io(&io::Error::new(
            kind,
            "injected failure (MemoryFiles::fail_reads)",
        )));
    }
    let n = job.readable_len()?;
    // readable_len has checked offset <= size, and size is the slice length.
    let start = usize::try_from(job.offset).map_err(|_| ReadError::OutOfRange {
        offset: job.offset,
        size: job.size,
    })?;
    let mut buffer = job.buffer;
    buffer.extend_from_slice(&job.file.bytes[start..start + n]);
    Ok(buffer)
}

/// Calls a callback returned by [`Core::finish`], outside any lock.
fn run_callback(callback: Option<(CompletionCallback, Completion)>) {
    if let Some((callback, completion)) = callback {
        callback(completion);
    }
}

impl Files for MemoryFiles {
    fn open(&self, path: &Path) -> io::Result<OpenFile> {
        let key = Self::key(path)?;
        let mut inner = self.lock();
        let bytes = inner.files.get(&key).cloned().ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, format!("no in-memory file {key}"))
        })?;
        let size = bytes.len() as u64;
        Ok(inner.core.open(MemFile { key, bytes }, size))
    }

    fn close(&self, file: FileHandle) -> bool {
        self.lock().core.close(file)
    }

    fn block_size(&self) -> u32 {
        self.lock().core.block_size()
    }

    fn submit(&self, request: ReadRequest) -> Result<RequestId, SubmitError> {
        self.lock().core.submit(request)
    }

    fn cancel(&self, id: RequestId) -> CancelOutcome {
        self.lock().core.cancel(id)
    }

    fn set_priority(&self, id: RequestId, priority: Priority) -> bool {
        self.lock().core.set_priority(id, priority)
    }

    fn status(&self, id: RequestId) -> Option<RequestStatus> {
        self.lock().core.status(id)
    }

    fn poll_completions(&self, out: &mut Vec<Completion>) -> usize {
        self.lock().core.poll_completions(out)
    }

    fn take_completion(&self, id: RequestId) -> Option<Completion> {
        self.lock().core.take_completion(id)
    }

    fn wait_any(&self, _timeout: Duration) -> bool {
        loop {
            if self.lock().core.has_completions() {
                return true;
            }
            if !self.step() {
                return false;
            }
        }
    }

    fn wait(&self, id: RequestId, _timeout: Duration) -> Option<RequestStatus> {
        loop {
            let status = self.lock().core.status(id);
            match status {
                Some(RequestStatus::Queued | RequestStatus::Running) => {
                    if !self.step() {
                        return status;
                    }
                }
                other => return other,
            }
        }
    }

    fn read_whole(&self, path: &Path) -> io::Result<FileBytes> {
        let key = Self::key(path)?;
        let inner = self.lock();
        if let Some(kind) = inner.failures.get(&key) {
            return Err(io::Error::new(
                *kind,
                "injected failure (MemoryFiles::fail_reads)",
            ));
        }
        inner
            .files
            .get(&key)
            .cloned()
            .map(FileBytes::new)
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::NotFound, format!("no in-memory file {key}"))
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::files::{Alignment, Delivery};
    use std::sync::atomic::{AtomicUsize, Ordering};

    const FOREVER: Duration = Duration::from_secs(1);

    fn backend() -> (MemoryFiles, OpenFile) {
        let files = MemoryFiles::with_block_size(16);
        let bytes: Vec<u8> = (0..100u8).collect();
        files.insert("data/a.bin", bytes).unwrap();
        let open = files.open(Path::new("data/a.bin")).unwrap();
        (files, open)
    }

    #[test]
    fn nothing_happens_until_stepped() {
        let (files, open) = backend();
        assert_eq!(open.size, 100);
        let id = files.submit(ReadRequest::new(open.handle, 10, 5)).unwrap();
        assert_eq!(files.status(id), Some(RequestStatus::Queued));
        let mut out = Vec::new();
        assert_eq!(files.poll_completions(&mut out), 0);
        assert!(files.step());
        assert_eq!(files.status(id), Some(RequestStatus::Finished));
        assert_eq!(files.poll_completions(&mut out), 1);
        assert_eq!(out[0].result, Ok(vec![10, 11, 12, 13, 14]));
        assert_eq!(out[0].requested, 5);
        assert_eq!(files.status(id), None);
        assert!(!files.step());
    }

    #[test]
    fn priority_then_submission_order() {
        let (files, open) = backend();
        let h = open.handle;
        let submit = |offset, priority| {
            files
                .submit(ReadRequest::new(h, offset, 1).priority(priority))
                .unwrap()
        };
        let low = submit(0, Priority::Low);
        let normal_first = submit(1, Priority::Normal);
        let urgent = submit(2, Priority::Urgent);
        let normal_second = submit(3, Priority::Normal);
        let background = submit(4, Priority::Background);
        // Promote the low request above the normal ones; it keeps its
        // place among other high requests by submission order.
        assert!(files.set_priority(low, Priority::High));
        assert_eq!(files.run_until_idle(), 5);
        assert_eq!(
            files.executed(),
            vec![urgent, low, normal_first, normal_second, background]
        );
        assert!(
            !files.set_priority(low, Priority::Low),
            "finished requests cannot move"
        );
    }

    #[test]
    fn cancel_outcomes_follow_the_lifecycle() {
        let (files, open) = backend();
        let a = files.submit(ReadRequest::new(open.handle, 0, 4)).unwrap();
        let b = files.submit(ReadRequest::new(open.handle, 4, 4)).unwrap();
        assert_eq!(files.cancel(b), CancelOutcome::Cancelled);
        assert_eq!(files.status(b), None);
        assert_eq!(files.cancel(b), CancelOutcome::Unknown);
        files.run_until_idle();
        assert_eq!(
            files.executed(),
            vec![a],
            "a cancelled request is never read"
        );
        assert_eq!(files.cancel(a), CancelOutcome::Finished);
        assert!(files.take_completion(a).is_some());
        assert_eq!(files.cancel(a), CancelOutcome::Unknown);
        assert_eq!(files.cancel(RequestId(999)), CancelOutcome::Unknown);
    }

    #[test]
    fn end_of_file_rules() {
        let (files, open) = backend();
        let h = open.handle;
        let short = files.submit(ReadRequest::new(h, 96, 10)).unwrap();
        let at_end_empty = files.submit(ReadRequest::new(h, 100, 0)).unwrap();
        let at_end = files.submit(ReadRequest::new(h, 100, 1)).unwrap();
        let past = files.submit(ReadRequest::new(h, 101, 0)).unwrap();
        files.run_until_idle();
        assert_eq!(
            files.take_completion(short).unwrap().result,
            Ok(vec![96, 97, 98, 99])
        );
        assert_eq!(
            files.take_completion(at_end_empty).unwrap().result,
            Ok(vec![])
        );
        let oor = ReadError::OutOfRange {
            offset: 100,
            size: 100,
        };
        assert_eq!(files.take_completion(at_end).unwrap().result, Err(oor));
        assert!(matches!(
            files.take_completion(past).unwrap().result,
            Err(ReadError::OutOfRange { offset: 101, .. })
        ));
    }

    #[test]
    fn block_alignment_is_enforced() {
        let (files, open) = backend();
        let h = open.handle;
        assert_eq!(files.block_size(), 16);
        assert!(
            files
                .submit(ReadRequest::new(h, 16, 32).block_aligned())
                .is_ok()
        );
        assert_eq!(
            files
                .submit(ReadRequest::new(h, 8, 16).block_aligned())
                .unwrap_err(),
            SubmitError::Misaligned {
                offset: 8,
                len: 16,
                block_size: 16
            }
        );
        assert!(
            files
                .submit(ReadRequest::new(h, 16, 15).block_aligned())
                .is_err()
        );
        // The same range is fine without the contract.
        assert!(files.submit(ReadRequest::new(h, 8, 16)).is_ok());
        let r = ReadRequest::new(h, 0, 0);
        assert_eq!(r.alignment, Alignment::Any);
    }

    #[test]
    fn submit_errors() {
        let (files, open) = backend();
        assert_eq!(
            files
                .submit(ReadRequest::new(FileHandle(77), 0, 1))
                .unwrap_err(),
            SubmitError::UnknownFile(FileHandle(77))
        );
        assert_eq!(
            files
                .submit(ReadRequest::new(open.handle, 0, usize::MAX))
                .unwrap_err(),
            SubmitError::TooLarge(usize::MAX)
        );
    }

    #[test]
    fn callbacks_receive_completions_and_may_submit() {
        let (files, open) = backend();
        let files = Arc::new(files);
        let seen = Arc::new(Mutex::new(Vec::new()));
        let (f2, s2) = (Arc::clone(&files), Arc::clone(&seen));
        let h = open.handle;
        let id = files
            .submit(ReadRequest::new(h, 0, 2).tag(9).on_complete(move |done| {
                s2.lock().unwrap().push((done.tag, done.result.clone()));
                // Chain a follow-up read from inside the callback.
                let s3 = Arc::clone(&s2);
                f2.submit(ReadRequest::new(h, 50, 1).tag(10).on_complete(move |d| {
                    s3.lock().unwrap().push((d.tag, d.result));
                }))
                .unwrap();
            }))
            .unwrap();
        assert_eq!(files.run_until_idle(), 2);
        assert_eq!(files.status(id), None, "callback requests are forgotten");
        let mut out = Vec::new();
        assert_eq!(
            files.poll_completions(&mut out),
            0,
            "nothing queued for poll"
        );
        assert_eq!(
            *seen.lock().unwrap(),
            vec![(9, Ok(vec![0, 1])), (10, Ok(vec![50]))]
        );
    }

    #[test]
    fn wait_runs_requests_in_priority_order() {
        let (files, open) = backend();
        let h = open.handle;
        let urgent = files
            .submit(ReadRequest::new(h, 0, 1).priority(Priority::Urgent))
            .unwrap();
        let low = files
            .submit(ReadRequest::new(h, 1, 1).priority(Priority::Low))
            .unwrap();
        let normal = files.submit(ReadRequest::new(h, 2, 1)).unwrap();
        assert_eq!(files.wait(normal, FOREVER), Some(RequestStatus::Finished));
        assert_eq!(files.executed(), vec![urgent, normal]);
        assert_eq!(files.status(low), Some(RequestStatus::Queued));
        assert!(files.wait_any(FOREVER));
        assert_eq!(
            files.read_blocking(ReadRequest::new(h, 99, 1), FOREVER),
            Ok(vec![99])
        );
    }

    #[test]
    fn close_cancels_queued_requests() {
        let (files, open) = backend();
        let calls = Arc::new(AtomicUsize::new(0));
        let c2 = Arc::clone(&calls);
        let id = files
            .submit(ReadRequest::new(open.handle, 0, 1).on_complete(move |_| {
                c2.fetch_add(1, Ordering::SeqCst);
            }))
            .unwrap();
        assert!(files.close(open.handle));
        assert!(!files.close(open.handle));
        assert_eq!(files.status(id), None);
        assert_eq!(files.run_until_idle(), 0);
        assert_eq!(
            calls.load(Ordering::SeqCst),
            0,
            "a cancelled callback never runs"
        );
        assert!(files.submit(ReadRequest::new(open.handle, 0, 1)).is_err());
    }

    #[test]
    fn injected_failures_and_paths() {
        let (files, open) = backend();
        files
            .fail_reads("data\\a.bin", io::ErrorKind::PermissionDenied)
            .unwrap();
        let id = files.submit(ReadRequest::new(open.handle, 0, 1)).unwrap();
        files.run_until_idle();
        assert!(matches!(
            files.take_completion(id).unwrap().result,
            Err(ReadError::Io {
                kind: io::ErrorKind::PermissionDenied,
                ..
            })
        ));
        assert!(files.read_whole(Path::new("data/a.bin")).is_err());
        assert_eq!(
            files.open(Path::new("../a.bin")).unwrap_err().kind(),
            io::ErrorKind::InvalidInput
        );
        assert_eq!(
            files.open(Path::new("/a.bin")).unwrap_err().kind(),
            io::ErrorKind::InvalidInput
        );
        assert_eq!(
            files.open(Path::new("missing")).unwrap_err().kind(),
            io::ErrorKind::NotFound
        );
        files.insert("b.txt", b"hello".to_vec()).unwrap();
        assert_eq!(
            files.read_whole(Path::new("b.txt")).unwrap().as_bytes(),
            b"hello"
        );
    }

    #[test]
    fn buffers_are_recycled() {
        let (files, open) = backend();
        let buf = Vec::with_capacity(4096);
        let ptr = buf.as_ptr();
        let id = files
            .submit(ReadRequest::new(open.handle, 0, 8).buffer(buf))
            .unwrap();
        files.run_until_idle();
        let data = files.take_completion(id).unwrap().result.unwrap();
        assert_eq!(data.len(), 8);
        assert!(data.capacity() >= 4096);
        assert_eq!(data.as_ptr(), ptr, "the same allocation comes back");
        let _ = Delivery::Poll;
    }
}
