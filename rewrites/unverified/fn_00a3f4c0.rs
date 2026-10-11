// original: 0x00A3F4C0 vehicle_has_active_attachment

/// Return one when the receiver's primary attachment or any of its eight
/// attachment slots contains a non-null object whose byte at `+0x218` is
/// clear and whose byte at `+0x219` is set. The primary object is checked
/// before the array; otherwise the result is zero. Return the predicate in
/// `AL` as the original does.
///
/// The original is a thiscall method with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00a3f4c0(this: u32) -> u8 {
    const PRIMARY_SLOT: u32 = 0x0f50;
    const ARRAY_FIRST_SLOT: u32 = 0x0f54;
    const ARRAY_LENGTH: u32 = 8;
    const SLOT_STRIDE: u32 = 4;
    const ACTIVE_BYTE: u32 = 0x218;
    const ENABLED_BYTE: u32 = 0x219;

    #[inline(always)]
    unsafe fn is_active(pointer: u32) -> bool {
        if pointer == 0 {
            return false;
        }
        unsafe {
            (pointer.wrapping_add(ACTIVE_BYTE) as *const u8).read() == 0
                && (pointer.wrapping_add(ENABLED_BYTE) as *const u8).read() != 0
        }
    }

    unsafe {
        let primary = (this.wrapping_add(PRIMARY_SLOT) as *const u32).read_unaligned();
        if is_active(primary) {
            return 1;
        }
        for index in 0..ARRAY_LENGTH {
            let slot = this.wrapping_add(ARRAY_FIRST_SLOT + index * SLOT_STRIDE);
            let candidate = (slot as *const u32).read_unaligned();
            if is_active(candidate) {
                return 1;
            }
        }
        0
    }
});
