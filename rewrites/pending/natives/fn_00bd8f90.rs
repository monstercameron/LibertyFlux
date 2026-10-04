// original: 0x00bd8f90 REGISTER_HOST_BROADCAST_VARIABLES
/// Script native `REGISTER_HOST_BROADCAST_VARIABLES` (hash 0x18DB4CAF).
///
/// Forwards three script arguments (an array pointer, a count and flags) to
/// the engine. No return slot is written.
export!(cdecl, rw_00bd8f90(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
