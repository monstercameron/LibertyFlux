// original: 0x00b86950 CREATE_VIEWPORT
/// Script native `CREATE_VIEWPORT` (hash 0x13134CCD).
///
/// Forwards one script argument (a viewport index) to the engine.
/// No return slot is written.
export!(cdecl, rw_00b86950(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
