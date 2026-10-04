// original: 0x00bb6690 SET_MISSION_RESPECT_TOTAL
/// Script native `SET_MISSION_RESPECT_TOTAL` (hash 0x3FA46EB8).
///
/// Forwards one script argument (a float bit-pattern) to the engine. Copied
/// as raw bits, so the forward is bit-exact. No return slot is written.
export!(cdecl, rw_00bb6690(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
