// original: 0x00b93cf0 ACTIVATE_CHEAT
/// Script native `ACTIVATE_CHEAT` (hash 0x69E742FC).
///
/// Forwards one script argument (a cheat identifier) to the engine.
/// No return slot is written.
export!(cdecl, rw_00b93cf0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
