// original: 0x00bc7150 POP_CAR_BOOT
/// Script native `POP_CAR_BOOT` (hash 0x3C78449F).
///
/// Forwards a vehicle handle to the engine, which opens the car's boot
/// and keeps it open. No return slot is written.
export!(cdecl, rw_00bc7150(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
