// original: 0x0087F7A0 rage::miniheapAllocator::allocate_aligned

/// Align the allocator cursor, reserve the requested byte count, and return
/// the aligned start. Sizes up to four use four-byte alignment, sizes up to
/// eight use eight-byte alignment, and larger sizes use the supplied
/// alignment. Cursor arithmetic wraps at 32 bits. If the new cursor exceeds
/// capacity, call the allocator's error reporter with its format pointer.
lf_checker_rt::export!(thiscall, rw_0087f7a0(_this: u32, size: u32, alignment: u32, _unused: u32) -> u32 {
    unsafe {
        const CURSOR_OFFSET: u32 = 0x30;
        const CAPACITY_OFFSET: u32 = 0x4C;
        const OVERFLOW_FORMAT_VA: u32 = 0x00FC8C44;
        const OVERFLOW_CALLEE: u32 = 1;

        let allocator = lf_checker_rt::tls_slot(0);
        let cursor_ptr = allocator.wrapping_add(CURSOR_OFFSET) as *mut u32;
        let current_cursor = cursor_ptr.read_unaligned();
        let capacity = (allocator.wrapping_add(CAPACITY_OFFSET) as *const u32).read_unaligned();
        let alignment_mask = if size <= 4 {
            3
        } else if size <= 8 {
            7
        } else {
            alignment.wrapping_sub(1)
        };
        let aligned_start = current_cursor.wrapping_add(alignment_mask) & !alignment_mask;
        let next_cursor = aligned_start.wrapping_add(size);
        cursor_ptr.write_unaligned(next_cursor);

        if next_cursor > capacity {
            let overflow_format = lf_checker_rt::relocated(OVERFLOW_FORMAT_VA);
            let _ = lf_checker_rt::callee_cdecl!(OVERFLOW_CALLEE, u32, overflow_format, 0, 0);
        }
        aligned_start
    }
});
