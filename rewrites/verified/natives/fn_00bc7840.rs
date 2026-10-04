// original: 0x00bc7840 SET_CAR_TRACTION
/// Script native `SET_CAR_TRACTION` (hash 0x278F2D0A).
///
/// Forwards 2 script arguments to the engine in order.
/// Float arguments are forwarded as raw bit patterns (bit-exact).
/// No return slot is written.
export!(cdecl, rw_00bc7840(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
