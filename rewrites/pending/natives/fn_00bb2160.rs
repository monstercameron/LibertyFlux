// original: 0x00bb2160 GET_VEHICLE_PLAYER_WOULD_ENTER
/// Script native `GET_VEHICLE_PLAYER_WOULD_ENTER` (hash 0x20430265).
///
/// Forwards two script arguments to the engine. No return slot is written.
export!(cdecl, rw_00bb2160(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
