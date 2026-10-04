// original: 0x00bc8000 SKIP_IN_PLAYBACK_RECORDED_CAR
/// Script native `SKIP_IN_PLAYBACK_RECORDED_CAR` (hash 0x2C8C61BA).
///
/// Forwards two script arguments (a vehicle handle and a skip offset, forwarded as raw bits) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bc8000(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
