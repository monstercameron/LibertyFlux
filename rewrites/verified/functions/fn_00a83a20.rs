// original: 0x00a83a20 CComplexNewUseCoverInfo::vf7

/// Log the 32-bit cover value at +0x18 and six state bytes at +0x1c through +0x21. The byte fields are zero-extended to 32 bits and records retain their original order after the base formatter call.
lf_checker_rt::export!(thiscall, rw_00a83a20(this: u32, logger_context: u32) -> () {
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
        const FORMAT_VALUE: u32 = 0x00EA0848;
        const FORMAT_ONE: u32 = 0x00EA086C;
        const FORMAT_TWO: u32 = 0x00EA0890;
        const FORMAT_THREE: u32 = 0x00EA08B4;
        const FORMAT_FOUR: u32 = 0x00EA08D8;
        const FORMAT_FIVE: u32 = 0x00EA08FC;
        const FORMAT_SIX: u32 = 0x00EA091C;
        let _ = lf_checker_rt::callee_thiscall!(2, u32, this, logger_context);
        emit(logger_context, FORMAT_VALUE, field_u32(this, 0x18));
        emit(logger_context, FORMAT_ONE, field_u8(this, 0x1c) as u32);
        emit(logger_context, FORMAT_TWO, field_u8(this, 0x1d) as u32);
        emit(logger_context, FORMAT_THREE, field_u8(this, 0x1e) as u32);
        emit(logger_context, FORMAT_FOUR, field_u8(this, 0x1f) as u32);
        emit(logger_context, FORMAT_FIVE, field_u8(this, 0x20) as u32);
        emit(logger_context, FORMAT_SIX, field_u8(this, 0x21) as u32);
});
