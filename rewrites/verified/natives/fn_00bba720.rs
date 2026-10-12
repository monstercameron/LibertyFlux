// original: 0x00bba720 TASK_SHOOT_AT_CHAR
/// `TASK_SHOOT_AT_CHAR` (native hash `0x08022967`): forward 4 script arguments to the
/// `NativeImpl_TASK_SHOOT_AT_CHAR` engine function. Nothing is written back to the call context.
export!(cdecl, rw_00bba720(ctx: *const NativeCtx08) -> () {
    unsafe {
        let args = (*ctx).args;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3));
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
