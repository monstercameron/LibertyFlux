// original: 0x00bb2d50 SET_SCRIPT_LIMIT_TO_GANG_SIZE
// Rewrite of the SET_SCRIPT_LIMIT_TO_GANG_SIZE native handler.

/// Script native `SET_SCRIPT_LIMIT_TO_GANG_SIZE(limit)`.
///
/// Forwards the single script argument to the engine gang-size routine. No
/// return slot is written; the engine's answer is left in the return register.
export!(cdecl, rw_bb2d50(ctx: *const u8) -> u32 {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        callee_cdecl!(1, u32, *args)
    }
});
