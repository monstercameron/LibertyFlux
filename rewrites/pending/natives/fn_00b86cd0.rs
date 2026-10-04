// original: 0x00b86cd0 GET_SCREEN_VIEWPORT_ID
// Rewrite of the GET_SCREEN_VIEWPORT_ID native handler.

/// Script native `GET_SCREEN_VIEWPORT_ID(...)`.
///
/// Forwards the single script argument to the engine viewport routine. No
/// return slot is written; the engine's answer is left in the return register.
export!(cdecl, rw_b86cd0(ctx: *const u8) -> u32 {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        callee_cdecl!(1, u32, *args)
    }
});
