// original: 0x00b9f2a0 GET_GROUP_MEMBER
/// Script native `GET_GROUP_MEMBER` (hash 0x2FF90FF5).
///
/// Forwards three script arguments (a group handle, a member index and an
/// out value) to the engine. No return slot is written by the handler.
export!(cdecl, rw_00b9f2a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
