// original: 0x00ba2150 SET_HEADING_LIMIT_FOR_ATTACHED_PED
/// Script native `SET_HEADING_LIMIT_FOR_ATTACHED_PED` (hash 0x15B07D4D).
///
/// Ped handle + two float limit bits.
///
/// Float script arguments are forwarded as raw bit patterns,
/// so the forward is bit-exact.
/// No return slot is written.
///
/// The handler returns the engine's answer in EAX (the register
/// is untouched after the call); the rewrite does the same.
export!(cdecl, rw_00ba2150(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
