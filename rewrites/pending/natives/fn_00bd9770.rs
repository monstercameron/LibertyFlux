// original: 0x00bd9770 TELEPORT_NETWORK_PLAYER
/// Script native `TELEPORT_NETWORK_PLAYER` (hash 0x2EE310C5).
///
/// Forwards five script arguments to the engine: an integer followed
/// by four float bit-patterns (coordinates). Floats are copied as raw
/// bits, so the forward is bit-exact. No return slot is written.
export!(cdecl, rw_00bd9770(ctx: *const u8) -> u32 {
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
