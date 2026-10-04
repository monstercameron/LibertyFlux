// original: 0x00b8c110 DISPLAY_ONSCREEN_TIMER_WITH_STRING
/// Script native `DISPLAY_ONSCREEN_TIMER_WITH_STRING` (hash 0x384F104F).
///
/// Forwards a timer handle, a boolean flag and a string reference. The flag
/// uses the stack-slot bool quirk (see `BREAK_CAR_DOOR`): the pushed word
/// is `(ctx & ~0xFF) | (arg != 0)`.
export!(cdecl, rw_00b8c110(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, quirked, *args.add(2))
    }
});
