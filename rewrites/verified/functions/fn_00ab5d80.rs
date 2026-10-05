// original: 0x00ab5d80 stream_reinit_sub8 (proposed)

/// Run the sub-object worker twice and forward the second result.
///
/// Calls the worker with `this + 8`, then tail-calls it again with the
/// same pointer, returning the second call's result. The two sites are
/// intercepted separately so each call is observed.
///
/// Callees: 1 = first call (thiscall, no stack words),
/// 2 = tail call (thiscall, no stack words).
///
/// Original: 0x00ab5d80 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00ab5d80(this: u32) -> u32 {
    unsafe {
        const FIRST: u32 = 1;
        const TAIL: u32 = 2;
        const SUB_OFF: u32 = 8;
        let sub = this.wrapping_add(SUB_OFF);
        lf_checker_rt::callee_thiscall!(FIRST, u32, sub);
        lf_checker_rt::callee_thiscall!(TAIL, u32, sub)
    }
});
