// original: 0x006673b0 alloc_keyframe_index

/// Allocate `count` twelve-byte keyframe-index records with 16-byte alignment
/// through allocator slot `+8` on the object in TLS slot zero. The allocator
/// is called even for zero or negative signed counts, with the byte count
/// computed by wrapping 32-bit `count * 12`. A positive count initializes
/// each record to `[0, 0, u32::MAX]`, advancing by 12 bytes. The allocation
/// pointer is returned unchanged. This is a stdcall function with one signed
/// 32-bit count argument; the outgoing allocator call is an intercepted
/// thiscall with three stack arguments.
lf_checker_rt::export!(stdcall, rw_006673b0(count: i32) -> u32 {
    unsafe {
        const ALLOCATOR_ALIGNMENT: u32 = 0x10;
        const ALLOCATOR_FLAGS: u32 = 0;
        const TLS_ALLOCATOR_OFFSET: u32 = 8;
        const VTABLE_ALLOCATE: u32 = 8;
        const ELEMENT_STRIDE: u32 = 12;
        const ELEMENT_SECOND_WORD: u32 = 4;
        const ELEMENT_SENTINEL_WORD: u32 = 8;
        const SENTINEL: u32 = u32::MAX;
        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let tls_value = lf_checker_rt::tls_slot(0);
        let allocator_object = read_word(tls_value + TLS_ALLOCATOR_OFFSET);
        let vtable = read_word(allocator_object);
        let allocate_entry = read_word(vtable + VTABLE_ALLOCATE);
        let allocate: unsafe extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(allocate_entry as usize);

        let allocation_bytes = (count as u32).wrapping_mul(ELEMENT_STRIDE);
        let allocation = allocate(
            allocator_object,
            allocation_bytes,
            ALLOCATOR_ALIGNMENT,
            ALLOCATOR_FLAGS,
        );

        let mut remaining = count;
        let mut cursor = allocation;
        while remaining > 0 {
            if cursor != 0 {
                write_word(cursor, 0);
                write_word(cursor + ELEMENT_SECOND_WORD, 0);
                write_word(cursor + ELEMENT_SENTINEL_WORD, SENTINEL);
            }
            cursor = cursor.wrapping_add(ELEMENT_STRIDE);
            remaining -= 1;
        }

        allocation
    }
});
