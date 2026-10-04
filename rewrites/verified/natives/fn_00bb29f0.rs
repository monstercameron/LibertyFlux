// original: 0x00bb29f0 SET_PLAYER_CAN_DO_DRIVE_BY
/// Script native `SET_PLAYER_CAN_DO_DRIVE_BY` (hash 0x561471FB).
///
/// Forwards a player index and a stack-slot-coerced boolean flag to the
/// engine. No return slot is written.
export!(cdecl, rw_00bb29f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, quirked)
    }
});
