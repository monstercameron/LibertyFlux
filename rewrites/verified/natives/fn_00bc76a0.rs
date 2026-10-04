// original: 0x00bc76a0 SET_CAR_LIVERY
/// Script native `SET_CAR_LIVERY` (hash 0x2E9E149D).
///
/// Forwards two script arguments (a vehicle handle and a livery index) to
/// the engine. No return slot is written.
export!(cdecl, rw_00bc76a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
