// original: 0x00A3F520 vehicle_contains_attachment_pointer

/// Test whether `target` is exactly equal to the receiver's primary
/// attachment pointer at `+0x0f50` or to one of the eight pointers beginning
/// at `+0x0f54`. Null is compared as an ordinary pointer value. Return the
/// result in `AL`.
///
/// The original is a thiscall method with one stack pointer argument.
lf_checker_rt::export!(thiscall, rw_00a3f520(this: u32, target: u32) -> u8 {
    const PRIMARY_SLOT: u32 = 0x0f50;
    const ARRAY_FIRST_SLOT: u32 = 0x0f54;
    const ARRAY_LENGTH: u32 = 8;
    const SLOT_STRIDE: u32 = 4;

    unsafe {
        if (this.wrapping_add(PRIMARY_SLOT) as *const u32).read_unaligned() == target {
            return 1;
        }
        for index in 0..ARRAY_LENGTH {
            let slot = this.wrapping_add(ARRAY_FIRST_SLOT + index * SLOT_STRIDE);
            if (slot as *const u32).read_unaligned() == target {
                return 1;
            }
        }
        0
    }
});
