// original: 0x00b9e5b0 CLEAR_ROOM_FOR_DUMMY_CHAR
/// Script native `CLEAR_ROOM_FOR_DUMMY_CHAR` (hash 0x2E373084).
///
/// Forwards one script argument to the engine. No return slot is written.
export!(cdecl, rw_00b9e5b0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
