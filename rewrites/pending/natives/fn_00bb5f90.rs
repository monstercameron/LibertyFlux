// original: 0x00bb5f90 DESTROY_THREAD
/// Script native `DESTROY_THREAD` (hash 0x47381E59).
///
/// Forwards one script argument (a thread handle) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bb5f90(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
