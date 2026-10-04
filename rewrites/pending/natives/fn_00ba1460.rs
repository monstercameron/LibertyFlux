// original: 0x00ba1460 SET_CHAR_DROWNS_IN_SINKING_VEHICLE
/// Script native `SET_CHAR_DROWNS_IN_SINKING_VEHICLE` (hash 0x1E805412).
///
/// Forwards a character handle and a stack-slot-coerced boolean flag to
/// the engine. No return slot is written.
export!(cdecl, rw_00ba1460(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, quirked)
    }
});
