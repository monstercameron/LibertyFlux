// original: 0x00b9f860 IS_CHAR_IN_ANGLED_AREA_2D
/// Script native `IS_CHAR_IN_ANGLED_AREA_2D` (hash 0x7D591EAD).
///
/// Forwards arg0, arg1 (float bits), arg2 (float bits), arg3 (float bits), arg4 (float bits), arg5 (float bits), arg6 (bool) to the engine.
/// Writes the zero-extended low byte of the engine answer into the return slot.
///
/// Boolean argument(s) arg6 reuse the incoming context-pointer stack slot as a
/// one-byte temporary, so the pushed word keeps the context address in its high
/// bytes; the engine reads only the low byte. Reproduced exactly from `ctx`.
export!(cdecl, rw_00b9f860(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let b6 = ((*args.add(6) != 0) as u32) | (ctx as u32 & 0xFFFFFF00);
        let ans = callee_cdecl!(1, u32, *args.add(0), *args.add(1), *args.add(2), *args.add(3), *args.add(4), *args.add(5), b6);
        let slot = (*(ctx as *const u32)) as *mut u32;
        *slot = ans & 0xFF;
        slot as u32
    }
});
