// original: 0x00bc55e0 DETACH_CAR
/// Script native `DETACH_CAR` (hash 0x34CC1F23).
///
/// Forwards one script argument (a vehicle handle) to the engine, which
/// detaches the vehicle. No return slot is written by the handler.
export!(cdecl, rw_00bc55e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
