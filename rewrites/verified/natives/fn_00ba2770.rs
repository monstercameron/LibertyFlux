// original: 0x00ba2770 SET_ROOM_FOR_DUMMY_CHAR_BY_NAME
/// Script native `SET_ROOM_FOR_DUMMY_CHAR_BY_NAME` (hash 0x75B024C6).
///
/// Forwards two script arguments (character handle, room name) to the engine. Writes no return slot.
export!(cdecl, rw_00ba2770(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args.add(0), *args.add(1))
    }
});
