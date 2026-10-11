// original: 0x009B7E50 NativeImpl_RESET_CAM_INTERP_CUSTOM_SPEED_GRAPH
/// Create the 8-byte reset-interpolation-speed-graph instruction. On allocation success it sets the vtable and clears the flag byte; on failure it returns null.
// original: 0x009B7E50 NativeImpl_RESET_CAM_INTERP_CUSTOM_SPEED_GRAPH
/// Create the 8-byte reset-interpolation-speed-graph instruction. On allocation success it sets the vtable and clears the flag byte; on failure it returns null.
/// The returned allocation is compared through the pointer-aware heap contract.
lf_checker_rt::export!(cdecl, rw_009b7e50() -> u32 {
    const ALLOCATION_SIZE: u32 = 0x8;
    const OBJECT_VTABLE_VA: u32 = 0x00E94558;
    const VTABLE_OFFSET: u32 = 0;
    const FLAGS_OFFSET: u32 = 4;
    const CLEARED_FLAGS: u8 = 0;
    const ALL_ONES: u32 = u32::MAX;
    let object = lf_checker_rt::callee_cdecl!(1, u32, ALLOCATION_SIZE);
    if object == 0 { return 0; }
    unsafe {
    (object.wrapping_add(VTABLE_OFFSET) as *mut u32).write_unaligned(lf_checker_rt::relocated(OBJECT_VTABLE_VA));
    (object.wrapping_add(FLAGS_OFFSET) as *mut u8).write(CLEARED_FLAGS);
    }
    object
});
