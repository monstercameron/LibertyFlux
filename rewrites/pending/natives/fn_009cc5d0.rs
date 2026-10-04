// original: 0x009cc5d0 SET_PED_IS_DRUNK
/// Script native `SET_PED_IS_DRUNK` (hash 0x67CC007C).
///
/// Forwards two script arguments (a character handle and a boolean flag) to
/// the engine. The flag is coerced with `arg != 0`. No return slot is
/// written.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The full dword is reproduced here
/// for bit-exact outgoing-call matching.
export!(cdecl, rw_009cc5d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, quirked)
    }
});
