// original: 0x00ba1bb0 SET_CHAR_VISIBLE
/// Script native `SET_CHAR_VISIBLE` (hash 0x04CF0105).
///
/// Forwards a character handle and a boolean visibility flag to the engine.
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot, so the pushed dword is the context pointer with
/// its low byte replaced by the flag; the rewrite reproduces that dword
/// exactly from `ctx`. No return slot is written.
export!(cdecl, rw_00ba1bb0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, quirked)
    }
});
