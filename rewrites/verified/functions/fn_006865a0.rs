// original: 0x006865A0 dof_array_append_slot (proposed)

/// Return the next free element slot in a 32-bit pointer array and increment
/// its 16-bit length. When length equals capacity, add the requested capacity,
/// allocate a replacement buffer, copy the existing pointer words, release
/// the old buffer through the thread-local memory manager, and install the
/// replacement. The array header stores its data pointer at `+0`, length at
/// `+4`, and capacity at `+6`. Capacity and length use unsigned 16-bit
/// arithmetic; the returned pointer addresses the slot at the old length.
lf_checker_rt::export!(thiscall, rw_006865A0(array_header: u32, requested_slots: u32) -> u32 {
    unsafe {
        const DATA_POINTER: u32 = 0x00;
        const LENGTH: u32 = 0x04;
        const CAPACITY: u32 = 0x06;
        const POINTER_BYTES: u32 = 4;
        const ALLOCATE_ARRAY_ID: u32 = 1;
        const RELEASE_BUFFER_ID: u32 = 2;

        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_u16(address: u32, value: u16) {
            unsafe { (address as *mut u16).write_unaligned(value) }
        }

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let mut data_pointer = unsafe { read_u32(array_header.wrapping_add(DATA_POINTER)) };
        let old_length = unsafe { read_u16(array_header.wrapping_add(LENGTH)) };
        let old_capacity = unsafe { read_u16(array_header.wrapping_add(CAPACITY)) };
        if old_length == old_capacity {
            let new_capacity = old_capacity.wrapping_add(requested_slots as u16);
            unsafe { write_u16(array_header.wrapping_add(CAPACITY), new_capacity) };
            let replacement = unsafe {
                lf_checker_rt::callee_thiscall!(
                    ALLOCATE_ARRAY_ID,
                    u32,
                    old_capacity as u32,
                    new_capacity as u32,
                )
            };
            for index in 0..old_length as u32 {
                let byte_offset = index.wrapping_mul(POINTER_BYTES);
                let element = unsafe { read_u32(data_pointer.wrapping_add(byte_offset)) };
                unsafe { write_u32(replacement.wrapping_add(byte_offset), element) };
            }
            if data_pointer != 0 {
                // The actual manager object is reached through TLS. The
                // checker stand-in represents its virtual release call.
                let _ = unsafe {
                    lf_checker_rt::callee_thiscall!(
                        RELEASE_BUFFER_ID,
                        u32,
                        array_header,
                        data_pointer,
                    )
                };
            }
            unsafe { write_u32(array_header.wrapping_add(DATA_POINTER), replacement) };
            data_pointer = replacement;
        }

        let slot = data_pointer.wrapping_add((old_length as u32).wrapping_mul(POINTER_BYTES));
        unsafe { write_u16(array_header.wrapping_add(LENGTH), old_length.wrapping_add(1)) };
        slot
    }
});
