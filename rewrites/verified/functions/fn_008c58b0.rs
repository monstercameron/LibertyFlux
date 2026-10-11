// original: 0x008C58B0 two_byte_element_buffer_create
/// Allocate an array with 2-byte input units, storing the quotient in the object.
/// The allocator receives quotient times the per-element allocation size; overflow saturates to `u32::MAX`.
/// The object receives the allocated pointer, and the function returns the allocator's non-null test in AL.
lf_checker_rt::export!(thiscall, rw_008c58b0(this: u32, requested: u32) -> u32 {
    unsafe {
        let count = requested >> 1;
        allocate(this, count, 2)
    }
});
