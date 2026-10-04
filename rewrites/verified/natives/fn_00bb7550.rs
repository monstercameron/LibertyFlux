// original: 0x00bb7550 REQUEST_COLLISION_AT_POSN
/// Script native `REQUEST_COLLISION_AT_POSN` (hash 0x12ED0BC9).
///
/// Forwards three script arguments (position coordinates) to the engine as
/// raw float bit-patterns, so the forward is bit-exact. No return slot is
/// written.
export!(cdecl, rw_00bb7550(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
