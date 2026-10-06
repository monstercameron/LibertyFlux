// original: 0x0094E2B0 index_lookup_forward (proposed)

/// Map a key through callee 1, forwarding hits to callee 2 by tail call.
///
/// Calls callee 1 with (`key`, `KIND` on the stack). An answer of -1
/// (exact equality) is returned unchanged; any other answer replaces the
/// incoming stack word and the function tails into callee 2 with it,
/// returning callee 2's answer. Written as a call that forwards the
/// argument and result; the original ends in a jump.
///
/// Original: 0x0094E2B0 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_0094E2B0(key: u32) -> u32 {
    unsafe {
        const KIND: u32 = 3;
        const LOOKUP: u32 = 1;
        const FORWARD: u32 = 2;
        const MISS: u32 = 0xFFFF_FFFF;
        let found = lf_checker_rt::callee_cdecl!(LOOKUP, u32, key, KIND);
        if found == MISS { found } else { lf_checker_rt::callee_cdecl!(FORWARD, u32, found) }
    }
});
