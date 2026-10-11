// original: 0x00a83830 CComplexGunTaskInfo::vf7

/// Log the weapon word at +0x40, three state labels at +0x44 through +0x46, and the trailing word at +0x48. Each label byte is tested against zero; the logger context is passed to the base task formatter and every record.
lf_checker_rt::export!(thiscall, rw_00a83830(this: u32, logger_context: u32) -> () {
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
        const FORMAT_WEAPON: u32 = 0x00EA03DC;
        const FORMAT_FLAG_ONE: u32 = 0x00EA0404;
        const LABEL_FLAG_ONE: u32 = 0x00EA03F4;
        const LABEL_FLAG_ONE_CLEAR: u32 = 0x00EA03FC;
        const FORMAT_FLAG_TWO: u32 = 0x00EA042C;
        const LABEL_FLAG_TWO: u32 = 0x00EA041C;
        const LABEL_FLAG_TWO_CLEAR: u32 = 0x00EA0424;
        const FORMAT_FLAG_THREE: u32 = 0x00EA0468;
        const LABEL_FLAG_THREE: u32 = 0x00EA0444;
        const LABEL_FLAG_THREE_CLEAR: u32 = 0x00EA044C;
        const FORMAT_VARIANT: u32 = 0x00EA0480;
        let _ = lf_checker_rt::callee_thiscall!(2, u32, this, logger_context);
        emit(logger_context, FORMAT_WEAPON, field_u32(this, 0x40));
        emit(logger_context, FORMAT_FLAG_ONE, lf_checker_rt::relocated(if field_u8(this, 0x44) != 0 { LABEL_FLAG_ONE } else { LABEL_FLAG_ONE_CLEAR }));
        emit(logger_context, FORMAT_FLAG_TWO, lf_checker_rt::relocated(if field_u8(this, 0x45) != 0 { LABEL_FLAG_TWO } else { LABEL_FLAG_TWO_CLEAR }));
        emit(logger_context, FORMAT_FLAG_THREE, lf_checker_rt::relocated(if field_u8(this, 0x46) != 0 { LABEL_FLAG_THREE } else { LABEL_FLAG_THREE_CLEAR }));
        emit(logger_context, FORMAT_VARIANT, field_u32(this, 0x48));
});
