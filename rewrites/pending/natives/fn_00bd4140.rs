// original: 0x00bd4140 REMOVE_PROJTEX_IN_RANGE
/// Script native `REMOVE_PROJTEX_IN_RANGE` (hash 0x170F0D58).
///
/// Forwards four float bit-patterns (a position and a range) to the engine. Floats are copied as raw bits, so the forward is bit-exact. No return slot is written.
export!(cdecl, rw_00bd4140(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
        )
    }
});
