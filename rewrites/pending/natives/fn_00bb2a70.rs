// original: 0x00bb2a70 SET_PLAYER_CONTROL_ADVANCED
/// Script native `SET_PLAYER_CONTROL_ADVANCED` (hash 0x31E25160).
///
/// Forwards arg0, arg1 (bool), arg2 (bool), arg3 (bool) to the engine.
/// No return slot is written.
///
/// Boolean argument(s) arg3 reuse the incoming context-pointer stack slot as a
/// one-byte temporary, so the pushed word keeps the context address in its high
/// bytes; the engine reads only the low byte. Reproduced exactly from `ctx`.
export!(cdecl, rw_00bb2a70(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let c1 = (*args.add(1) != 0) as u32;
        let c2 = (*args.add(2) != 0) as u32;
        let b3 = ((*args.add(3) != 0) as u32) | (ctx as u32 & 0xFFFFFF00);
        let ans = callee_cdecl!(1, u32, *args.add(0), c1, c2, b3);
        ans
    }
});
