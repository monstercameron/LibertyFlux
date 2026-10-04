// original: 0x00b9eeb0 GET_CHAR_LAST_DAMAGE_BONE
/// `GET_CHAR_LAST_DAMAGE_BONE` (native hash `0x767E5013`): forward 2 script arguments to the
/// `NativeImpl_GET_CHAR_LAST_DAMAGE_BONE_2` engine function and store the low byte of its result
/// (zero-extended) into the context return slot.
export!(cdecl, rw_00b9eeb0(ctx: *const crate::NativeCtx) -> () {
    unsafe {
        let args = (*ctx).args;
        let answer: u32 = callee_cdecl!(1, u32, *args, *args.add(1));
        *(*ctx).ret_slot = answer & 0xFF;
    }
});
