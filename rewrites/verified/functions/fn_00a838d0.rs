// original: 0x00a838d0 CComplexJumpTaskInfo::vf7

/// Log three jump-task state bytes at +0x18, +0x19 and +0x1a. A nonzero byte selects its affirmative label; zero selects the alternate. Calls are made after the base task formatter in field order.
lf_checker_rt::export!(thiscall, rw_00a838d0(this: u32, logger_context: u32) -> () {
    #[inline(always)]
    fn field_u8(base: u32, offset: u32) -> u8 {
        unsafe { (base.wrapping_add(offset) as *const u8).read() }
    }
    #[inline(always)]
    fn field_u16(base: u32, offset: u32) -> u16 {
        unsafe { (base.wrapping_add(offset) as *const u16).read_unaligned() }
    }
    #[inline(always)]
    fn field_u32(base: u32, offset: u32) -> u32 {
        unsafe { (base.wrapping_add(offset) as *const u32).read_unaligned() }
    }
    #[inline(always)]
    fn field_f32(base: u32, offset: u32) -> f32 {
        f32::from_bits(field_u32(base, offset))
    }
    #[inline(always)]
    fn f64_low_word(value: f32) -> u32 {
        (value as f64).to_bits() as u32
    }
    #[inline(always)]
    fn f64_high_word(value: f32) -> u32 {
        ((value as f64).to_bits() >> 32) as u32
    }
    #[inline(always)]
    fn emit(context: u32, format: u32, value: u32) {
        let _ = lf_checker_rt::callee_cdecl!(
            1, u32, context, 1u32, 1u32, lf_checker_rt::relocated(format), value
        );
    }
        const FORMAT_ONE: u32 = 0x00EA0CDC;
        const LABEL_ONE: u32 = 0x00EA0CCC;
        const LABEL_ONE_CLEAR: u32 = 0x00EA0CD4;
        const FORMAT_TWO: u32 = 0x00EA0D00;
        const LABEL_TWO: u32 = 0x00EA0CF0;
        const LABEL_TWO_CLEAR: u32 = 0x00EA0CF8;
        const FORMAT_THREE: u32 = 0x00EA0D24;
        const LABEL_THREE: u32 = 0x00EA0D14;
        const LABEL_THREE_CLEAR: u32 = 0x00EA0D1C;
        let _ = lf_checker_rt::callee_thiscall!(2, u32, this, logger_context);
        emit(logger_context, FORMAT_ONE, lf_checker_rt::relocated(if field_u8(this, 0x18) != 0 { LABEL_ONE } else { LABEL_ONE_CLEAR }));
        emit(logger_context, FORMAT_TWO, lf_checker_rt::relocated(if field_u8(this, 0x19) != 0 { LABEL_TWO } else { LABEL_TWO_CLEAR }));
        emit(logger_context, FORMAT_THREE, lf_checker_rt::relocated(if field_u8(this, 0x1a) != 0 { LABEL_THREE } else { LABEL_THREE_CLEAR }));
});
