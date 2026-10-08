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
