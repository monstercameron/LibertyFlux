// original: 0x00bc80a0 START_PLAYBACK_RECORDED_CAR
/// Script native `START_PLAYBACK_RECORDED_CAR` (hash 0x53335A45).
///
/// Forwards two script arguments (a vehicle handle and a recording id) to the engine. No return slot is written.
export!(cdecl, rw_00bc80a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args.add(0), *args.add(1))
    }
});
