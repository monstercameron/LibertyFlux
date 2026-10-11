// original: 0x00863A20 physics_set_surface_response

/// Sets response fields in three child records reached through the object's
/// data block. For each child, flag bit 2 at `+0xe8` protects that record
/// from modification. The first response is half of the child's float at
/// `+0x198`; the other records receive fixed values. The final child pointer
/// is left in EAX, and the caller's one stack word is popped by the original
/// convention.
lf_checker_rt::export!(thiscall, rw_00863a20(this: u32, _unused: u32) -> u32 {
    unsafe {
        const DATA_BLOCK: u32 = 0x0224;
        const FIRST_CHILD: u32 = 0x0040;
        const SECOND_CHILD: u32 = 0x0048;
        const THIRD_CHILD: u32 = 0x0050;
        const CHILD_FLAGS: u32 = 0x00e8;
        const PROTECTED_BIT: u8 = 0x04;
        const FIRST_SOURCE: u32 = 0x0198;
        const FIRST_DEST: u32 = 0x0168;
        const SECOND_UNIT: u32 = 0x0164;
        const SECOND_RESPONSE: u32 = 0x0168;
        const SECOND_LIMIT: u32 = 0x016c;
        const THIRD_UNIT: u32 = 0x0120;
        const HALF_BITS: u32 = 0x3f00_0000;
        const SEVEN_TENTHS_BITS: u32 = 0x3f33_3333;
        const UNIT_BITS: u32 = 0x3f80_0000;
        const HALF_UNIT_BITS: u32 = 0x3f00_0000;

        #[inline(always)]
        unsafe fn read_word(base: u32, offset: u32) -> u32 {
            unsafe { ((base.wrapping_add(offset)) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_byte(base: u32, offset: u32) -> u8 {
            unsafe { ((base.wrapping_add(offset)) as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn write_word(base: u32, offset: u32, value: u32) {
            unsafe { ((base.wrapping_add(offset)) as *mut u32).write_unaligned(value) }
        }

        let data = read_word(this, DATA_BLOCK);
        let first = read_word(data, FIRST_CHILD);
        if read_byte(first, CHILD_FLAGS) & PROTECTED_BIT == 0 {
            let input = f32::from_bits(read_word(first, FIRST_SOURCE));
            let response = core::hint::black_box(input) * core::hint::black_box(f32::from_bits(HALF_BITS));
            write_word(first, FIRST_DEST, response.to_bits());
        }

        let second = read_word(data, SECOND_CHILD);
        if read_byte(second, CHILD_FLAGS) & PROTECTED_BIT == 0 {
            write_word(second, SECOND_UNIT, UNIT_BITS);
            write_word(second, SECOND_RESPONSE, SEVEN_TENTHS_BITS);
            write_word(second, SECOND_LIMIT, HALF_UNIT_BITS);
        }

        let third = read_word(data, THIRD_CHILD);
        if read_byte(third, CHILD_FLAGS) & PROTECTED_BIT == 0 {
            write_word(third, THIRD_UNIT, UNIT_BITS);
        }
        third
    }
});
