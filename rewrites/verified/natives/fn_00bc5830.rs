// original: 0x00bc5830 FIX_CAR_TYRE
/// Script native `FIX_CAR_TYRE` (hash 0x0FDA7965).
///
/// Forwards a vehicle handle and a tyre index to the engine, which
/// repairs that tyre. No return slot is written.
export!(cdecl, rw_00bc5830(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
