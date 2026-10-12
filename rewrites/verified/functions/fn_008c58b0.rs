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

/// Store the element count and request its checked byte size from the allocator stand-in.
/// The original returns only AL: non-null allocation is true, null is false.
unsafe fn allocate(this: u32, count: u32, unit: u32) -> u32 {
    unsafe {
        write_u32(this + 4, count);
        let product = (count as u64) * (unit as u64);
        let byte_size = if product > u32::MAX as u64 { u32::MAX } else { product as u32 };
        let allocation = lf_checker_rt::callee_cdecl!(ALLOCATOR_CALLEE, u32, byte_size);
        write_u32(this, allocation);
        u32::from(allocation != 0)
    }
}

const ALLOCATOR_CALLEE: u32 = 1;

#[inline(always)]
unsafe fn write_u32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}
