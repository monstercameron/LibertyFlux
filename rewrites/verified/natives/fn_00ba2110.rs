// original: 0x00ba2110 SET_GROUP_MEMBER
/// Script native `SET_GROUP_MEMBER` (hash 0x5E0F611E).
///
/// Forwards two script arguments (a group handle and a member index) to the
/// engine. No return slot is written.
export!(cdecl, rw_00ba2110(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
