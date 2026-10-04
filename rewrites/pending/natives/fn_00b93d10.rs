// original: 0x00b93d10 ACTIVATE_REPLAY_MENU
/// Script native `ACTIVATE_REPLAY_MENU` (hash 0x61040B08).
///
/// Activates the video editor
///
/// The handler body is a single jump to a shared implementation
/// (a tail thunk): it forwards the call context unchanged. The
/// rewrite forwards the context pointer through the checker's
/// intercepted tail call and returns its answer.
export!(cdecl, rw_00b93d10(ctx: *const u8) -> u32 {
    unsafe {
        // Tail thunk: forward the context to the shared target.
        // The leading zero aligns the rewrite-side call log with the
        // tail stub's extra return-address word (contract skips arg 0).
        callee_cdecl!(1, u32, 0, ctx as u32)
    }
});
