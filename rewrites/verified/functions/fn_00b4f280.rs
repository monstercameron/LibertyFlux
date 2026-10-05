// original: 0x00b4f280 check_status_not_6 (proposed)

/// Test whether the current task's kind differs from 6, tolerantly.
///
/// The current task is fetched twice (callees 1 and 2, thiscall on `this`
/// with no arguments); a null first fetch counts as settled (returns 1).
/// Otherwise the second fetch's virtual slot at `+0x04` is queried: kind 6
/// counts as settled (1), anything else as busy (0).
///
/// Original: 0x00b4f280 (thiscall, no stack words; boolean in AL).
lf_checker_rt::export!(thiscall, rw_00b4f280(this: u32) -> u32 {
    unsafe {
        const FETCH: u32 = 1;
        const FETCH_AGAIN: u32 = 2;
        const KIND_SLOT: u32 = 0x04;
        const SETTLED_KIND: u32 = 6;
        let first = lf_checker_rt::callee_thiscall!(FETCH, u32, this);
        if first == 0 {
            return 1;
        }
        let task = lf_checker_rt::callee_thiscall!(FETCH_AGAIN, u32, this);
        let vt = (task as *const u32).read_unaligned();
        let kind: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(((vt + KIND_SLOT) as *const u32).read_unaligned() as usize);
        u32::from(kind(task) == SETTLED_KIND)
    }
});
