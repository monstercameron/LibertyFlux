// original: 0x00DF2D20 UIMontageText::increase_text_scale

/// Increase the scale value at `this + 0x210` through the original ordered
/// thresholds: values below 0.5 become 0.5, values from 0.5 to below 1 become
/// 1, values from 1 to below 30 increase by 1, and values at least 30 become
/// 0.25. The original `JBE` branches also take unordered comparisons, so a
/// NaN follows the final 0.25 path. Each selected result is handed to the
/// duration-label formatter by a tail call. The thresholds are loaded from
/// mapped image data, and the addition keeps the original operand order.
lf_checker_rt::export!(thiscall, rw_00df2d20(this: u32) -> u32 {
    unsafe {
        const SCALE_VALUE: u32 = 0x210;
        const HALF: u32 = 0x00fe_8830;
        const ONE: u32 = 0x00fe_88e8;
        const THIRTY: u32 = 0x00fe_8b48;
        const QUARTER_BITS: u32 = 0x3e80_0000;
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
        fn ordered_add(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }

        #[inline(always)]
        unsafe fn refresh(this: u32) -> u32 {
            lf_checker_rt::callee_thiscall!(FORMATTER, u32, this)
        }

        #[inline(always)]
        unsafe fn reaches_threshold(value: f32, threshold: f32) -> bool {
            value.is_nan() || value >= threshold
        }

        let value_address = this.wrapping_add(SCALE_VALUE);
        let current_scale = read_f32(value_address);
        let half = read_f32(lf_checker_rt::relocated(HALF));
        if !reaches_threshold(current_scale, half) {
            write_f32(value_address, half);
            return refresh(this);
        }

        let one = read_f32(lf_checker_rt::relocated(ONE));
        if !reaches_threshold(current_scale, one) {
            write_f32(value_address, one);
            return refresh(this);
        }

        let thirty = read_f32(lf_checker_rt::relocated(THIRTY));
        if !reaches_threshold(current_scale, thirty) {
            write_f32(value_address, ordered_add(current_scale, one));
            return refresh(this);
        }

        write_f32(value_address, f32::from_bits(QUARTER_BITS));
        refresh(this)
    }
});
