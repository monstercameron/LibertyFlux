// original: 0x00b873c0 SET_CAM_INHERIT_ROLL_PED
/// Script native `SET_CAM_INHERIT_ROLL_PED` (hash 0x09A34209).
///
/// Forwards two script arguments (a camera handle and a ped handle) to
/// the engine. No return slot is written.
export!(cdecl, rw_00b873c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
