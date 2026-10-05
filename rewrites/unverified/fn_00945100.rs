// original: 0x00945100 streaming_open_wrapped (proposed)

/// Resolve the streaming handle, then tail-jump to the opener with it.
///
/// Calls the resolver (cdecl, two arguments: the requested id and zero),
/// then tail-jumps to the opener routine with the resolved handle as its
/// argument, returning the opener's result.
///
/// Original: 0x00945100 (cdecl, one stack argument; ends in a tail jump).
lf_checker_rt::export!(cdecl, rw_00945100(id: u32) -> u32 {
    const RESOLVE: u32 = 1;
    const OPEN: u32 = 2;
    let handle: u32 = lf_checker_rt::callee_cdecl!(RESOLVE, u32, id, 0u32);
    lf_checker_rt::callee_cdecl!(OPEN, u32, handle)
});
