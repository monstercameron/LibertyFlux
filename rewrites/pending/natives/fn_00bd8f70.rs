// original: 0x00bd8f70 REGISTER_CLIENT_BROADCAST_VARIABLES
/// Script native `REGISTER_CLIENT_BROADCAST_VARIABLES` (hash 0x499B6DB6).
///
/// Forwards three script arguments to the engine. No return slot is written.
export!(cdecl, rw_00bd8f70(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
