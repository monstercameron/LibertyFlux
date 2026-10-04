// original: 0x00b93ef0 ATTACH_PARACHUTE_MODEL_TO_PLAYER
/// Script native `ATTACH_PARACHUTE_MODEL_TO_PLAYER` (hash 0x7EDD58E1).
///
/// Forwards two script arguments (a player index and a parachute model handle) to the engine.
/// No return slot is written.
export!(cdecl, rw_00b93ef0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
