// original: 0x00bb6580 REGISTER_FLOAT_STAT
/// Script native `REGISTER_FLOAT_STAT` (hash 0x347E05F3).
///
/// Stat id + float value bits.
///
/// Float script arguments are forwarded as raw bit patterns,
/// so the forward is bit-exact.
/// No return slot is written.
///
/// The handler returns the engine's answer in EAX (the register
/// is untouched after the call); the rewrite does the same.
export!(cdecl, rw_00bb6580(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
