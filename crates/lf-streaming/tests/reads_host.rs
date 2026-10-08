//! Host integration tests for the streamed read request lifecycle.
use std::io;
use std::path::Path;

use lf_platform::files::{
    CancelOutcome, Files, MemoryFiles, OpenFile, Priority, ReadError, ReadRequest, RequestStatus,
};
use lf_streaming::reads::ReadOwner;

fn backend() -> (MemoryFiles, OpenFile) {
    let files = MemoryFiles::new();
    files
        .insert("stream/data.bin", b"streamed bytes".to_vec())
        .unwrap();
    let open = files.open(Path::new("stream/data.bin")).unwrap();
    (files, open)
}

#[test]
fn completion_is_tagged_after_step_and_delivered_only_once() {
    let (files, open) = backend();
    let mut reads = ReadOwner::new(&files);
    let id = reads.submit(open.handle, 9, 5, Priority::High, 23).unwrap();

    assert_eq!(reads.pending_count(), 1);
    assert!(reads.poll_completions().is_empty());
    assert!(files.step());

    let completed = reads.poll_completions();
    assert_eq!(completed.len(), 1);
    assert_eq!(completed[0].slot_tag, Some(23));
    assert_eq!(completed[0].completion.id, id);
    assert_eq!(completed[0].completion.file, open.handle);
    assert_eq!(completed[0].completion.offset, 9);
    assert_eq!(completed[0].completion.requested, 5);
    assert_eq!(completed[0].completion.tag, 23);
    assert_eq!(completed[0].completion.result, Ok(b"bytes".to_vec()));
    assert_eq!(reads.pending_count(), 0);
    assert!(reads.poll_completions().is_empty());
}

#[test]
fn cancel_and_finished_outcomes_follow_the_files_contract() {
    let (files, open) = backend();
    let mut reads = ReadOwner::new(&files);

    let cancelled = reads.submit(open.handle, 0, 1, Priority::Low, 4).unwrap();
    assert_eq!(reads.cancel(cancelled), CancelOutcome::Cancelled);
    assert_eq!(reads.pending_count(), 0);
    assert!(!files.step());
    assert!(reads.poll_completions().is_empty());
    assert_eq!(reads.cancel(cancelled), CancelOutcome::Unknown);

    let finished = reads
        .submit(open.handle, 1, 1, Priority::Normal, 5)
        .unwrap();
    assert!(files.step());
    assert_eq!(reads.cancel(finished), CancelOutcome::Finished);
    assert_eq!(reads.pending_count(), 1);

    let completed = reads.poll_completions();
    assert_eq!(completed.len(), 1);
    assert_eq!(completed[0].slot_tag, Some(5));
    assert_eq!(completed[0].completion.id, finished);
    assert_eq!(completed[0].completion.result, Ok(b"t".to_vec()));
    assert_eq!(reads.pending_count(), 0);
    assert!(reads.poll_completions().is_empty());
}

#[test]
fn injected_read_error_is_tagged_once_and_clears_pending_state() {
    let (files, open) = backend();
    files
        .fail_reads("stream/data.bin", io::ErrorKind::PermissionDenied)
        .unwrap();
    let mut reads = ReadOwner::new(&files);
    let id = reads
        .submit(open.handle, 0, 4, Priority::Urgent, 31)
        .unwrap();

    assert!(reads.poll_completions().is_empty());
    assert_eq!(reads.pending_count(), 1);
    assert!(files.step());

    let completed = reads.poll_completions();
    assert_eq!(completed.len(), 1);
    assert_eq!(completed[0].slot_tag, Some(31));
    assert_eq!(completed[0].completion.id, id);
    assert!(matches!(
        &completed[0].completion.result,
        Err(ReadError::Io {
            kind: io::ErrorKind::PermissionDenied,
            ..
        })
    ));
    assert_eq!(reads.pending_count(), 0);
    assert!(reads.poll_completions().is_empty());
}

#[test]
fn caller_priority_reaches_the_backend() {
    let (files, open) = backend();
    let mut reads = ReadOwner::new(&files);
    let low = reads.submit(open.handle, 0, 1, Priority::Low, 1).unwrap();
    let urgent = reads
        .submit(open.handle, 1, 1, Priority::Urgent, 2)
        .unwrap();

    assert!(files.step());
    assert_eq!(files.executed(), vec![urgent]);
    let first = reads.poll_completions();
    assert_eq!(first.len(), 1);
    assert_eq!(first[0].completion.id, urgent);
    assert_eq!(first[0].slot_tag, Some(2));

    assert!(files.step());
    assert_eq!(files.executed(), vec![urgent, low]);
    let second = reads.poll_completions();
    assert_eq!(second.len(), 1);
    assert_eq!(second[0].completion.id, low);
    assert_eq!(second[0].slot_tag, Some(1));
}

#[test]
fn submit_error_does_not_create_pending_state() {
    let (files, _) = backend();
    let mut reads = ReadOwner::new(&files);

    let error = reads
        .submit(
            lf_platform::files::FileHandle(999),
            0,
            1,
            Priority::Normal,
            77,
        )
        .unwrap_err();
    assert_eq!(
        error,
        lf_platform::files::SubmitError::UnknownFile(lf_platform::files::FileHandle(999))
    );
    assert_eq!(reads.pending_count(), 0);
    assert!(reads.poll_completions().is_empty());
}

#[test]
fn unrelated_backend_completions_are_returned_without_a_slot_tag() {
    let (files, open) = backend();
    let mut reads = ReadOwner::new(&files);
    let id = files
        .submit(ReadRequest::new(open.handle, 2, 3).tag(88))
        .unwrap();
    assert!(files.step());

    let completed = reads.poll_completions();
    assert_eq!(completed.len(), 1);
    assert_eq!(completed[0].slot_tag, None);
    assert_eq!(completed[0].completion.id, id);
    assert_eq!(completed[0].completion.tag, 88);
    assert_eq!(completed[0].completion.result, Ok(b"rea".to_vec()));
    assert_eq!(reads.pending_count(), 0);
}

#[test]
fn closing_one_file_cancels_only_its_queued_reads() {
    let (files, open_a) = backend();
    files.insert("stream/other.bin", b"other".to_vec()).unwrap();
    let open_b = files.open(Path::new("stream/other.bin")).unwrap();
    let mut reads = ReadOwner::new(&files);
    let cancelled = reads
        .submit(open_a.handle, 0, 1, Priority::Low, 41)
        .unwrap();
    let retained = reads
        .submit(open_b.handle, 0, 1, Priority::Normal, 42)
        .unwrap();

    assert!(reads.close_file(open_a.handle));
    assert_eq!(files.status(cancelled), None);
    assert_eq!(files.status(retained), Some(RequestStatus::Queued));
    assert_eq!(reads.pending_count(), 1);
    assert!(files.step());

    let completed = reads.poll_completions();
    assert_eq!(completed.len(), 1);
    assert_eq!(completed[0].completion.id, retained);
    assert_eq!(completed[0].slot_tag, Some(42));
    assert_eq!(completed[0].completion.result, Ok(b"o".to_vec()));
    assert_eq!(reads.pending_count(), 0);
    assert!(reads.poll_completions().is_empty());
}

#[test]
fn closing_file_keeps_finished_read_until_its_completion_is_collected() {
    let (files, open) = backend();
    let mut reads = ReadOwner::new(&files);
    let id = reads.submit(open.handle, 0, 4, Priority::High, 53).unwrap();
    assert!(files.step());
    assert_eq!(files.status(id), Some(RequestStatus::Finished));

    assert!(reads.close_file(open.handle));
    assert_eq!(reads.pending_count(), 1);
    let completed = reads.poll_completions();
    assert_eq!(completed.len(), 1);
    assert_eq!(completed[0].slot_tag, Some(53));
    assert_eq!(completed[0].completion.id, id);
    assert_eq!(completed[0].completion.result, Ok(b"stre".to_vec()));
    assert_eq!(reads.pending_count(), 0);
    assert!(reads.poll_completions().is_empty());
}

#[test]
fn explicit_reconciliation_clears_a_read_cancelled_by_direct_backend_close() {
    let (files, open) = backend();
    let mut reads = ReadOwner::new(&files);
    let id = reads
        .submit(open.handle, 0, 2, Priority::Normal, 64)
        .unwrap();

    assert!(files.close(open.handle));
    assert_eq!(files.status(id), None);
    assert_eq!(reads.pending_count(), 1);
    assert_eq!(reads.reconcile_pending(), 1);
    assert_eq!(reads.pending_count(), 0);
    assert!(!files.step());
    assert!(reads.poll_completions().is_empty());
}

/// Small controllable backend for exercising lifecycle states that MemoryFiles
/// cannot expose between its synchronous `step` and completion collection.
mod running_backend {
    use std::collections::{HashMap, VecDeque};
    use std::io;
    use std::path::Path;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    use lf_platform::files::{
        CancelOutcome, Completion, FileBytes, FileHandle, Files, OpenFile, Priority, ReadRequest,
        RequestId, RequestStatus, SubmitError,
    };

    const FILE: FileHandle = FileHandle(7);

    struct Request {
        file: FileHandle,
        offset: u64,
        len: usize,
        tag: u64,
        buffer: Vec<u8>,
        priority: Priority,
        status: RequestStatus,
    }

    #[derive(Default)]
    struct State {
        open: bool,
        next_id: u64,
        requests: HashMap<RequestId, Request>,
        completions: VecDeque<Completion>,
    }

    #[derive(Default)]
    pub(super) struct ControlledFiles(Mutex<State>);

    impl ControlledFiles {
        pub(super) fn open(&self) -> OpenFile {
            let mut state = self.0.lock().unwrap();
            state.open = true;
            OpenFile {
                handle: FILE,
                size: 64,
            }
        }

        pub(super) fn start(&self, id: RequestId) {
            let mut state = self.0.lock().unwrap();
            let request = state.requests.get_mut(&id).expect("request is queued");
            assert_eq!(request.status, RequestStatus::Queued);
            request.status = RequestStatus::Running;
        }

        pub(super) fn complete(&self, id: RequestId) {
            let mut state = self.0.lock().unwrap();
            let completion = {
                let request = state.requests.get_mut(&id).expect("request is running");
                assert_eq!(request.status, RequestStatus::Running);
                request.status = RequestStatus::Finished;
                let mut data = std::mem::take(&mut request.buffer);
                data.clear();
                data.extend((0..request.len).map(|index| {
                    request.offset.wrapping_add(index as u64) as u8
                }));
                Completion {
                    id,
                    file: request.file,
                    offset: request.offset,
                    requested: request.len,
                    tag: request.tag,
                    result: Ok(data),
                }
            };
            state.completions.push_back(completion);
        }
    }

    impl Files for ControlledFiles {
        fn open(&self, _path: &Path) -> io::Result<OpenFile> {
            Ok(ControlledFiles::open(self))
        }

        fn close(&self, file: FileHandle) -> bool {
            let mut state = self.0.lock().unwrap();
            if file != FILE || !state.open {
                return false;
            }
            state.open = false;
            state.requests.retain(|_, request| {
                request.file != file || request.status != RequestStatus::Queued
            });
            true
        }

        fn block_size(&self) -> u32 {
            1
        }

        fn submit(&self, request: ReadRequest) -> Result<RequestId, SubmitError> {
            if !matches!(&request.delivery, lf_platform::files::Delivery::Poll) {
                panic!("ControlledFiles only accepts polled test requests");
            }
            let mut state = self.0.lock().unwrap();
            if !state.open || request.file != FILE {
                return Err(SubmitError::UnknownFile(request.file));
            }
            state.next_id += 1;
            let id = RequestId(state.next_id);
            state.requests.insert(
                id,
                Request {
                    file: request.file,
                    offset: request.offset,
                    len: request.len,
                    tag: request.tag,
                    buffer: request.buffer,
                    priority: request.priority,
                    status: RequestStatus::Queued,
                },
            );
            Ok(id)
        }

        fn cancel(&self, id: RequestId) -> CancelOutcome {
            let mut state = self.0.lock().unwrap();
            match state.requests.get(&id).map(|request| request.status) {
                Some(RequestStatus::Queued) => {
                    state.requests.remove(&id);
                    CancelOutcome::Cancelled
                }
                Some(RequestStatus::Running) => CancelOutcome::Running,
                Some(RequestStatus::Finished) => CancelOutcome::Finished,
                None => CancelOutcome::Unknown,
            }
        }

        fn set_priority(&self, id: RequestId, priority: Priority) -> bool {
            let mut state = self.0.lock().unwrap();
            if let Some(request) = state.requests.get_mut(&id) {
                if request.status == RequestStatus::Queued {
                    request.priority = priority;
                    return true;
                }
            }
            false
        }

        fn status(&self, id: RequestId) -> Option<RequestStatus> {
            self.0
                .lock()
                .unwrap()
                .requests
                .get(&id)
                .map(|request| request.status)
        }

        fn poll_completions(&self, out: &mut Vec<Completion>) -> usize {
            let mut state = self.0.lock().unwrap();
            let mut count = 0;
            while let Some(completion) = state.completions.pop_front() {
                state.requests.remove(&completion.id);
                out.push(completion);
                count += 1;
            }
            count
        }

        fn take_completion(&self, id: RequestId) -> Option<Completion> {
            let mut state = self.0.lock().unwrap();
            let index = state.completions.iter().position(|item| item.id == id)?;
            let completion = state.completions.remove(index)?;
            state.requests.remove(&id);
            Some(completion)
        }

        fn wait_any(&self, _timeout: Duration) -> bool {
            !self.0.lock().unwrap().completions.is_empty()
        }

        fn wait(&self, id: RequestId, _timeout: Duration) -> Option<RequestStatus> {
            self.status(id)
        }

        fn read_whole(&self, _path: &Path) -> io::Result<FileBytes> {
            Ok(FileBytes::new(Arc::from(&b"controlled"[..])))
        }
    }
}

#[test]
fn running_cancel_keeps_owner_mapping_until_completion() {
    use running_backend::ControlledFiles;

    let files = ControlledFiles::default();
    let open = files.open();
    let mut reads = ReadOwner::new(&files);
    let id = reads.submit(open.handle, 10, 3, Priority::High, 71).unwrap();
    files.start(id);

    assert_eq!(reads.cancel(id), CancelOutcome::Running);
    assert_eq!(files.status(id), Some(RequestStatus::Running));
    assert_eq!(reads.pending_count(), 1);
    assert!(reads.poll_completions().is_empty());
    assert_eq!(reads.pending_count(), 1);

    files.complete(id);
    let completed = reads.poll_completions();
    assert_eq!(completed.len(), 1);
    assert_eq!(completed[0].slot_tag, Some(71));
    assert_eq!(completed[0].completion.id, id);
    assert_eq!(completed[0].completion.result, Ok(vec![10, 11, 12]));
    assert_eq!(reads.pending_count(), 0);
    assert!(reads.poll_completions().is_empty());
}

#[test]
fn closing_file_retains_running_read_until_tagged_completion() {
    use running_backend::ControlledFiles;

    let files = ControlledFiles::default();
    let open = files.open();
    let mut reads = ReadOwner::new(&files);
    let id = reads.submit(open.handle, 20, 2, Priority::Urgent, 82).unwrap();
    files.start(id);

    assert!(reads.close_file(open.handle));
    assert_eq!(files.status(id), Some(RequestStatus::Running));
    assert_eq!(reads.pending_count(), 1);
    assert!(reads.poll_completions().is_empty());
    assert_eq!(reads.pending_count(), 1);

    files.complete(id);
    let completed = reads.poll_completions();
    assert_eq!(completed.len(), 1);
    assert_eq!(completed[0].slot_tag, Some(82));
    assert_eq!(completed[0].completion.id, id);
    assert_eq!(completed[0].completion.result, Ok(vec![20, 21]));
    assert_eq!(reads.pending_count(), 0);
    assert!(reads.poll_completions().is_empty());
}

#[test]
fn running_untracked_completion_is_returned_intact_without_owner_tag() {
    use running_backend::ControlledFiles;

    let files = ControlledFiles::default();
    let open = files.open();
    let mut reads = ReadOwner::new(&files);
    let id = files
        .submit(ReadRequest::new(open.handle, 30, 4).tag(99))
        .unwrap();
    files.start(id);
    files.complete(id);

    let completed = reads.poll_completions();
    assert_eq!(completed.len(), 1);
    assert_eq!(completed[0].slot_tag, None);
    assert_eq!(completed[0].completion.id, id);
    assert_eq!(completed[0].completion.file, open.handle);
    assert_eq!(completed[0].completion.offset, 30);
    assert_eq!(completed[0].completion.requested, 4);
    assert_eq!(completed[0].completion.tag, 99);
    assert_eq!(completed[0].completion.result, Ok(vec![30, 31, 32, 33]));
    assert_eq!(files.status(id), None);
    assert!(reads.poll_completions().is_empty());
}
