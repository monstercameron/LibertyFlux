//! Owns pending streamed reads and joins platform completions to slot tags.
//!
//! The caller supplies the file range and priority. This module does not
//! derive ranges from streaming metadata or mutate load slots.

use std::collections::HashMap;

use lf_platform::files::{
    CancelOutcome, Completion, FileHandle, Files, Priority, ReadRequest, RequestId, SubmitError,
};

/// One completion drained from the platform backend.
///
/// `slot_tag` is present for a request submitted through this owner. A
/// completion with no tag belongs to another user of the same `Files`
/// backend; it is returned intact so polling here does not discard it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamReadCompletion {
    /// The streaming slot tag associated with this request, if this owner
    /// submitted it.
    pub slot_tag: Option<u64>,
    /// The platform completion, including its byte result or read error.
    pub completion: Completion,
}

/// Tracks each submitted request's file and slot tag through its lifecycle.
///
/// `poll_completions` drains the backend's shared completion queue. Use one
/// owner as the queue's poller, or route completions with `slot_tag: None` to
/// their other consumers. A tracked completion removes its request mapping,
/// so subsequent polls cannot deliver it as a tagged result again. Closing
/// through [`ReadOwner::close_file`] reconciles queued requests that the
/// backend cancels without delivering a completion. If another caller closes
/// a file directly, call [`ReadOwner::reconcile_pending`] or poll afterward.
pub struct ReadOwner<'files> {
    files: &'files dyn Files,
    pending: HashMap<RequestId, PendingRead>,
}

struct PendingRead {
    file: FileHandle,
    slot_tag: u64,
}

impl<'files> ReadOwner<'files> {
    /// Creates an empty owner backed by `files`.
    #[must_use]
    pub fn new(files: &'files dyn Files) -> Self {
        Self {
            files,
            pending: HashMap::new(),
        }
    }

    /// Submits a caller-supplied file range and remembers its slot tag.
    ///
    /// The request uses the platform default `Alignment::Any`; the supplied
    /// offset and length are passed through unchanged.
    ///
    /// # Errors
    ///
    /// Returns the backend's [`SubmitError`] without creating pending state.
    pub fn submit(
        &mut self,
        file: FileHandle,
        offset: u64,
        len: usize,
        priority: Priority,
        slot_tag: u64,
    ) -> Result<RequestId, SubmitError> {
        let request = ReadRequest::new(file, offset, len)
            .priority(priority)
            .tag(slot_tag);
        let id = self.files.submit(request)?;
        self.pending.insert(id, PendingRead { file, slot_tag });
        Ok(id)
    }

    /// Attempts to cancel one of this owner's pending requests.
    ///
    /// A queued request that is cancelled is removed from pending state.
    /// Running and finished requests stay pending because the backend still
    /// delivers their completion. `Unknown` is terminal and clears any stale
    /// mapping because the backend has no completion left to deliver.
    pub fn cancel(&mut self, id: RequestId) -> CancelOutcome {
        if !self.pending.contains_key(&id) {
            return CancelOutcome::Unknown;
        }

        let outcome = self.files.cancel(id);
        if matches!(outcome, CancelOutcome::Cancelled | CancelOutcome::Unknown) {
            self.pending.remove(&id);
        }
        outcome
    }

    /// Closes a file through the backend and reconciles requests it cancelled.
    ///
    /// The backend cancels queued requests without a completion, while
    /// requests already running or finished still complete. This method
    /// removes only pending mappings whose status is now absent.
    pub fn close_file(&mut self, file: FileHandle) -> bool {
        let closed = self.files.close(file);
        self.reconcile_file(file);
        closed
    }

    /// Removes mappings for requests the backend no longer knows about.
    ///
    /// Call this after closing a file directly through [`Files`] or after a
    /// different consumer collects one of this owner's completions. Queued,
    /// running, and finished requests remain tracked while their status exists.
    /// Returns how many mappings were removed.
    pub fn reconcile_pending(&mut self) -> usize {
        let files = self.files;
        let before = self.pending.len();
        self.pending.retain(|id, _| files.status(*id).is_some());
        before - self.pending.len()
    }

    /// Drains ready platform completions and attaches this owner's slot tag.
    ///
    /// A request submitted through this owner is returned with `Some(slot)`
    /// exactly once. Completions for other requests are returned with no slot
    /// tag and are not discarded. Requests cancelled through a direct backend
    /// close are also reconciled after this poll.
    pub fn poll_completions(&mut self) -> Vec<StreamReadCompletion> {
        let mut completions = Vec::new();
        self.files.poll_completions(&mut completions);

        let tagged = completions
            .into_iter()
            .map(|completion| StreamReadCompletion {
                slot_tag: self
                    .pending
                    .remove(&completion.id)
                    .map(|pending| pending.slot_tag),
                completion,
            })
            .collect();
        self.reconcile_pending();
        tagged
    }

    /// Number of requests still tracked by this owner.
    ///
    /// After a direct backend close, call [`ReadOwner::reconcile_pending`] or
    /// [`ReadOwner::poll_completions`] before relying on this count.
    #[must_use]
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }

    fn reconcile_file(&mut self, file: FileHandle) -> usize {
        let files = self.files;
        let before = self.pending.len();
        self.pending
            .retain(|id, pending| pending.file != file || files.status(*id).is_some());
        before - self.pending.len()
    }
}
