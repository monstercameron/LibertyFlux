// original: 0x00bd8c30 NETWORK_SET_HEALTH_RETICULE_OPTION
/// Script native `NETWORK_SET_HEALTH_RETICULE_OPTION` (hash 0x3998154E).
///
/// Forwards one boolean script argument (coerced with `arg != 0`) to the engine.
///
/// The flag rides in the low byte of a word whose upper bytes repeat
/// the context pointer (the original coerces it in its own stack slot);
/// the rewrite rebuilds that exact word from `ctx`.
export!(cdecl, rw_00bd8c30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, quirked)
    }
});
