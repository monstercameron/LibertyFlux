// original: 0x00b8bae0 ADD_BLIP_FOR_PICKUP
/// Script native `ADD_BLIP_FOR_PICKUP` (hash 0x04F567FB).
///
/// Forwards a pickup handle and an out-pointer argument to the engine,
/// which adds a blip and marker to the pickup. No return slot is written.
export!(cdecl, rw_00b8bae0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
