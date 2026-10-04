// original: 0x00bd21d0 EXTINGUISH_OBJECT_FIRE
/// Script native `EXTINGUISH_OBJECT_FIRE` (hash 0x5FBC5FFF).
///
/// Forwards one script argument (an object handle) to the engine. No return
/// slot is written.
export!(cdecl, rw_00bd21d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
