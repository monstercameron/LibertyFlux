// original: 0x00bc7d70 SET_TRAIN_SPEED
/// Script native `SET_TRAIN_SPEED` (hash 0x3F4950AC).
///
/// Forwards two script arguments (a vehicle handle and a speed value) to
/// the engine. The speed word is moved through an SSE register but only
/// copied as a bit pattern, so forwarding it as `u32` bits is bit-exact.
/// No return slot is written.
export!(cdecl, rw_00bc7d70(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
