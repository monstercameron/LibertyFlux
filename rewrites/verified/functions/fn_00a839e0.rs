// original: 0x00a839e0 CComplexNewGetInVehicleTaskInfo::vf7

/// Log whether the new get-in-vehicle task's byte at +0x30 is nonzero. A nonzero value selects the affirmative label; zero selects the alternate. The formatter call follows the base method.
lf_checker_rt::export!(thiscall, rw_00a839e0(this: u32, logger_context: u32) -> () {
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
        const FORMAT: u32 = 0x00EA050C;
        const ACTIVE_LABEL: u32 = 0x00EA04FC;
        const INACTIVE_LABEL: u32 = 0x00EA0504;
        let _ = lf_checker_rt::callee_thiscall!(2, u32, this, logger_context);
        emit(logger_context, FORMAT, lf_checker_rt::relocated(if field_u8(this, 0x30) != 0 { ACTIVE_LABEL } else { INACTIVE_LABEL }));
});
