// original: 0x009B7D30 create_cam_instruction_with_two_words
/// Create a 16-byte camera instruction carrying two 32-bit values. The shared allocator may return null; otherwise the first word is its vtable, byte 4 is cleared, and the values go to offsets 8 and 12. The callee cleans both stack arguments.
// original: 0x009B7D30 create_cam_instruction_with_two_words
/// Create a 16-byte camera instruction carrying two 32-bit values. The shared allocator may return null; otherwise the first word is its vtable, byte 4 is cleared, and the values go to offsets 8 and 12. The callee cleans both stack arguments.
/// The returned allocation is compared through the pointer-aware heap contract.
lf_checker_rt::export!(stdcall, rw_009b7d30(arg0: u32, arg1: u32) -> u32 {
    const ALLOCATION_SIZE: u32 = 0x10;
    const OBJECT_VTABLE_VA: u32 = 0x00E94408;
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
