// original: 0x008C5920 twenty_four_byte_record_buffer_create
/// Allocate an array with 24-byte input units, storing the quotient in the object.
/// The allocator receives quotient times the per-element allocation size; overflow saturates to `u32::MAX`.
/// The object receives the allocated pointer, and the function returns the allocator's non-null test in AL.
lf_checker_rt::export!(thiscall, rw_008c5920(this: u32, requested: u32) -> u32 {
    unsafe {
        let count = (((requested as u64) * 0xAAAA_AAABu64) >> 35) as u32;
        allocate(this, count, 16)
    }
});
