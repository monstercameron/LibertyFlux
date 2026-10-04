// original: 0x00a01900 REMOVE_PROJTEX_FROM_OBJECT
/// Script native `REMOVE_PROJTEX_FROM_OBJECT` (hash 0x7330132C).
///
/// Forwards one script argument (an object handle) to the engine.
/// No return slot is written.
export!(cdecl, rw_00a01900(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
