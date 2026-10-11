// original: 0x009B8450 NativeImpl_SET_CAM_FOV
/// Create a CCamScriptInstruction_SetFOV record, preserving the second argument's floating-point bits at offset 12.
// original: 0x009B8450 NativeImpl_SET_CAM_FOV
/// Create a CCamScriptInstruction_SetFOV record, preserving the second argument's floating-point bits at offset 12.
/// The returned allocation is compared through the pointer-aware heap contract.
lf_checker_rt::export!(stdcall, rw_009b8450(arg0: u32, arg1: u32) -> u32 {
    const ALLOCATION_SIZE: u32 = 0x10;
    const OBJECT_VTABLE_VA: u32 = 0x00E94038;
    const VTABLE_OFFSET: u32 = 0;
    const FLAGS_OFFSET: u32 = 4;
    const CLEARED_FLAGS: u8 = 0;
    const ALL_ONES: u32 = u32::MAX;
    let object = lf_checker_rt::callee_cdecl!(1, u32, ALLOCATION_SIZE);
    if object == 0 { return 0; }
    const FIELD_OFFSET_08: u32 = 0x8;
    const FIELD_OFFSET_0C: u32 = 0xC;
    unsafe {
    (object.wrapping_add(VTABLE_OFFSET) as *mut u32).write_unaligned(lf_checker_rt::relocated(OBJECT_VTABLE_VA));
    (object.wrapping_add(FLAGS_OFFSET) as *mut u8).write(CLEARED_FLAGS);
    (object.wrapping_add(FIELD_OFFSET_08) as *mut u32).write_unaligned(arg0);
    (object.wrapping_add(FIELD_OFFSET_0C) as *mut u32).write_unaligned(arg1);
    }
    object
});
