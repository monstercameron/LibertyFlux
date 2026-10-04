// original: 0x009cbfc0 PANIC_SCREAM
/// Script native `PANIC_SCREAM` (hash 0x4F8B4507).
///
/// Forwards one script argument (a character handle) to the engine.
/// No return slot is written.
export!(cdecl, rw_009cbfc0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
