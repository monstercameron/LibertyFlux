// original: 0x00bc7c10 SET_RECORDING_TO_POINT_NEAREST_TO_COORS
/// Script native `SET_RECORDING_TO_POINT_NEAREST_TO_COORS` (hash 0x7B732460).
///
/// Forwards four script arguments to the engine: an integer followed
/// by three float bit-patterns (coordinates). Floats are copied as
/// raw bits, so the forward is bit-exact. No return slot is written.
export!(cdecl, rw_00bc7c10(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
        )
    }
});
