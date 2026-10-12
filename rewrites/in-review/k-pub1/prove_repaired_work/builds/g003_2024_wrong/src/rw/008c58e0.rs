use crate::{read_u32, write_u32};

/// Deliberately wrong version: use a different element divisor to test count and size visibility.
lf_checker_rt::export!(thiscall, rw_008c58e0(this: u32, requested: u32) -> u32 {
    unsafe {
        let count = requested / 16;
        allocate(this, count, 8)
    }
});

/// Allocate storage for eight byte element buffer create.
/// `this` receives the element count at +4 and allocator result at +0.
/// The helper returns AL as the allocation success flag; multiplication overflow requests `u32::MAX` bytes.
lf_checker_rt::export!(thiscall, lane_base_008c58e0(this: u32, requested: u32) -> u32 {
    unsafe {
        let count = requested >> 3;
        allocate(this, count, 8)
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
