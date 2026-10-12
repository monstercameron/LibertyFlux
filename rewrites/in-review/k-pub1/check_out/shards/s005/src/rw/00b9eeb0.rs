// original: 0x00b9eeb0 GET_CHAR_LAST_DAMAGE_BONE
/// `GET_CHAR_LAST_DAMAGE_BONE` (native hash `0x767E5013`): forward 2 script arguments to the
/// `NativeImpl_GET_CHAR_LAST_DAMAGE_BONE_2` engine function and store the low byte of its result
/// (zero-extended) into the context return slot.
export!(cdecl, rw_00b9eeb0(ctx: *const NativeCtx08) -> () {
    unsafe {
        let args = (*ctx).args;
        let answer: u32 = __lf_callee_cdecl!(1, u32, *args, *args.add(1));
        *(*ctx).ret_slot = answer & 0xFF;
    }
});

/// Native call context, r-n08 shape: return-slot pointer, reserved word,
/// argument-array pointer, argument count. (Renamed from r-n08's NativeCtx.)
#[repr(C)]
pub struct NativeCtx08 {
    pub ret_slot: *mut u32,
    pub reserved: u32,
    pub args: *const u32,
    pub argc: u32,
}
