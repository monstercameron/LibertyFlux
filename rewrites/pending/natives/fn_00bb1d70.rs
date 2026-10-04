// original: 0x00bb1d70 FORCE_AIR_DRAG_MULT_FOR_PLAYERS_CAR
/// Script native `FORCE_AIR_DRAG_MULT_FOR_PLAYERS_CAR` (hash 0x554053ED).
///
/// Forwards a handle and a float multiplier to the engine; the float travels as raw bits, so the forward is bit-exact.
export!(cdecl, rw_00bb1d70(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
