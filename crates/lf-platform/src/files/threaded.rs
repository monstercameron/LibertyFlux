//! [`ThreadedFiles`]: the portable [`Files`] backend, built on std threads.

use std::fs::File;
use std::io;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use super::core::{Core, Job, check_rooted};
use super::{
    CancelOutcome, Completion, DEFAULT_BLOCK_SIZE, FileBytes, FileHandle, Files, OpenFile,
    Priority, ReadError, ReadRequest, RequestId, RequestStatus, SubmitError,
};

/// Default number of reader threads.
pub const DEFAULT_WORKERS: usize = 2;

/// State shared between the handle and its reader threads.
struct Shared {
    core: Mutex<Core<Arc<File>>>,
    /// Signalled when a request is queued or shutdown begins.
    work: Condvar,
    /// Signalled when a request finishes.
    done: Condvar,
    /// Callbacks that panicked (contained so the reader survives).
    callback_panics: AtomicU64,
}

impl Shared {
    fn lock(&self) -> MutexGuard<'_, Core<Arc<File>>> {
        self.core.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

/// A [`Files`] backend that reads on a pool of std threads.
///
/// Works on every OS the standard library supports. Each reader thread takes
/// the highest-priority queued request, performs a positioned read on the
/// shared file object (so readers never contend on a file cursor) and
/// records the completion; callbacks run on the reader thread after the
/// backend lock is released. Reads go through the OS page cache: the block
/// alignment contract is enforced so engine code is ready for an
/// unbuffered backend, but this backend does not need it.
///
/// With a root, paths follow the rooted rule (relative, no `..`) and resolve
/// under the root; without one ([`ThreadedFiles::unrooted`], for tools) they
/// are used as given.
///
/// Dropping the backend stops the readers: running reads finish (and their
/// callbacks run), queued requests are dropped without completions.
pub struct ThreadedFiles {
    shared: Arc<Shared>,
    workers: Vec<JoinHandle<()>>,
    root: Option<PathBuf>,
}

impl ThreadedFiles {
    /// A backend rooted at `root`, with [`DEFAULT_WORKERS`] readers and
    /// [`DEFAULT_BLOCK_SIZE`].
    ///
    /// # Errors
    ///
    /// Returns the OS error if a reader thread cannot be spawned.
    pub fn new(root: impl Into<PathBuf>) -> io::Result<Self> {
        Self::with_options(Some(root.into()), DEFAULT_WORKERS, DEFAULT_BLOCK_SIZE)
    }

    /// A backend without a root: paths are used exactly as given.
    ///
    /// # Errors
    ///
    /// Returns the OS error if a reader thread cannot be spawned.
    pub fn unrooted() -> io::Result<Self> {
        Self::with_options(None, DEFAULT_WORKERS, DEFAULT_BLOCK_SIZE)
    }

    /// A backend with explicit settings. `workers` is clamped to at least 1.
    ///
    /// # Errors
    ///
    /// Returns [`io::ErrorKind::InvalidInput`] if `block_size` is not a power
    /// of two, or the OS error if a reader thread cannot be spawned.
    pub fn with_options(
        root: Option<PathBuf>,
        workers: usize,
        block_size: u32,
    ) -> io::Result<Self> {
        if !block_size.is_power_of_two() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "block size must be a power of two",
            ));
        }
        let shared = Arc::new(Shared {
            core: Mutex::new(Core::new(block_size)),
            work: Condvar::new(),
            done: Condvar::new(),
            callback_panics: AtomicU64::new(0),
        });
        let mut this = ThreadedFiles {
            shared,
            workers: Vec::new(),
            root,
        };
        for index in 0..workers.max(1) {
            let shared = Arc::clone(&this.shared);
            let handle = std::thread::Builder::new()
                .name(format!("lf-io-{index}"))
                .spawn(move || reader_loop(&shared))?;
            // On a spawn error `this` drops here and stops earlier readers.
            this.workers.push(handle);
        }
        Ok(this)
    }

    /// Number of reader threads.
    #[must_use]
    pub fn worker_count(&self) -> usize {
        self.workers.len()
    }

    /// How many completion callbacks have panicked. A panic is contained
    /// (the reader keeps serving) but counted, so tests can assert zero.
    #[must_use]
    pub fn callback_panics(&self) -> u64 {
        self.shared.callback_panics.load(Ordering::Relaxed)
    }

    fn resolve(&self, path: &Path) -> io::Result<PathBuf> {
        match &self.root {
            Some(root) => {
                check_rooted(path)?;
                Ok(root.join(path))
            }
            None => Ok(path.to_path_buf()),
        }
    }
}

impl Drop for ThreadedFiles {
    fn drop(&mut self) {
        self.shared.lock().shut_down();
        self.shared.work.notify_all();
        for worker in self.workers.drain(..) {
            // A reader only ends by returning; a join error would mean a
            // panic outside the contained callback, which is a bug to see.
            if worker.join().is_err() {
                self.shared.callback_panics.fetch_add(1, Ordering::Relaxed);
            }
        }
    }
}

/// One reader thread: take a job, read it, record it, run its callback.
fn reader_loop(shared: &Shared) {
    loop {
        let job = {
            let mut core = shared.lock();
            loop {
                if core.is_shut_down() {
                    return;
                }
                if let Some(job) = core.take_job() {
                    break job;
                }
                core = shared
                    .work
                    .wait(core)
                    .unwrap_or_else(PoisonError::into_inner);
            }
        };
        let id = job.id;
        let result = read_job(job);
        let callback = {
            let mut core = shared.lock();
            let callback = core.finish(id, result);
            shared.done.notify_all();
            callback
        };
        if let Some((callback, completion)) = callback
            && catch_unwind(AssertUnwindSafe(move || callback(completion))).is_err()
        {
            shared.callback_panics.fetch_add(1, Ordering::Relaxed);
        }
    }
}

/// Performs a job's read with positioned reads.
fn read_job(job: Job<Arc<File>>) -> Result<Vec<u8>, ReadError> {
    let n = job.readable_len()?;
    let mut buffer = job.buffer;
    buffer.resize(n, 0);
    let mut filled = 0;
    while filled < n {
        let offset = job.offset + filled as u64;
        match read_at(&job.file, &mut buffer[filled..], offset) {
            // The file shrank since it was opened: return what exists.
            Ok(0) => break,
            Ok(got) => filled += got,
            Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
            Err(e) => return Err(ReadError::from_io(&e)),
        }
    }
    buffer.truncate(filled);
    Ok(buffer)
}

/// Positioned read: Unix `pread`.
#[cfg(unix)]
fn read_at(file: &File, buf: &mut [u8], offset: u64) -> io::Result<usize> {
    std::os::unix::fs::FileExt::read_at(file, buf, offset)
}

/// Positioned read: Windows `ReadFile` with an offset. It also moves the
/// file cursor, which nothing here uses.
#[cfg(windows)]
fn read_at(file: &File, buf: &mut [u8], offset: u64) -> io::Result<usize> {
    std::os::windows::fs::FileExt::seek_read(file, buf, offset)
}

/// Positioned read fallback: seek then read, serialised by one lock.
#[cfg(not(any(unix, windows)))]
fn read_at(file: &File, buf: &mut [u8], offset: u64) -> io::Result<usize> {
    use std::io::{Read, Seek, SeekFrom};
    static CURSOR: Mutex<()> = Mutex::new(());
    let _guard = CURSOR.lock().unwrap_or_else(PoisonError::into_inner);
    let mut f = file;
    f.seek(SeekFrom::Start(offset))?;
    f.read(buf)
}

impl Files for ThreadedFiles {
    fn open(&self, path: &Path) -> io::Result<OpenFile> {
        let resolved = self.resolve(path)?;
        let file = File::open(&resolved)?;
        let meta = file.metadata()?;
        if !meta.is_file() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} is not a regular file", path.display()),
            ));
        }
        Ok(self.shared.lock().open(Arc::new(file), meta.len()))
    }

    fn close(&self, file: FileHandle) -> bool {
        self.shared.lock().close(file)
    }

    fn block_size(&self) -> u32 {
        self.shared.lock().block_size()
    }

    fn submit(&self, request: ReadRequest) -> Result<RequestId, SubmitError> {
        let id = self.shared.lock().submit(request)?;
        self.shared.work.notify_one();
        Ok(id)
    }

    fn cancel(&self, id: RequestId) -> CancelOutcome {
        self.shared.lock().cancel(id)
    }

    fn set_priority(&self, id: RequestId, priority: Priority) -> bool {
        self.shared.lock().set_priority(id, priority)
    }

    fn status(&self, id: RequestId) -> Option<RequestStatus> {
        self.shared.lock().status(id)
    }

    fn poll_completions(&self, out: &mut Vec<Completion>) -> usize {
        self.shared.lock().poll_completions(out)
    }

    fn take_completion(&self, id: RequestId) -> Option<Completion> {
        self.shared.lock().take_completion(id)
    }

    fn wait_any(&self, timeout: Duration) -> bool {
        let deadline = Instant::now() + timeout;
        let mut core = self.shared.lock();
        loop {
            if core.has_completions() {
                return true;
            }
            let now = Instant::now();
            if now >= deadline {
                return false;
            }
            core = self
                .shared
                .done
                .wait_timeout(core, deadline - now)
                .unwrap_or_else(PoisonError::into_inner)
                .0;
        }
    }

    fn wait(&self, id: RequestId, timeout: Duration) -> Option<RequestStatus> {
        let deadline = Instant::now() + timeout;
        let mut core = self.shared.lock();
        loop {
            let status = core.status(id);
            if !matches!(status, Some(RequestStatus::Queued | RequestStatus::Running)) {
                return status;
            }
            let now = Instant::now();
            if now >= deadline {
                return status;
            }
            core = self
                .shared
                .done
                .wait_timeout(core, deadline - now)
                .unwrap_or_else(PoisonError::into_inner)
                .0;
        }
    }

    fn read_whole(&self, path: &Path) -> io::Result<FileBytes> {
        let resolved = self.resolve(path)?;
        Ok(FileBytes::new(std::fs::read(resolved)?.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;

    const TIMEOUT: Duration = Duration::from_secs(10);

    /// A unique scratch folder under the system temporary directory,
    /// removed on drop.
    struct TempDir(PathBuf);

    impl TempDir {
        fn new(label: &str) -> Self {
            static COUNTER: AtomicUsize = AtomicUsize::new(0);
            let n = COUNTER.fetch_add(1, Ordering::SeqCst);
            let dir = std::env::temp_dir().join(format!(
                "lf-platform-test-{}-{label}-{n}",
                std::process::id()
            ));
            std::fs::create_dir_all(&dir).unwrap();
            TempDir(dir)
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn pattern(len: usize) -> Vec<u8> {
        (0..len)
            .map(|i| u8::try_from(i * 7 % 251).unwrap())
            .collect()
    }

    fn setup(label: &str, len: usize) -> (TempDir, ThreadedFiles, OpenFile) {
        let dir = TempDir::new(label);
        std::fs::create_dir_all(dir.0.join("sub")).unwrap();
        std::fs::write(dir.0.join("sub/data.bin"), pattern(len)).unwrap();
        let files = ThreadedFiles::new(&dir.0).unwrap();
        let open = files.open(Path::new("sub/data.bin")).unwrap();
        (dir, files, open)
    }

    #[test]
    fn reads_complete_through_the_poll_queue() {
        let (_dir, files, open) = setup("poll", 10_000);
        assert_eq!(open.size, 10_000);
        assert_eq!(files.worker_count(), DEFAULT_WORKERS);
        let want = pattern(10_000);
        let mut ids = Vec::new();
        for i in 0..50u64 {
            let req = ReadRequest::new(open.handle, i * 150, 300).tag(i);
            ids.push(files.submit(req).unwrap());
        }
        let mut got = Vec::new();
        while got.len() < ids.len() {
            assert!(files.wait_any(TIMEOUT), "reads did not complete");
            files.poll_completions(&mut got);
        }
        for done in &got {
            let start = usize::try_from(done.tag * 150).unwrap();
            assert_eq!(done.result.as_deref(), Ok(&want[start..start + 300]));
            assert_eq!(files.status(done.id), None);
        }
    }

    #[test]
    fn short_reads_and_out_of_range() {
        let (_dir, files, open) = setup("eof", 100);
        let h = open.handle;
        assert_eq!(
            files.read_blocking(ReadRequest::new(h, 90, 50), TIMEOUT),
            Ok(pattern(100)[90..].to_vec())
        );
        assert_eq!(
            files.read_blocking(ReadRequest::new(h, 100, 1), TIMEOUT),
            Err(ReadError::OutOfRange {
                offset: 100,
                size: 100
            })
        );
        assert_eq!(
            files.read_blocking(ReadRequest::new(h, 100, 0), TIMEOUT),
            Ok(vec![])
        );
    }

    #[test]
    fn block_aligned_reads() {
        let (_dir, files, open) = setup("aligned", 8192);
        let h = open.handle;
        let bs = files.block_size();
        assert_eq!(bs, DEFAULT_BLOCK_SIZE);
        let data = files
            .read_blocking(
                ReadRequest::new(h, u64::from(bs), bs as usize).block_aligned(),
                TIMEOUT,
            )
            .unwrap();
        assert_eq!(data, pattern(8192)[bs as usize..2 * bs as usize].to_vec());
        assert!(matches!(
            files.submit(ReadRequest::new(h, 1, bs as usize).block_aligned()),
            Err(SubmitError::Misaligned { .. })
        ));
    }

    #[test]
    fn callbacks_run_on_reader_threads() {
        let (_dir, files, open) = setup("callback", 4096);
        let (tx, rx) = std::sync::mpsc::channel();
        for i in 0..8u64 {
            let tx = tx.clone();
            files
                .submit(
                    ReadRequest::new(open.handle, i * 512, 512)
                        .tag(i)
                        .on_complete(move |d| {
                            let name = std::thread::current().name().unwrap_or("").to_string();
                            tx.send((d.tag, d.result.map(|v| v.len()), name)).unwrap();
                        }),
                )
                .unwrap();
        }
        let mut tags = Vec::new();
        for _ in 0..8 {
            let (tag, len, thread) = rx.recv_timeout(TIMEOUT).unwrap();
            assert_eq!(len, Ok(512));
            assert!(thread.starts_with("lf-io-"), "callback ran on {thread}");
            tags.push(tag);
        }
        tags.sort_unstable();
        assert_eq!(tags, (0..8).collect::<Vec<_>>());
        let mut out = Vec::new();
        assert_eq!(files.poll_completions(&mut out), 0);
        assert_eq!(files.callback_panics(), 0);
    }

    #[test]
    fn panicking_callback_is_contained() {
        let (_dir, files, open) = setup("panic", 64);
        files
            .submit(ReadRequest::new(open.handle, 0, 1).on_complete(|_| panic!("test panic")))
            .unwrap();
        // The reader survives and serves the next request.
        let after = files.read_blocking(ReadRequest::new(open.handle, 1, 1), TIMEOUT);
        assert_eq!(after, Ok(vec![7]));
        let deadline = Instant::now() + TIMEOUT;
        while files.callback_panics() == 0 && Instant::now() < deadline {
            std::thread::yield_now();
        }
        assert_eq!(files.callback_panics(), 1);
    }

    #[test]
    fn paths_and_whole_files() {
        let (dir, files, _open) = setup("paths", 32);
        assert_eq!(
            files
                .read_whole(Path::new("sub/data.bin"))
                .unwrap()
                .as_bytes(),
            &pattern(32)[..]
        );
        let escape = files.open(Path::new("../outside.bin")).unwrap_err();
        assert_eq!(escape.kind(), io::ErrorKind::InvalidInput);
        let absolute = dir.0.join("sub/data.bin");
        assert_eq!(
            files.open(&absolute).unwrap_err().kind(),
            io::ErrorKind::InvalidInput
        );
        // A directory is refused: by the regular-file check on Unix, by the
        // OS itself (access denied) on Windows.
        assert!(files.open(Path::new("sub")).is_err());
        assert_eq!(
            files.open(Path::new("missing.bin")).unwrap_err().kind(),
            io::ErrorKind::NotFound
        );
        let tool = ThreadedFiles::unrooted().unwrap();
        assert_eq!(tool.open(&absolute).unwrap().size, 32);
        assert!(ThreadedFiles::with_options(None, 1, 1000).is_err());
    }

    #[test]
    fn priorities_reorder_a_backlog() {
        // One reader, and the reader is held busy by a callback until every
        // request is queued, so the queue order is what decides.
        let dir = TempDir::new("prio");
        std::fs::write(dir.0.join("f.bin"), pattern(64)).unwrap();
        let files = ThreadedFiles::with_options(Some(dir.0.clone()), 1, 16).unwrap();
        let h = files.open(Path::new("f.bin")).unwrap().handle;
        let gate = Arc::new((Mutex::new(false), Condvar::new()));
        let g2 = Arc::clone(&gate);
        let held = files
            .submit(ReadRequest::new(h, 0, 1).on_complete(move |_| {
                let (lock, cv) = &*g2;
                let mut open = lock.lock().unwrap();
                while !*open {
                    open = cv.wait(open).unwrap();
                }
            }))
            .unwrap();
        // A callback request is forgotten once its callback starts, so the
        // reader is inside the gate when the status reads `None`.
        let deadline = Instant::now() + TIMEOUT;
        while files.status(held).is_some() && Instant::now() < deadline {
            std::thread::yield_now();
        }
        assert_eq!(files.status(held), None);
        let order = Arc::new(Mutex::new(Vec::new()));
        let mut ids = Vec::new();
        for (tag, prio) in [
            (1, Priority::Low),
            (2, Priority::Urgent),
            (3, Priority::Normal),
            (4, Priority::Urgent),
        ] {
            let o = Arc::clone(&order);
            ids.push(
                files
                    .submit(
                        ReadRequest::new(h, 1, 1)
                            .priority(prio)
                            .tag(tag)
                            .on_complete(move |d| {
                                o.lock().unwrap().push(d.tag);
                            }),
                    )
                    .unwrap(),
            );
        }
        // Cancel one queued request while the reader is held.
        assert_eq!(files.cancel(ids[2]), CancelOutcome::Cancelled);
        {
            let (lock, cv) = &*gate;
            *lock.lock().unwrap() = true;
            cv.notify_all();
        }
        let deadline = Instant::now() + TIMEOUT;
        while order.lock().unwrap().len() < 3 && Instant::now() < deadline {
            std::thread::yield_now();
        }
        assert_eq!(*order.lock().unwrap(), vec![2, 4, 1]);
    }

    #[test]
    fn drop_with_queued_work_returns() {
        let (_dir, files, open) = setup("drop", 1 << 16);
        for i in 0..200u64 {
            files
                .submit(ReadRequest::new(open.handle, i * 64, 4096))
                .unwrap();
        }
        drop(files);
    }
}
