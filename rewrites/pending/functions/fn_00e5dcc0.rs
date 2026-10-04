// original: 0x00e5dcc0 dispatch_thunk_dcc0
/// Tail-dispatch to the shared block initialiser (jump thunk).
///
/// Forwards to the shared zero-argument initialiser and returns its
/// result. Takes no inputs.
lf_checker_rt::export!(cdecl, rw_00e5dcc0() -> u32 {
    lf_checker_rt::callee_cdecl!(2, u32,)
});
