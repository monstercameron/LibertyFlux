// original: 0x00b9e3a0 BLOCK_CHAR_VISEME_ANIMS
/// Script native `BLOCK_CHAR_VISEME_ANIMS` (hash 0x44881D27).
///
/// Forwards a character handle and a toggled flag. The flag is coerced with the stack-slot bool quirk (section 2 of the r-n01 report). No return slot is written.
export!(cdecl, rw_00b9e3a0(ctx: *const u8) -> u32 {
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
