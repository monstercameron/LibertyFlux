// original: 0x009cc4e0 SET_LOBBY_MUTE_OVERRIDE
/// Script native `SET_LOBBY_MUTE_OVERRIDE` (hash 0x10800FD6).
///
/// Forwards a single toggle flag, coerced with the stack-slot bool quirk. No return slot is written.
export!(cdecl, rw_009cc4e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(0) != 0);
        // Stack-slot bool quirk: the original coerces into its own
        // incoming stack slot, so the pushed word's high bytes repeat
        // the context pointer. Reproduced exactly for bit-exact calls.
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, quirked)
    }
});
