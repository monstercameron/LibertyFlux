// original: 0x0060B0F0 physics_has_two_clear_bits

/// Scan the first `parameter[0x1f2]` bits of the bitmap reached through
/// `this + 0x10c`. Return one when at least two scanned bits are clear, or
/// when the optional state object at `this + 0x180` has a signed mode below
/// three. The result is defined in AL; upper EAX bits are not compared. This
/// is a 32-bit thiscall with a parameter pointer on the stack.
lf_checker_rt::export!(thiscall, rw_0060B0F0(this: u32, parameter: u32) -> u32 {
    unsafe {
        const BITMAP_TABLE: u32 = 0x10c;
        const STATE_OBJECT: u32 = 0x180;
        const PARAMETER_COUNT: u32 = 0x1f2;
        const STATE_MODE: u32 = 0x2c0;
        let count = ((parameter + PARAMETER_COUNT) as *const u8).read() as u32;
        let bitmap_words = ((this + BITMAP_TABLE) as *const u32).read_unaligned();
        let bitmap = (bitmap_words as *const u32).read_unaligned();
        let mut clear_bits = 0u32;
        for bit_index in 0..count {
            if bitmap & (1u32 << bit_index) == 0 {
                clear_bits += 1;
                if clear_bits == 2 {
                    break;
                }
            }
        }
        let found_two_clear_bits = clear_bits == 2;
        let state = ((this + STATE_OBJECT) as *const u32).read_unaligned();
        let state_forces_true = state != 0
            && (((state + STATE_MODE) as *const i32).read_unaligned() < 3);
        u32::from(found_two_clear_bits || state_forces_true)
    }
});
