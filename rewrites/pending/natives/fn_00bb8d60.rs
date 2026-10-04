// original: 0x00bb8d60 IS_CHAR_USING_MAP_ATTRACTOR
/// `IS_CHAR_USING_MAP_ATTRACTOR` (native hash `0x60B26D74`): forward 1 script argument to the
/// `NativeImpl_IS_CHAR_USING_MAP_ATTRACTOR` engine function and store the low byte of its result
/// (zero-extended) into the context return slot.
export!(cdecl, rw_00bb8d60(ctx: *const crate::NativeCtx) -> () {
    unsafe {
        let args = (*ctx).args;
        let answer: u32 = callee_cdecl!(1, u32, *args);
        *(*ctx).ret_slot = answer & 0xFF;
    }
});
