// original: 0x008619C0 physics_clear_elapsed_time

/// Clears the elapsed-time word at `this + 0x1ac` when flag bit `0x40` at
/// `this + 0xb5` is clear, or when that bit is set and the elapsed value is at
/// least one. The original uses COMISS and branches away for unordered values,
/// so a NaN remains unchanged when the flag bit is set. No helper is called.
lf_checker_rt::export!(thiscall, rw_008619c0(this: u32) -> u32 {
    unsafe {
        const FLAGS: u32 = 0x00b5;
        const FROZEN_BIT: u8 = 0x40;
        const ELAPSED_TIME: u32 = 0x01ac;
        const ONE_BITS: u32 = 0x3f80_0000;

        #[inline(always)]
        unsafe fn read_byte(base: u32, offset: u32) -> u8 {
            unsafe { ((base.wrapping_add(offset)) as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn read_bits(base: u32, offset: u32) -> u32 {
            unsafe { ((base.wrapping_add(offset)) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_bits(base: u32, offset: u32, bits: u32) {
            unsafe { ((base.wrapping_add(offset)) as *mut u32).write_unaligned(bits) }
        }

        if read_byte(this, FLAGS) & FROZEN_BIT == 0 {
            write_bits(this, ELAPSED_TIME, 0);
        } else {
            let elapsed = f32::from_bits(read_bits(this, ELAPSED_TIME));
            if elapsed >= f32::from_bits(ONE_BITS) {
                write_bits(this, ELAPSED_TIME, 0);
            }
        }
        0
    }
});
