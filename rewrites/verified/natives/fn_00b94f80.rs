// original: 0x00b94f80 SET_PHYS_CCD_HANDLES_ROTATION
/// Script native `SET_PHYS_CCD_HANDLES_ROTATION` (hash 0x0C7B7CF4).
///
/// Forwards arg0 (bool) to the engine.
/// No return slot is written.
///
/// Boolean argument(s) arg0 reuse the incoming context-pointer stack slot as a
/// one-byte temporary, so the pushed word keeps the context address in its high
/// bytes; the engine reads only the low byte. Reproduced exactly from `ctx`.
export!(cdecl, rw_00b94f80(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let b0 = ((*args.add(0) != 0) as u32) | (ctx as u32 & 0xFFFFFF00);
        let ans = callee_cdecl!(1, u32, b0);
        ans
    }
});
