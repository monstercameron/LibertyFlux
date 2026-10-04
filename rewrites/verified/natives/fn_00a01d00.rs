// original: 0x00a01d00 SET_OBJECT_HEADING
/// Script native `SET_OBJECT_HEADING` (hash 0x4F5D027C).
///
/// Forwards an object handle and a float heading as raw bits to the engine.
/// No return slot is written.
export!(cdecl, rw_00a01d00(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
