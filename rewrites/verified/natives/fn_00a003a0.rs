// original: 0x00a003a0 ADD_OBJECT_TO_INTERIOR_ROOM_BY_NAME
/// Script native `ADD_OBJECT_TO_INTERIOR_ROOM_BY_NAME` (hash 0x076863C9).
///
/// Forwards two script arguments (an object handle and an interior-room
/// name) to the engine. No return slot is written.
export!(cdecl, rw_00a003a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
