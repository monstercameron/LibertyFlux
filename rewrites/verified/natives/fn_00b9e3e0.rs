// original: 0x00b9e3e0 BLOCK_PED_WEAPON_SWITCHING
/// Script native `BLOCK_PED_WEAPON_SWITCHING` (hash 0x315238D5).
///
/// Forwards a ped handle and a weapon-switching flag to the engine. The flag is coerced with `arg != 0`.
///
/// The flag rides in the low byte of a word whose upper bytes repeat
/// the context pointer (the original coerces it in its own stack slot);
/// the rewrite rebuilds that exact word from `ctx`.
export!(cdecl, rw_00b9e3e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, quirked)
    }
});
