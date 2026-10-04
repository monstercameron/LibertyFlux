// original: 0x00bb25f0 IS_PLAYER_TARGETTING_CHAR
/// `IS_PLAYER_TARGETTING_CHAR` (native hash `0x58A6457C`): forward 2 script arguments to the
/// `NativeImpl_IS_PLAYER_TARGETTING_CHAR` engine function and store the low byte of its result
/// (zero-extended) into the context return slot.
export!(cdecl, rw_00bb25f0(ctx: *const crate::NativeCtx) -> () {
    unsafe {
        let args = (*ctx).args;
        let answer: u32 = callee_cdecl!(1, u32, *args, *args.add(1));
        *(*ctx).ret_slot = answer & 0xFF;
    }
});
