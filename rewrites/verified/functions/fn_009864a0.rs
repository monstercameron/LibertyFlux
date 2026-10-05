// original: 0x009864A0 audEmitter_forward_const1 (proposed)

/// Forwarding call through the shared emitter object: always returns 1.
///
/// Passes `index` to the callee at slot id 1 with `this` fixed to the shared
/// object at `SHARED_EMITTER`, ignores the answer and returns 1 in `al`.
/// Original: cdecl, one stack word (caller cleans; the inner call pops only
/// its own pushed copy).
lf_checker_rt::export!(cdecl, rw_009864A0(index: u32) -> u32 {
    const SHARED_EMITTER: u32 = 0x12389e0;
    const FWD_CALLEE: u32 = 1;
    unsafe {
        let _: u32 = lf_checker_rt::callee_thiscall!(
            FWD_CALLEE,
            u32,
            lf_checker_rt::relocated(SHARED_EMITTER),
            index
        );
    }
    1
});
