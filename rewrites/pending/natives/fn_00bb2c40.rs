// original: 0x00bb2c40 SET_PLAYER_MAY_ONLY_ENTER_THIS_VEHICLE
/// Script native `SET_PLAYER_MAY_ONLY_ENTER_THIS_VEHICLE` (hash 0x6BC05942).
///
/// Forwards two script arguments (a player number and a vehicle handle) to the engine.
///
/// No return slot is written.
export!(cdecl, rw_00bb2c40(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
