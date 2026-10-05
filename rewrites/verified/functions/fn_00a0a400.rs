// original: 0x00a0a400 mission_cleanup_contains (proposed)
/// Test whether a mission-cleanup entry exists for a tag and handle.
///
/// Runs the table find with a zero extra word and returns 1 when it finds a
/// record, 0 otherwise. Only the low byte of the answer is set by the
/// original. Thiscall with two stack words.
lf_checker_rt::export!(thiscall, rw_00a0a400(this: u32, tag: u32, handle: u32) -> u32 {
    unsafe {
        const FIND: u32 = 0;
        let found = lf_checker_rt::callee_thiscall!(FIND, u32, this, tag, handle, 0u32);
        (found != 0) as u32
    }
});
