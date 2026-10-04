// original: 0x00abbb00 copy_record_halves

/// Split a 32-byte record into two 16-byte outputs.
///
/// Copies the second half of the source record into the second destination
/// first, then the first half into the first destination (all plain bit
/// copies; the original moves the middle words through vector registers).
/// Returns the first destination pointer.
export!(cdecl, rs64_abbb00(src: *const u32, dst0: *mut u32, dst1: *mut u32) -> u32 {
    unsafe {
        *dst1 = *((src as u32).wrapping_add(0x10) as *const u32);
        *dst1.add(1) = *((src as u32).wrapping_add(0x14) as *const u32);
        *dst1.add(2) = *((src as u32).wrapping_add(0x18) as *const u32);
        *dst1.add(3) = *((src as u32).wrapping_add(0x1c) as *const u32);
        *dst0 = *src;
        *dst0.add(1) = *src.add(1);
        *dst0.add(2) = *src.add(2);
        *dst0.add(3) = *src.add(3);
        dst0 as u32
    }
});
