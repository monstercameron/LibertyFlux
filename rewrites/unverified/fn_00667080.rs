// original: 0x00667080 alloc_keyframe_array

/// Requests `count * 48` bytes from the thread-local allocator with 16-byte alignment and zero flags, using wrapping 32-bit multiplication. It returns the allocation pointer. If signed `count` is positive and allocation succeeds, each 48-byte record has words at offsets 0, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28, and 0x2c cleared; other bytes are untouched. Nonpositive signed counts and null results skip the initialization loop.
lf_checker_rt::export!(stdcall, rw_00667080(count: u32) -> u32 {
    const RECORD_BYTES: u32 = 0x30;
    let tls_root = lf_checker_rt::tls_slot(0);
    let allocator_interface = unsafe { *((tls_root.wrapping_add(8)) as *const u32) };
    let allocated = lf_checker_rt::callee_thiscall!(1, u32, allocator_interface,
        count.wrapping_mul(RECORD_BYTES), 16u32, 0u32);
    if (count as i32) > 0 && allocated != 0 {
        const RECORD_STRIDE: u32 = 0x30;
        const CLEARED_WORD_OFFSETS: [u32; 9] = [0, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c];
        for record_index in 0..count {
            let record = allocated.wrapping_add(record_index.wrapping_mul(RECORD_STRIDE));
            for field_offset in CLEARED_WORD_OFFSETS {
                let field_value = 0;
                unsafe { *((record.wrapping_add(field_offset)) as *mut u32) = field_value; }
            }
        }
    }
    allocated
});
