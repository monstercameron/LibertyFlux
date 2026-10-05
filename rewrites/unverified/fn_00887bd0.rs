// original: 0x00887BD0 stream_ref_inc (proposed)

/// Add one reference to the stream object under its lock.
///
/// Locks (callee 1) with the lock word at `this + 0x38`, increments the
/// reference count at `this + 8`, unlocks (callee 2) with the same word.
/// The answer is the unlock entry's answer.
///
/// Original: 0x00887BD0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00887BD0(this: u32) -> u32 {
    unsafe {
        const LOCK_WORD: u32 = 0x38;
        const REFCOUNT: u32 = 0x08;
        const LOCK: u32 = 1;
        const UNLOCK: u32 = 2;
        let w = ((this + LOCK_WORD) as *const u32).read_unaligned();
        lf_checker_rt::callee_cdecl!(LOCK, u32, w);
        let rc = ((this + REFCOUNT) as *const u32).read_unaligned();
        ((this + REFCOUNT) as *mut u32).write_unaligned(rc.wrapping_add(1));
        lf_checker_rt::callee_cdecl!(UNLOCK, u32, w)
    }
});
