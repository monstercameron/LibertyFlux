// original: 0x00bc7b90 SET_PLAYBACK_SPEED
/// Native handler `SET_PLAYBACK_SPEED` (script context in, engine call out).
///
/// Sets car-recording playback speed; forwards vehicle handle and speed float.
/// `ctx.ret` points at the return slot, `ctx.args` at the argument array.
export!(cdecl, rw_00bc7b90(ctx: *mut NativeCtx) -> u32 {
    unsafe {
        let a = (*ctx).args;
        callee_cdecl!(1, u32, *a.add(0), *a.add(1))
    }
});
