// original: 0x009B7EF0 create_cam_instruction_reset_variant_b
/// Create an 8-byte camera instruction record with no payload. Allocation failure returns null.
// original: 0x009B7EF0 create_cam_instruction_reset_variant_b
/// Create an 8-byte camera instruction record with no payload. Allocation failure returns null.
/// The returned allocation is compared through the pointer-aware heap contract.
lf_checker_rt::export!(cdecl, rw_009b7ef0() -> u32 {
    const ALLOCATION_SIZE: u32 = 0x8;
    const OBJECT_VTABLE_VA: u32 = 0x00E94358;
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
