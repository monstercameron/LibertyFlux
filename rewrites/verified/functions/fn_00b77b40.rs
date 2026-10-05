// original: 0x00b77b40 lookup_then_dispatch (proposed)

/// Look up the handler for `key` and dispatch to it, else return 0.
///
/// Calls the lookup callee (cdecl, one stack word: `key`). A null answer
/// means no handler (returns 0). Otherwise calls the dispatch callee
/// (thiscall on the handler, one stack word: the caller's return address,
/// which the original re-pushes from its own frame) and returns its answer.
///
/// Original: 0x00b77b40 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00b77b40(key: u32) -> u32 {
    unsafe {
        const LOOKUP_CALLEE: u32 = 1;
        const DISPATCH_CALLEE: u32 = 2;
        let handler: u32 = lf_checker_rt::callee_cdecl!(LOOKUP_CALLEE, u32, key);
        if handler == 0 {
            return 0;
        }
        let retaddr = (&key as *const u32).offset(-1).read_unaligned();
        lf_checker_rt::callee_thiscall!(DISPATCH_CALLEE, u32, handler, retaddr)
    }
});
