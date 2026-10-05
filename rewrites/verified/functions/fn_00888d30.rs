// original: 0x00888D30 stream_ref_dec (proposed)

/// Drop one reference from the stream object under its lock.
///
/// Locks (callee 1) with the lock word at `this + 0x38`, decrements the
/// reference count at `this + 8` unless it is already zero, unlocks
/// (callee 2) with the same word. The answer is the unlock entry's answer.
///
/// Original: 0x00888D30 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00888D30(this: u32) -> u32 {
    unsafe {
        const LOCK_WORD: u32 = 0x38;
        const REFCOUNT: u32 = 0x08;
        const LOCK: u32 = 1;
        const UNLOCK: u32 = 2;
        let w = ((this + LOCK_WORD) as *const u32).read_unaligned();
        lf_checker_rt::callee_cdecl!(LOCK, u32, w);
        let rc = ((this + REFCOUNT) as *const u32).read_unaligned();
        if rc != 0 {
            ((this + REFCOUNT) as *mut u32).write_unaligned(rc.wrapping_sub(1));
        }
        lf_checker_rt::callee_cdecl!(UNLOCK, u32, w)
    }
});
