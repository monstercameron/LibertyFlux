// original: 0x00667160 resize_keyframe_index

/// Updates the requested capacity in a keyframe-index descriptor. If the
/// descriptor has no capacity yet, the function stores the requested low
/// 16 bits, asks the thread-local allocator for the full 32-bit byte count
/// when it is nonzero, and stores the returned pointer and low 16-bit size.
/// With an existing capacity it only replaces the size field. The allocation
/// helper is intercepted by the checker. The original leaves the high half
/// of EAX unspecified on the existing-capacity branch, so the proof compares
/// the descriptor and stack effects and omits the return register.
lf_checker_rt::export!(cdecl, rw_00667160(descriptor: *mut u8, requested_bytes: u32) -> u32 {
    const DATA_POINTER_OFFSET: u32 = 0;
    const SIZE_OFFSET: u32 = 4;
    const CAPACITY_OFFSET: u32 = 6;
    let descriptor_address = descriptor as u32;
    let old_capacity = unsafe { *((descriptor_address + CAPACITY_OFFSET) as *const u16) };
    let requested_size = requested_bytes as u16;

    if old_capacity != 0 {
        unsafe { *((descriptor_address + SIZE_OFFSET) as *mut u16) = requested_size; }
        requested_size as u32
    } else {
        unsafe { *((descriptor_address + CAPACITY_OFFSET) as *mut u16) = requested_size; }
        let data_pointer = if requested_bytes == 0 {
            0
        } else {
            lf_checker_rt::callee_stdcall!(1, u32, requested_bytes)
        };
        unsafe { *((descriptor_address + SIZE_OFFSET) as *mut u16) = requested_size; }
        unsafe { *((descriptor_address + DATA_POINTER_OFFSET) as *mut u32) = data_pointer; }
        data_pointer
    }
});
