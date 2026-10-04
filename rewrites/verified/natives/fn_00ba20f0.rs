// original: 0x00ba20f0 SET_GROUP_LEADER
/// Script native `SET_GROUP_LEADER` (hash 0x04C85E23).
///
/// Forwards two script arguments (a group handle and a ped handle) to the
/// engine. No return slot is written.
export!(cdecl, rw_00ba20f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
