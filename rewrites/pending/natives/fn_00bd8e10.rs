// original: 0x00bd8e10 NETWORK_SHOW_PLAYER_PROFILE_UI
/// `NETWORK_SHOW_PLAYER_PROFILE_UI` (native hash `0x6F2A5430`): forward 1 script argument to the
/// `NativeImpl_NETWORK_SHOW_PLAYER_PROFILE_UI` engine function. Nothing is written back to the call context.
export!(cdecl, rw_00bd8e10(ctx: *const crate::NativeCtx08) -> () {
    unsafe {
        let args = (*ctx).args;
        callee_cdecl!(1, u32, *args);
    }
});
