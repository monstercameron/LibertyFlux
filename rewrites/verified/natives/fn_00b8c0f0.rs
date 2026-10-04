// original: 0x00b8c0f0 DISPLAY_NTH_ONSCREEN_COUNTER_WITH_STRING
/// Script native `DISPLAY_NTH_ONSCREEN_COUNTER_WITH_STRING`.
///
/// Forwards four script arguments (counter slot, literal flag and two text
/// labels) to the engine. No return slot is written.
export!(cdecl, rw_00b8c0f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
