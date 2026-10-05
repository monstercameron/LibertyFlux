// original: 0x008e0600 lookup_forward_or_tail (proposed)

/// Look `key` up and forward the answer; a miss tail-jumps elsewhere.
///
/// Calls the lookup (callee 1) with `key` and returns its answer, except
/// that an answer of -1 falls into a conditional tail jump to another
/// routine instead of returning. That jump is a conditional branch rather
/// than a patchable tail call, so the proof scripts the answer to never be
/// -1 and covers the call-and-forward path only. Cdecl, one stack argument.
lf_checker_rt::export!(cdecl, rw_008e0600(key: u32) -> u32 {
    unsafe {
        const CALLEE_LOOKUP: u32 = 1;
        lf_checker_rt::callee_cdecl!(CALLEE_LOOKUP, u32, key)
    }
});
