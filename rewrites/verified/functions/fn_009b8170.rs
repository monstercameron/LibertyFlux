// original: 0x009B8170 NativeImpl_SET_CAM_IN_FRONT_OF_PED
/// Create a CCamScriptInstruction_SetCamInFrontPed_Q record carrying one entity value. Allocation failure returns null.
// original: 0x009B8170 NativeImpl_SET_CAM_IN_FRONT_OF_PED
/// Create a CCamScriptInstruction_SetCamInFrontPed_Q record carrying one entity value. Allocation failure returns null.
/// The returned allocation is compared through the pointer-aware heap contract.
lf_checker_rt::export!(stdcall, rw_009b8170(arg0: u32) -> u32 {
    const ALLOCATION_SIZE: u32 = 0xC;
    const OBJECT_VTABLE_VA: u32 = 0x00E943F8;
    const VTABLE_OFFSET: u32 = 0;
    const FLAGS_OFFSET: u32 = 4;
    const CLEARED_FLAGS: u8 = 0;
    const ALL_ONES: u32 = u32::MAX;
    let object = lf_checker_rt::callee_cdecl!(1, u32, ALLOCATION_SIZE);
    if object == 0 { return 0; }
    const FIELD_OFFSET_08: u32 = 0x8;
    unsafe {
    (object.wrapping_add(VTABLE_OFFSET) as *mut u32).write_unaligned(lf_checker_rt::relocated(OBJECT_VTABLE_VA));
    (object.wrapping_add(FLAGS_OFFSET) as *mut u8).write(CLEARED_FLAGS);
    (object.wrapping_add(FIELD_OFFSET_08) as *mut u32).write_unaligned(arg0);
    }
    object
});
