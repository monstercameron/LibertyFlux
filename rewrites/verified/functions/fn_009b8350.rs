// original: 0x009B8350 create_cam_instruction_with_boolean_a
/// Create a 12-byte camera instruction and store the low byte of its boolean argument.
// original: 0x009B8350 create_cam_instruction_with_boolean_a
/// Create a 12-byte camera instruction and store the low byte of its boolean argument.
/// The returned allocation is compared through the pointer-aware heap contract.
lf_checker_rt::export!(stdcall, rw_009b8350(arg0: u32) -> u32 {
    const ALLOCATION_SIZE: u32 = 0xC;
    const OBJECT_VTABLE_VA: u32 = 0x00E94588;
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
