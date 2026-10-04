// original: 0x00bc7bd0 SET_RAILTRACK_RESISTANCE_MULT
/// Script native `SET_RAILTRACK_RESISTANCE_MULT` (hash 0x3D7B10E7).
///
/// Forwards one script argument (a float multiplier) to the engine as raw
/// bits, so the forward is bit-exact. No return slot is written.
export!(cdecl, rw_00bc7bd0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
