// original: 0x00b9a240 ADD_GROUP_TO_NETWORK_RESTART_NODE_GROUP_LIST
/// Script native `ADD_GROUP_TO_NETWORK_RESTART_NODE_GROUP_LIST`.
///
/// Forwards one script argument (a group handle) to the engine. No return
/// slot is written.
export!(cdecl, rw_00b9a240(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
