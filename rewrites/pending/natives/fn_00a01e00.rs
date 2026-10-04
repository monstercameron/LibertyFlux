// original: 0x00a01e00 SET_OBJECT_ONLY_DAMAGED_BY_PLAYER
/// Script native `SET_OBJECT_ONLY_DAMAGED_BY_PLAYER` (hash 0x2E321155).
///
/// Forwards an object handle and a toggle flag, coerced with the stack-slot bool quirk. No return slot is written.
export!(cdecl, rw_00a01e00(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        // Stack-slot bool quirk: the original coerces into its own
        // incoming stack slot, so the pushed word's high bytes repeat
        // the context pointer. Reproduced exactly for bit-exact calls.
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args.add(0), quirked)
    }
});
