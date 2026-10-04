// original: 0x00b87110 POINT_FIXED_CAM_AT_POS
/// Script native `POINT_FIXED_CAM_AT_POS` (hash 0x6D4E2A4A).
///
/// Forwards four script arguments to the engine: three float bit-patterns
/// (a target position) followed by one integer. Floats are copied as raw
/// bits, so the forward is bit-exact. No return slot is written.
export!(cdecl, rw_00b87110(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
