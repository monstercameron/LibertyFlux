// original: 0x00ab5d70 stream_thunk_70a0 (proposed)

/// Tail-thunk into the neighbouring worker with the object shifted by 4.
///
/// Adds 4 to the object pointer and tail-calls the callee, forwarding its
/// result unchanged. The original is a bare jump; the rewrite expresses it
/// as a call so the checker observes both sides identically.
///
/// Callees: 1 = tail target (thiscall, no stack words).
///
/// Original: 0x00ab5d70 (thiscall, no stack words; jump thunk).
lf_checker_rt::export!(thiscall, rw_00ab5d70(this: u32) -> u32 {
    unsafe {
        const TARGET: u32 = 1;
        const SHIFT: u32 = 4;
        lf_checker_rt::callee_thiscall!(TARGET, u32, this.wrapping_add(SHIFT))
    }
});
