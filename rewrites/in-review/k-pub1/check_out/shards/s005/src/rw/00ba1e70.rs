// original: 0x00ba1e70 SET_DECISION_MAKER_ATTRIBUTE_LOW_HEALTH
/// `SET_DECISION_MAKER_ATTRIBUTE_LOW_HEALTH` (native hash `0x2FFA6C89`): forward 2 script arguments to the
/// `NativeImpl_SET_DECISION_MAKER_ATTRIBUTE_LOW_HEALTH` engine function. Nothing is written back to the call context.
export!(cdecl, rw_00ba1e70(ctx: *const NativeCtx08) -> () {
    unsafe {
        let args = (*ctx).args;
        __lf_callee_cdecl!(1, u32, *args, *args.add(1));
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
