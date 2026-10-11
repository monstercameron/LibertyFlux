// original: 0x009B81F0 create_cam_instruction_from_vec4
/// Create a 32-byte camera instruction and copy four source words bit-for-bit to offsets 16 through 28.
// original: 0x009B81F0 create_cam_instruction_from_vec4
/// Create a 32-byte camera instruction and copy four source words bit-for-bit to offsets 16 through 28.
/// The returned allocation is compared through the pointer-aware heap contract.
lf_checker_rt::export!(stdcall, rw_009b81f0(arg0: u32) -> u32 {
    const ALLOCATION_SIZE: u32 = 0x20;
    const OBJECT_VTABLE_VA: u32 = 0x00E943B8;
    const VTABLE_OFFSET: u32 = 0;
    const FLAGS_OFFSET: u32 = 4;
    const CLEARED_FLAGS: u8 = 0;
    const ALL_ONES: u32 = u32::MAX;
    let object = lf_checker_rt::callee_cdecl!(1, u32, ALLOCATION_SIZE);
    if object == 0 { return 0; }
    const FIELD_OFFSET_10: u32 = 0x10;
    const FIELD_OFFSET_14: u32 = 0x14;
    const FIELD_OFFSET_18: u32 = 0x18;
    const FIELD_OFFSET_1C: u32 = 0x1C;
    let input0 = unsafe { (arg0.wrapping_add(0) as *const u32).read_unaligned() };
    let input1 = unsafe { (arg0.wrapping_add(4) as *const u32).read_unaligned() };
    let input2 = unsafe { (arg0.wrapping_add(8) as *const u32).read_unaligned() };
    let input3 = unsafe { (arg0.wrapping_add(12) as *const u32).read_unaligned() };
    unsafe {
    (object.wrapping_add(VTABLE_OFFSET) as *mut u32).write_unaligned(lf_checker_rt::relocated(OBJECT_VTABLE_VA));
    (object.wrapping_add(FLAGS_OFFSET) as *mut u8).write(CLEARED_FLAGS);
    (object.wrapping_add(FIELD_OFFSET_10) as *mut u32).write_unaligned(input0);
    (object.wrapping_add(FIELD_OFFSET_14) as *mut u32).write_unaligned(input1);
    (object.wrapping_add(FIELD_OFFSET_18) as *mut u32).write_unaligned(input2);
    (object.wrapping_add(FIELD_OFFSET_1C) as *mut u32).write_unaligned(input3);
    }
    object
});
