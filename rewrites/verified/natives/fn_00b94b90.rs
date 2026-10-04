// original: 0x00b94b90 LIMIT_ANGLE
/// Script native `LIMIT_ANGLE` (hash 0x4CAE3B65).
///
/// Forwards one float bit-pattern (the angle) and one integer (the range)
/// to the engine. No return slot is written.
export!(cdecl, rw_00b94b90(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
        )
    }
});
