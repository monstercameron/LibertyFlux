// original: 0x00b9a2f0 ADD_SPAWN_BLOCKING_DISC
/// Script native `ADD_SPAWN_BLOCKING_DISC` (hash 0x2B4E2A8C).
///
/// Forwards five script words (position and size floats) to the engine as
/// raw bit patterns, so the forward is bit-exact. No return slot is written.
export!(cdecl, rw_00b9a2f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4)
        )
    }
});
