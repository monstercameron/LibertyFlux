// original: 0x00bc5230 BREAK_CAR_DOOR
/// Script native `BREAK_CAR_DOOR` (hash 0x18BD071B).
///
/// Forwards a vehicle handle, a door index and a boolean flag. The flag is
/// coerced with `arg != 0` into the low byte of the handler's own incoming
/// stack slot, so the pushed word's high bytes repeat the context pointer;
/// the engine reads only the low byte (Inferred) and the full dword is
/// reproduced here for bit-exact outgoing-call matching.
export!(cdecl, rw_00bc5230(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(2) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, *args.add(1), quirked)
    }
});
