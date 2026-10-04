// original: 0x00b9e5c0 CLEAR_SCRIPTED_CONVERSION_CENTRE
/// Script native `CLEAR_SCRIPTED_CONVERSION_CENTRE` (hash 0x2E4662B3).
///
/// Tail-thunks to the shared clear routine with the call context unchanged, forwarding its result.
export!(cdecl, rw_00b9e5c0(ctx: *const u8) -> u32 {
        callee_cdecl!(1, u32, ctx as u32)
});
