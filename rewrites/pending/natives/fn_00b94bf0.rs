// original: 0x00b94bf0 OVERRIDE_NEXT_RESTART
/// Script native `OVERRIDE_NEXT_RESTART` (hash 0x27636B69).
///
/// Forwards four float script arguments (position and heading) to the
/// engine as raw bit-patterns. No return slot is written.
export!(cdecl, rw_00b94bf0(ctx: *const u8) -> u32 {
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
