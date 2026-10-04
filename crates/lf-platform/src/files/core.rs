//! Request bookkeeping shared by every [`Files`](super::Files) backend.
//!
//! [`Core`] holds the open files, the priority queue, the status of every
//! live request and the completions waiting to be collected. Backends wrap
//! it in a mutex and add only the part that differs: how a [`Job`] is read.
//! Keeping the state machine in one place means the deterministic test
//! backend exercises exactly the logic the threaded backend runs.

use std::cmp::Reverse;
use std::collections::{BTreeMap, HashMap, VecDeque};

use super::{
    Alignment, CancelOutcome, Completion, CompletionCallback, Delivery, FileHandle,
    MAX_REQUEST_LEN, OpenFile, Priority, ReadError, ReadRequest, RequestId, RequestStatus,
    SubmitError,
};

/// Queue key: highest priority first, then oldest submission first.
type QueueKey = (Reverse<Priority>, u64);

/// One open file: the backend's file object and its size at open time.
struct FileEntry<F> {
    file: F,
    size: u64,
}

/// Where a live request is.
enum State {
    Queued(QueueKey),
    Running,
    Finished,
}

/// A live request (queued, running, or finished and uncollected).
struct Entry {
    state: State,
    file: FileHandle,
    offset: u64,
    len: usize,
    tag: u64,
    /// The recycled buffer, until a job takes it.
    buffer: Option<Vec<u8>>,
    /// The delivery, until the request finishes.
    delivery: Option<Delivery>,
}

/// A request taken by a reader: everything needed to perform the read.
pub(super) struct Job<F> {
    /// The request being served.
    pub id: RequestId,
    /// The backend's file object.
    pub file: F,
    /// File size at open time.
    pub size: u64,
    /// Requested offset.
    pub offset: u64,
    /// Requested length.
    pub len: usize,
    /// Buffer to fill (cleared, capacity kept).
    pub buffer: Vec<u8>,
}

impl<F> Job<F> {
    /// How many bytes this job may read: the request clamped to the file,
    /// or the out-of-range error the completion must carry.
    pub fn readable_len(&self) -> Result<usize, ReadError> {
        let past_end = self.offset > self.size || (self.offset == self.size && self.len > 0);
        if past_end {
            return Err(ReadError::OutOfRange {
                offset: self.offset,
                size: self.size,
            });
        }
        let remaining = self.size - self.offset;
        Ok(usize::try_from(remaining).map_or(self.len, |r| r.min(self.len)))
    }
}

/// The shared state machine. See the module docs of [`super`].
pub(super) struct Core<F> {
    block_size: u32,
    next_file: u32,
    next_id: u64,
    next_seq: u64,
    files: HashMap<FileHandle, FileEntry<F>>,
    queue: BTreeMap<QueueKey, RequestId>,
    requests: HashMap<RequestId, Entry>,
    completions: VecDeque<Completion>,
    shut_down: bool,
}

impl<F: Clone> Core<F> {
    /// An empty core with the given block size (a power of two).
    pub fn new(block_size: u32) -> Self {
        Core {
            block_size,
            next_file: 1,
            next_id: 1,
            next_seq: 0,
            files: HashMap::new(),
            queue: BTreeMap::new(),
            requests: HashMap::new(),
            completions: VecDeque::new(),
            shut_down: false,
        }
    }

    /// The block size for aligned requests.
    pub fn block_size(&self) -> u32 {
        self.block_size
    }

    /// Registers an opened file and returns its handle.
    pub fn open(&mut self, file: F, size: u64) -> OpenFile {
        let handle = FileHandle(self.next_file);
        self.next_file = self.next_file.wrapping_add(1).max(1);
        self.files.insert(handle, FileEntry { file, size });
        OpenFile { handle, size }
    }

    /// Forgets a file and cancels its queued requests.
    pub fn close(&mut self, handle: FileHandle) -> bool {
        if self.files.remove(&handle).is_none() {
            return false;
        }
        let doomed: Vec<RequestId> = self
            .requests
            .iter()
            .filter(|(_, e)| e.file == handle && matches!(e.state, State::Queued(_)))
            .map(|(id, _)| *id)
            .collect();
        for id in doomed {
            self.cancel(id);
        }
        true
    }

    /// Marks the core as shutting down: later submits are refused.
    pub fn shut_down(&mut self) {
        self.shut_down = true;
    }

    /// True once [`Core::shut_down`] has been called.
    pub fn is_shut_down(&self) -> bool {
        self.shut_down
    }

    /// Validates and queues a request.
    pub fn submit(&mut self, request: ReadRequest) -> Result<RequestId, SubmitError> {
        if self.shut_down {
            return Err(SubmitError::ShutDown);
        }
        if !self.files.contains_key(&request.file) {
            return Err(SubmitError::UnknownFile(request.file));
        }
        if request.len > MAX_REQUEST_LEN {
            return Err(SubmitError::TooLarge(request.len));
        }
        if request.alignment == Alignment::Block {
            let block = u64::from(self.block_size);
            let len_ok = u64::try_from(request.len).is_ok_and(|l| l.is_multiple_of(block));
            if !request.offset.is_multiple_of(block) || !len_ok {
                return Err(SubmitError::Misaligned {
                    offset: request.offset,
                    len: request.len,
                    block_size: self.block_size,
                });
            }
        }
        let id = RequestId(self.next_id);
        self.next_id += 1;
        let key = (Reverse(request.priority), self.next_seq);
        self.next_seq += 1;
        self.queue.insert(key, id);
        self.requests.insert(
            id,
            Entry {
                state: State::Queued(key),
                file: request.file,
                offset: request.offset,
                len: request.len,
                tag: request.tag,
                buffer: Some(request.buffer),
                delivery: Some(request.delivery),
            },
        );
        Ok(id)
    }

    /// Takes the next request in priority order and marks it running.
    pub fn take_job(&mut self) -> Option<Job<F>> {
        loop {
            let (_, id) = self.queue.pop_first()?;
            let Some(entry) = self.requests.get_mut(&id) else {
                continue;
            };
            // A queued request's file is open: close cancels its requests.
            let Some(file) = self.files.get(&entry.file) else {
                continue;
            };
            entry.state = State::Running;
            let mut buffer = entry.buffer.take().unwrap_or_default();
            buffer.clear();
            return Some(Job {
                id,
                file: file.file.clone(),
                size: file.size,
                offset: entry.offset,
                len: entry.len,
                buffer,
            });
        }
    }

    /// Records a running request's result. A polled request's completion is
    /// queued; a callback request is forgotten and its callback returned, to
    /// be called by the backend after it releases its lock.
    pub fn finish(
        &mut self,
        id: RequestId,
        result: Result<Vec<u8>, ReadError>,
    ) -> Option<(CompletionCallback, Completion)> {
        let entry = self.requests.get_mut(&id)?;
        let completion = Completion {
            id,
            file: entry.file,
            offset: entry.offset,
            requested: entry.len,
            tag: entry.tag,
            result,
        };
        match entry.delivery.take().unwrap_or_default() {
            Delivery::Poll => {
                entry.state = State::Finished;
                self.completions.push_back(completion);
                None
            }
            Delivery::Callback(callback) => {
                self.requests.remove(&id);
                Some((callback, completion))
            }
        }
    }

    /// See [`super::Files::cancel`].
    pub fn cancel(&mut self, id: RequestId) -> CancelOutcome {
        let Some(entry) = self.requests.get(&id) else {
            return CancelOutcome::Unknown;
        };
        match entry.state {
            State::Queued(key) => {
                self.queue.remove(&key);
                self.requests.remove(&id);
                CancelOutcome::Cancelled
            }
            State::Running => CancelOutcome::Running,
            State::Finished => CancelOutcome::Finished,
        }
    }

    /// See [`super::Files::set_priority`]. The request keeps its original
    /// submission order within the new priority.
    pub fn set_priority(&mut self, id: RequestId, priority: Priority) -> bool {
        let Some(entry) = self.requests.get_mut(&id) else {
            return false;
        };
        let State::Queued(old) = entry.state else {
            return false;
        };
        self.queue.remove(&old);
        let key = (Reverse(priority), old.1);
        entry.state = State::Queued(key);
        self.queue.insert(key, id);
        true
    }

    /// See [`super::Files::status`].
    pub fn status(&self, id: RequestId) -> Option<RequestStatus> {
        self.requests.get(&id).map(|e| match e.state {
            State::Queued(_) => RequestStatus::Queued,
            State::Running => RequestStatus::Running,
            State::Finished => RequestStatus::Finished,
        })
    }

    /// True when a completion is waiting.
    pub fn has_completions(&self) -> bool {
        !self.completions.is_empty()
    }

    /// See [`super::Files::poll_completions`].
    pub fn poll_completions(&mut self, out: &mut Vec<Completion>) -> usize {
        let n = self.completions.len();
        for done in self.completions.drain(..) {
            self.requests.remove(&done.id);
            out.push(done);
        }
        n
    }

    /// See [`super::Files::take_completion`].
    pub fn take_completion(&mut self, id: RequestId) -> Option<Completion> {
        let at = self.completions.iter().position(|c| c.id == id)?;
        let done = self.completions.remove(at)?;
        self.requests.remove(&id);
        Some(done)
    }
}

/// Checks a path against the rooted-path rule: relative, and no `..`,
/// root or drive-prefix components. Used by every backend that has a root.
///
/// # Errors
///
/// Returns [`std::io::ErrorKind::InvalidInput`] for a refused path.
pub(super) fn check_rooted(path: &std::path::Path) -> std::io::Result<()> {
    use std::path::Component;
    let ok = path
        .components()
        .all(|c| matches!(c, Component::Normal(_) | Component::CurDir));
    if ok && path.components().next().is_some() {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!(
                "path {} must be relative to the root, without `..`",
                path.display()
            ),
        ))
    }
}
