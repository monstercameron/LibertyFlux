// original: 0x00a83650 CComplexClimbLadderTaskInfo::vf7

/// Log the climb-ladder task's three one-byte state flags. `this` supplies the state bytes at +0x18, +0x19 and +0x1a; each zero byte selects its alternate label. The logger context is forwarded to the base task formatter and every logger call in order.
lf_checker_rt::export!(thiscall, rw_00a83650(this: u32, logger_context: u32) -> () {
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
        const FMT_CLIMBING: u32 = 0x00EA0950;
        const LABEL_CLIMBING: u32 = 0x00EA0940;
        const LABEL_NOT_CLIMBING: u32 = 0x00EA0948;
        const FMT_HOLDING: u32 = 0x00EA09B0;
        const LABEL_HOLDING: u32 = 0x00EA0988;
        const LABEL_NOT_HOLDING: u32 = 0x00EA0990;
        const FMT_USING_HANDS: u32 = 0x00EA09E4;
        const LABEL_USING_HANDS: u32 = 0x00EA09C4;
        const LABEL_NOT_USING_HANDS: u32 = 0x00EA09CC;
        let _ = lf_checker_rt::callee_thiscall!(2, u32, this, logger_context);
        emit(logger_context, FMT_CLIMBING, lf_checker_rt::relocated(if field_u8(this, 0x18) != 0 { LABEL_CLIMBING } else { LABEL_NOT_CLIMBING }));
        emit(logger_context, FMT_HOLDING, lf_checker_rt::relocated(if field_u8(this, 0x19) != 0 { LABEL_HOLDING } else { LABEL_NOT_HOLDING }));
        emit(logger_context, FMT_USING_HANDS, lf_checker_rt::relocated(if field_u8(this, 0x1a) != 0 { LABEL_USING_HANDS } else { LABEL_NOT_USING_HANDS }));
});
