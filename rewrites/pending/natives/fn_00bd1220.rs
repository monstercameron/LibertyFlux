// original: 0x00bd1220 SET_DEATH_WEAPONS_PERSIST
/// Script native `SET_DEATH_WEAPONS_PERSIST` (hash 0x49F86791).
///
/// Forwards a weapon value and a stack-slot-coerced boolean flag to the
/// engine. No return slot is written.
export!(cdecl, rw_00bd1220(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, quirked)
    }
});
