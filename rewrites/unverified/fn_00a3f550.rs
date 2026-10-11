// original: 0x00A3F550 vehicle_has_enabled_attachment

/// Return one when the primary attachment or any of the receiver's eight
/// attachment slots points to a non-null object with a nonzero byte at
/// offset `+0x219`. The primary slot is checked first, followed by the array.
/// The result is an 8-bit boolean in `AL`.
///
/// The original is a thiscall method with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00a3f550(this: u32) -> u8 {
    const PRIMARY_SLOT: u32 = 0x0f50;
    const ARRAY_FIRST_SLOT: u32 = 0x0f54;
    const ARRAY_LENGTH: u32 = 8;
    const SLOT_STRIDE: u32 = 4;
    const ENABLED_BYTE: u32 = 0x219;

    #[inline(always)]
    unsafe fn is_enabled(pointer: u32) -> bool {
        pointer != 0
            && unsafe { (pointer.wrapping_add(ENABLED_BYTE) as *const u8).read() != 0 }
    }

    unsafe {
        let primary = (this.wrapping_add(PRIMARY_SLOT) as *const u32).read_unaligned();
        if is_enabled(primary) {
            return 1;
        }
        for index in 0..ARRAY_LENGTH {
            let slot = this.wrapping_add(ARRAY_FIRST_SLOT + index * SLOT_STRIDE);
            let candidate = (slot as *const u32).read_unaligned();
            if is_enabled(candidate) {
                return 1;
            }
        }
        0
    }
});
