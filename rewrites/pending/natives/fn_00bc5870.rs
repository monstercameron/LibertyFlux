// original: 0x00bc5870 FORCE_CAR_LIGHTS
/// Script native `FORCE_CAR_LIGHTS` (hash 0x71B81DE7).
///
/// Forwards two script arguments (a vehicle handle and a light mode) to the
/// engine. No return slot is written.
export!(cdecl, rw_00bc5870(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
