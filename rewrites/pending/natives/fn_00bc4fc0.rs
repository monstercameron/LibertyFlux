// original: 0x00bc4fc0 ADD_CAR_TO_MISSION_DELETION_LIST
/// Script native `ADD_CAR_TO_MISSION_DELETION_LIST` (hash 0x45E80BF7).
///
/// Forwards one script argument (a vehicle handle) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bc4fc0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
