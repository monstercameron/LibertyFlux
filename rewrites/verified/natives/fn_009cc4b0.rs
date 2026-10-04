// original: 0x009cc4b0 SET_BRIANS_MOOD
/// Native handler `SET_BRIANS_MOOD` (script context in, engine call out).
///
/// Sets Brian's mood value; forwards the integer to the audio setter.
/// `ctx.ret` points at the return slot, `ctx.args` at the argument array.
export!(cdecl, rw_009cc4b0(ctx: *mut NativeCtx) -> u32 {
    unsafe {
        let a = (*ctx).args;
        callee_cdecl!(1, u32, *a.add(0))
    }
});
