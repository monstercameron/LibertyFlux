// original: 0x00b876f0 SET_CAM_SPLINE_SPEED_CONSTANT
/// Script native `SET_CAM_SPLINE_SPEED_CONSTANT` (hash 0x2CF72EB7).
///
/// Forwards arg0, arg1 (bool) to the engine.
/// No return slot is written.
///
/// Boolean argument(s) arg1 reuse the incoming context-pointer stack slot as a
/// one-byte temporary, so the pushed word keeps the context address in its high
/// bytes; the engine reads only the low byte. Reproduced exactly from `ctx`.
export!(cdecl, rw_00b876f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let b1 = ((*args.add(1) != 0) as u32) | (ctx as u32 & 0xFFFFFF00);
        let ans = callee_cdecl!(1, u32, *args.add(0), b1);
        ans
    }
});
