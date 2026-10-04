// original: 0x00b9fe10 IS_CHAR_STUCK_UNDER_CAR
/// Native handler `IS_CHAR_STUCK_UNDER_CAR` (script context in, engine call out).
///
/// True if the char is stuck under a car; forwards the ped handle, stores the byte answer.
/// `ctx.ret` points at the return slot, `ctx.args` at the argument array.
export!(cdecl, rw_00b9fe10(ctx: *mut NativeCtx) -> u32 {
    unsafe {
        let a = (*ctx).args;
        let r = callee_cdecl!(1, u32, *a.add(0));
        // The original keeps only AL and zero-extends it into the slot.
        *(*ctx).ret = r & 0xFF;
        (*ctx).ret as u32
    }
});
