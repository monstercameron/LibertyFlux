// original: 0x009FD240 frag_drain_pending (proposed)

/// Move every pending node of `obj` into its list at `+0x18`.
///
/// `obj + 8` points at the next pending node and `obj + 0x0c` marks the end.
/// While the two differ, the current node is passed to the move callee
/// (which advances the pending pointer) and the pointer is re-read. Only
/// zero or one iteration per call is observable under the checker (the
/// callee's answer is scripted per trial, so longer drains cannot be
/// driven). Returns the final pending pointer.
///
/// Original: 0x009FD240 (thiscall, `obj` in `ecx`, no stack words).
lf_checker_rt::export!(thiscall, rw_009FD240(obj: u32) -> u32 {
    unsafe {
        const NEXT_OFF: u32 = 8;
        const END_OFF: u32 = 0x0C;
        let end = obj + END_OFF;
        let mut cur = ((obj + NEXT_OFF) as *const u32).read_unaligned();
        if cur != end {
            loop {
                lf_checker_rt::callee_thiscall!(1, u32, obj, cur);
                cur = ((obj + NEXT_OFF) as *const u32).read_unaligned();
                if cur == end {
                    break;
                }
            }
        }
        cur
    }
});
