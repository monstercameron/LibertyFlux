// original: 0x00DF3040 UIMontageText::decrease_text_scale

/// Decrease the scale at `this + 0x210`. Values above one subtract the mapped
/// one-unit step; otherwise values above one-half become one-half, values above
/// one-quarter become one-quarter, and the remaining values become thirty.
/// Each comparison is ordered: a NaN falls through all three strict greater
/// branches to thirty. The subtraction preserves the original `current - one`
/// operand order, and every branch forwards the updated view to the duration
/// label formatter by a tail call.
lf_checker_rt::export!(thiscall, rw_00df3040(this: u32) -> u32 {
    unsafe {
        const SCALE_VALUE: u32 = 0x210;
        const ONE: u32 = 0x00fe_88e8;
        const HALF: u32 = 0x00fe_8830;
        const QUARTER: u32 = 0x00fe_87e4;
        const THIRTY_BITS: u32 = 0x41f0_0000;
        const FORMATTER: u32 = 1;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_f32(address: u32) -> f32 {
            f32::from_bits(unsafe { read_u32(address) })
        }

        #[inline(always)]
        unsafe fn write_f32(address: u32, value: f32) {
            unsafe { (address as *mut u32).write_unaligned(value.to_bits()) }
        }

        #[inline(always)]
        fn ordered_subtract(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) - core::hint::black_box(right)
        }

        #[inline(always)]
        unsafe fn refresh(this: u32) -> u32 {
            lf_checker_rt::callee_thiscall!(FORMATTER, u32, this)
        }

        let value_address = this.wrapping_add(SCALE_VALUE);
        let current = read_f32(value_address);
        let one = read_f32(lf_checker_rt::relocated(ONE));
        if current > one {
            write_f32(value_address, ordered_subtract(current, one));
            return refresh(this);
        }

        let half = read_f32(lf_checker_rt::relocated(HALF));
        if current > half {
            write_f32(value_address, half);
            return refresh(this);
        }

        let quarter = read_f32(lf_checker_rt::relocated(QUARTER));
        if current > quarter {
            write_f32(value_address, quarter);
            return refresh(this);
        }

        write_f32(value_address, f32::from_bits(THIRTY_BITS));
        refresh(this)
    }
});
