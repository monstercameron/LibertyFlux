// original: 0x00a00380 ADD_OBJECT_TO_INTERIOR_ROOM_BY_KEY
/// Script native `ADD_OBJECT_TO_INTERIOR_ROOM_BY_KEY` (hash 0x67D83807).
///
/// Forwards two script arguments (an object handle and a room key) to the
/// engine. No return slot is written.
export!(cdecl, rw_00a00380(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
