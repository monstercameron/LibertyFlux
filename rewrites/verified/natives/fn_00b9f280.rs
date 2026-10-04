// original: 0x00b9f280 GET_GROUP_LEADER
/// Script native `GET_GROUP_LEADER` (hash 0x5DBB46B5).
///
/// Forwards two script arguments (a group handle and an out-reference) to the engine. No return slot is written by the handler itself.
export!(cdecl, rw_00b9f280(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
