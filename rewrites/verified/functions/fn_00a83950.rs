// original: 0x00a83950 CComplexNewExitVehicleTaskInfo::vf7

/// Log whether the exit-vehicle task's low flag bit is set, then report a nonzero 16-bit vehicle id through the vehicle lookup and name conversion helpers. If lookup fails, print the identifier sign-extended from i16. A zero id emits no vehicle record.
lf_checker_rt::export!(thiscall, rw_00a83950(this: u32, logger_context: u32) -> () {
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
        const VEHICLE_TABLE: u32 = 0x01908EF0;
        const FORMAT_EXIT_FLAG: u32 = 0x00EA0698;
        const EXIT_FLAG_SET: u32 = 0x00EA0668;
        const EXIT_FLAG_CLEAR: u32 = 0x00EA0680;
        const FORMAT_VEHICLE_NAME: u32 = 0x00EA06B8;
        const FORMAT_VEHICLE_ID: u32 = 0x00EA06CC;
        let _ = lf_checker_rt::callee_thiscall!(2, u32, this, logger_context);
        let exit_flag = field_u8(this, 0x30) & 1;
        emit(logger_context, FORMAT_EXIT_FLAG, lf_checker_rt::relocated(if exit_flag != 0 { EXIT_FLAG_SET } else { EXIT_FLAG_CLEAR }));
        let vehicle_id = field_u16(this, 0x38);
        if vehicle_id != 0 {
            let descriptor = lf_checker_rt::callee_thiscall!(
                3, u32, lf_checker_rt::relocated(VEHICLE_TABLE), vehicle_id as u32, 0u32
            );
            if descriptor != 0 {
                let vehicle_name = lf_checker_rt::callee_thiscall!(4, u32, descriptor);
                emit(logger_context, FORMAT_VEHICLE_NAME, vehicle_name);
            } else {
                let signed_vehicle_id = vehicle_id as i16 as i32 as u32;
                emit(logger_context, FORMAT_VEHICLE_ID, signed_vehicle_id);
            }
        }
});
