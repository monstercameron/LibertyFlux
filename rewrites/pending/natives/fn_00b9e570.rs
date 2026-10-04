// original: 0x00b9e570 CLEAR_PED_NON_REMOVAL_AREA
/// Script native `CLEAR_PED_NON_REMOVAL_AREA` (hash 0x0A74017B).
///
/// Tail-thunk handler: forwards the call context to the shared
/// implementation and returns its answer.
export!(cdecl, rw_00b9e570(ctx: *const u8) -> u32 {
    unsafe { callee_cdecl!(1, u32, ctx as u32) }
});
