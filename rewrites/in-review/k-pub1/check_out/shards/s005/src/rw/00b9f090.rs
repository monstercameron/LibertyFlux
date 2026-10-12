// original: 0x00b9f090 GET_CHAR_VELOCITY
/// `GET_CHAR_VELOCITY` (native hash `0x3B977FD4`): forward 4 script arguments to the
/// `NativeImpl_GET_CHAR_VELOCITY` engine function. Nothing is written back to the call context.
export!(cdecl, rw_00b9f090(ctx: *const NativeCtx08) -> () {
    unsafe {
        let args = (*ctx).args;
        __lf_callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3));
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
