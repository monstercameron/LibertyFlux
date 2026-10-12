// original: 0x00667020 resize_keyframe_array

/// Updates a keyframe-array descriptor. When its capacity is already set,
/// the requested low 16 bits replace the size field. When capacity is zero,
/// the function records the requested capacity and, for a nonzero request,
/// asks the thread-local allocator for 32 bytes per entry with 16-byte
/// alignment. It stores the allocator result and the requested size. The
/// indirect allocator call is intercepted through the planted interface
/// table. The old-capacity branch only defines AX, so the proof omits EAX.
lf_checker_rt::export!(thiscall, rw_00667020(descriptor: *mut u8, requested_count: u32) -> u32 {
    const DATA_POINTER_OFFSET: u32 = 0;
    const SIZE_OFFSET: u32 = 4;
    const CAPACITY_OFFSET: u32 = 6;
    const ENTRY_BYTES: u32 = 32;
    const ALIGNMENT: u32 = 16;
    const ALLOCATOR_FLAGS: u32 = 0;
    let descriptor_address = descriptor as u32;
    let old_capacity = unsafe { *((descriptor_address + CAPACITY_OFFSET) as *const u16) };
    let requested_size = requested_count as u16;

    if old_capacity != 0 {
        unsafe { *((descriptor_address + SIZE_OFFSET) as *mut u16) = requested_size; }
        requested_size as u32
    } else {
        unsafe { *((descriptor_address + CAPACITY_OFFSET) as *mut u16) = requested_size; }
        let data_pointer = if requested_count == 0 {
            0
        } else {
            let tls_root = lf_checker_rt::tls_slot(0);
            let allocator_interface = unsafe { *((tls_root.wrapping_add(8)) as *const u32) };
            lf_checker_rt::callee_thiscall!(1, u32, allocator_interface,
                requested_count.wrapping_mul(ENTRY_BYTES), ALIGNMENT, ALLOCATOR_FLAGS)
        };
        unsafe { *((descriptor_address + SIZE_OFFSET) as *mut u16) = requested_size; }
        unsafe { *((descriptor_address + DATA_POINTER_OFFSET) as *mut u32) = data_pointer; }
        data_pointer
    }
});
