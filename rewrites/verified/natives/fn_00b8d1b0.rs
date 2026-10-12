// original: 0x00b8d1b0 PRINT_WITH_3_NUMBERS
/// `PRINT_WITH_3_NUMBERS` (native hash `0x5FE61572`): forward 6 script arguments to the
/// `NativeImpl_PRINT_WITH_3_NUMBERS` engine function. Nothing is written back to the call context.
export!(cdecl, rw_00b8d1b0(ctx: *const NativeCtx08) -> () {
    unsafe {
        let args = (*ctx).args;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4), *args.add(5));
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
