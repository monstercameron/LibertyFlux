// original: 0x00bd7670 ADD_NETWORK_RESTART
/// Script native handler `ADD_NETWORK_RESTART`.
///
/// Adds a respawn point used when the player dies in a network game.
///
/// Forwards 5 script arguments to the engine function; no return slot.
/// Float arguments pass through by value as raw bits (bit-exact copies).
/// handler function: `0x00bd7670`, engine call site: `0x00bd76be`.
export!(cdecl, rw_bd7670(ctx: *const NativeCtx) -> u32 {
    unsafe {
        let args = (*ctx).args_ptr;
        let x = *args.add(0);
        let y = *args.add(1);
        let z = *args.add(2);
        let heading = *args.add(3);
        let flags = *args.add(4);
        callee_cdecl!(1, u32, x, y, z, heading, flags)
    }
});
