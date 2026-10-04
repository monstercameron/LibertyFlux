// original: 0x00b8c640 FREEZE_ONSCREEN_TIMER
/// Script native `FREEZE_ONSCREEN_TIMER` (hash 0x4B8B6F24).
///
/// Forwards one boolean script argument to the on-screen timer freeze switch. Quirk (observed): the handler coerces the flag into the low byte of its own incoming stack slot and pushes the whole dword, so the pushed word's high bytes repeat the context pointer. The engine reads only the low byte (Inferred); the full dword is reproduced here for bit-exact outgoing-call matching..
export!(cdecl, rw_00b8c640(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, quirked)
    }
});
