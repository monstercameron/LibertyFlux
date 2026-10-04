// original: 0x00b8d5d0 SET_FILTER_SAVE_SETTING
/// Script native `SET_FILTER_SAVE_SETTING` (hash 0x47F971E8).
///
/// Forwards two script arguments (a filter index and a setting value) to
/// the engine. No return slot is written.
export!(cdecl, rw_00b8d5d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
