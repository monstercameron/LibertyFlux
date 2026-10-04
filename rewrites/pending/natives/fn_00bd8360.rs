// original: 0x00bd8360 NETWORK_GET_GAME_MODE
/// Native handler `NETWORK_GET_GAME_MODE`.
///
/// Returns the running multiplayer game mode, or -1 when offline.
///
/// Handler mechanics: takes the native call context,
/// Takes no arguments; stores the engine answer in the return slot.
lf_rn21_rt::export!(cdecl, rw_00bd8360(ctx: u32) -> () {
    let ret = unsafe { *(ctx as *const u32) } as *mut u32;
    let mode = lf_rn21_rt::callee_cdecl!(1, u32,);
    unsafe { *ret = mode };
});
