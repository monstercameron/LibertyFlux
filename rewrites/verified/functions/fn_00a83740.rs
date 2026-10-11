// original: 0x00a83740 CComplexGangDrivebyTaskInfo::vf7

/// Log the gang-driveby task's vehicle, three converted coordinates and two state flags. The 16-bit vehicle id at +0x18 is unsigned for lookup and signed when printed as a fallback; zero selects the coordinate report. Each stored f32 coordinate is converted to f64 in field order and passed as a double. Bytes at +0x31 and +0x32 select their labels.
lf_checker_rt::export!(thiscall, rw_00a83740(this: u32, logger_context: u32) -> () {
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
        const FORMAT_VEHICLE_NAME: u32 = 0x00EA0318;
        const FORMAT_VEHICLE_ID: u32 = 0x00EA032C;
        const FORMAT_COORDINATES: u32 = 0x00EA0344;
        const VEHICLE_TABLE: u32 = 0x01908EF0;
        const FORMAT_FLAG_ONE: u32 = 0x00EA036C;
        const LABEL_FLAG_ONE: u32 = 0x00EA035C;
        const LABEL_FLAG_ONE_CLEAR: u32 = 0x00EA0364;
        const FORMAT_FLAG_TWO: u32 = 0x00EA0388;
        const LABEL_FLAG_TWO: u32 = 0x00EA0378;
        const LABEL_FLAG_TWO_CLEAR: u32 = 0x00EA0380;
        let _ = lf_checker_rt::callee_thiscall!(2, u32, this, logger_context);
        let vehicle_id = field_u16(this, 0x18);
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
        } else {
            let x = field_f32(this, 0x20);
            let y = field_f32(this, 0x24);
            let z = field_f32(this, 0x28);
            let _ = lf_checker_rt::callee_cdecl!(
                5, u32, logger_context, 1u32, 1u32,
                lf_checker_rt::relocated(FORMAT_COORDINATES),
                f64_low_word(x), f64_high_word(x),
                f64_low_word(y), f64_high_word(y),
                f64_low_word(z), f64_high_word(z)
            );
        }
        emit(logger_context, FORMAT_FLAG_ONE, lf_checker_rt::relocated(if field_u8(this, 0x31) != 0 { LABEL_FLAG_ONE } else { LABEL_FLAG_ONE_CLEAR }));
        emit(logger_context, FORMAT_FLAG_TWO, lf_checker_rt::relocated(if field_u8(this, 0x32) != 0 { LABEL_FLAG_TWO } else { LABEL_FLAG_TWO_CLEAR }));
});
