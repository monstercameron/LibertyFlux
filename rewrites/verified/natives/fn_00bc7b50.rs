// original: 0x00bc7b50 SET_PLANE_THROTTLE
/// Script native `SET_PLANE_THROTTLE` (hash 0x05B2442A).
///
/// Forwards two script arguments to the engine: an integer followed by a
/// float bit-pattern. No return slot is written.
export!(cdecl, rw_00bc7b50(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
