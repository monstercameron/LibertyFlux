// original: 0x00bc82d0 TAKE_CAR_OUT_OF_PARKED_CARS_BUDGET
/// Script native `TAKE_CAR_OUT_OF_PARKED_CARS_BUDGET` (hash 0x60EF0519).
///
/// Forwards arg0, arg1 (bool) to the engine.
/// No return slot is written.
///
/// Boolean argument(s) arg1 reuse the incoming context-pointer stack slot as a
/// one-byte temporary, so the pushed word keeps the context address in its high
/// bytes; the engine reads only the low byte. Reproduced exactly from `ctx`.
export!(cdecl, rw_00bc82d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let b1 = ((*args.add(1) != 0) as u32) | (ctx as u32 & 0xFFFFFF00);
        let ans = callee_cdecl!(1, u32, *args.add(0), b1);
        ans
    }
});
