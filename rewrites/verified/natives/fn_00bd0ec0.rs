// original: 0x00bd0ec0 GET_IS_STICKY_BOMB_STUCK_TO_OBJECT
/// `GET_IS_STICKY_BOMB_STUCK_TO_OBJECT` (native hash `0x04D623FF`): forward 1 script argument to the
/// `NativeImpl_GET_IS_STICKY_BOMB_STUCK_TO_OBJECT` engine function and store the low byte of its result
/// (zero-extended) into the context return slot.
export!(cdecl, rw_00bd0ec0(ctx: *const crate::NativeCtx08) -> () {
    unsafe {
        let args = (*ctx).args;
        let answer: u32 = callee_cdecl!(1, u32, *args);
        *(*ctx).ret_slot = answer & 0xFF;
    }
});
