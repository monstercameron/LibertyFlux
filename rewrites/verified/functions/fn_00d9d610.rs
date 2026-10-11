// original: 0x00D9D610 allocate_zeroed_pair_array

/// Request an eight-byte record array from the allocator and clear both
/// 32-bit words of every element when the signed element count is positive.
/// The allocator result is returned unchanged. Nonpositive counts still call
/// the allocator with the wrapped byte count, but skip initialization.
///
/// The function is a 32-bit stdcall routine with one signed stack argument.
/// Each record is eight bytes; the original checks the count as signed and
/// leaves a null allocation untouched.
lf_checker_rt::export!(stdcall, rw_00D9D610(count: i32) -> u32 {
    const RECORD_SIZE: u32 = 8;
    const ALLOCATOR_CALL: u32 = 2;
    let byte_count = (count as u32).wrapping_mul(RECORD_SIZE);
    let allocation = lf_checker_rt::callee_cdecl!(ALLOCATOR_CALL, u32, byte_count);
    if count > 0 && allocation != 0 {
        unsafe {
            for index in 0..count as u32 {
                let record = allocation.wrapping_add(index.wrapping_mul(RECORD_SIZE));
                (record as *mut u32).write_unaligned(0);
                (record.wrapping_add(4) as *mut u32).write_unaligned(0);
            }
        }
    }
    allocation
});
