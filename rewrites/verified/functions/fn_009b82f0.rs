// original: 0x009B82F0 NativeImpl_SET_CAMERA_CONTROLS_DISABLED_WITH_PLAYER_CONTROLS
/// Create a camera-controls instruction and store only the low byte of its boolean argument at offset 8.
// original: 0x009B82F0 NativeImpl_SET_CAMERA_CONTROLS_DISABLED_WITH_PLAYER_CONTROLS
/// Create a camera-controls instruction and store only the low byte of its boolean argument at offset 8.
/// The returned allocation is compared through the pointer-aware heap contract.
lf_checker_rt::export!(stdcall, rw_009b82f0(arg0: u32) -> u32 {
    const ALLOCATION_SIZE: u32 = 0xC;
    const OBJECT_VTABLE_VA: u32 = 0x00E945D8;
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
    (object.wrapping_add(FIELD_OFFSET_08) as *mut u8).write((arg0 as u8) as u8);
    }
    object
});
