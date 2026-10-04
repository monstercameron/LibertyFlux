// original: 0x00ba0c50 REGISTER_HATED_TARGETS_IN_AREA
/// Script native `REGISTER_HATED_TARGETS_IN_AREA` (hash 0x619E7657).
///
/// Forwards five script arguments to the engine: an integer followed by four
/// float bit-patterns (area coordinates). Floats are copied as raw bits, so
/// the forward is bit-exact. No return slot is written.
export!(cdecl, rw_00ba0c50(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
        )
    }
});
