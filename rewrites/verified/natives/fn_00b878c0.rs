// original: 0x00b878c0 SET_DRUNK_CAM
/// Script native `SET_DRUNK_CAM` (hash 0x74B90C48).
///
/// Forwards three script arguments to the engine: two integers and one
/// float bit-pattern (the middle argument). Floats are copied as raw bits,
/// so the forward is bit-exact. No return slot is written.
export!(cdecl, rw_00b878c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
