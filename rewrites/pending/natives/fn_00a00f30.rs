// original: 0x00A00F30 GET_ROOM_KEY_FROM_OBJECT
/// F07 GET_ROOM_KEY_FROM_OBJECT: forwards 2 args, no return slot use.
export!(cdecl, rn10_get_room_key_from_object(ctx: *const u32) -> u32 {
    unsafe {
        let (_, args) = ctx_parts(ctx);
        callee_cdecl!(2, u32, *args, *args.add(1));
        0
    }
});
