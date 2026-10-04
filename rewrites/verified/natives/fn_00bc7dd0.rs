// original: 0x00bc7dd0 SET_VEHICLE_ALPHA
/// Script native `SET_VEHICLE_ALPHA` (hash 0x0C4B7DD3).
///
/// Forwards two script arguments to the engine. No return slot is written.
export!(cdecl, rw_00bc7dd0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
