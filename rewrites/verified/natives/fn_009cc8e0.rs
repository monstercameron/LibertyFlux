// original: 0x009cc8e0 STOP_STREAM
/// Script native `STOP_STREAM` (hash 0x66915CE9).
///
/// The handler body is a single jump to a shared implementation
/// (a tail thunk): it forwards the call context unchanged. The
/// rewrite forwards the context pointer through the checker's
/// intercepted tail call and returns its answer.
export!(cdecl, rw_009cc8e0(ctx: *const u8) -> u32 {
    unsafe {
        // Tail thunk: forward the context to the shared target.
        // The leading zero aligns the rewrite-side call log with the
        // tail stub's extra return-address word (contract skips arg 0).
        callee_cdecl!(1, u32, 0, ctx as u32)
    }
});
