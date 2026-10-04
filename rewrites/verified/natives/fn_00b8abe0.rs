// original: 0x00b8abe0 SET_CUTSCENE_EXTRA_ROOM_POS
/// Script native `SET_CUTSCENE_EXTRA_ROOM_POS` (hash 0x226A7227).
///
/// Forwards three float script arguments (a position) as raw bits to the
/// engine cutscene routine. No return slot is written.
export!(cdecl, rw_00b8abe0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
