// original: 0x00A844F0 CSitDownIdleThenStandTaskInfo::append_seat_state

/// Forward the base report, then emit seat type and direction, heading as an f64 widened from the f32 at +0x20, two status bytes at +0x24 and +0x25, and the f32 seat position at +0x30, +0x34 and +0x38.
lf_checker_rt::export!(thiscall, rw_00a844f0(this: u32, logger_context: u32) -> () {

    #[inline(always)]
    fn read_u8(base: u32, offset: u32) -> u8 {
        unsafe { core::ptr::read(base.wrapping_add(offset) as *const u8) }
    }
    #[inline(always)]
    fn read_u32(base: u32, offset: u32) -> u32 {
        unsafe { core::ptr::read_unaligned(base.wrapping_add(offset) as *const u32) }
    }
    #[inline(always)]
    fn double_words(value: f32) -> (u32, u32) {
        let bits = (value as f64).to_bits();
        (bits as u32, (bits >> 32) as u32)
    }

    const FIELD_SEAT_TYPE: u32 = 0x18;
    const FIELD_SEAT_DIRECTION: u32 = 0x1C;
    const FIELD_HEADING: u32 = 0x20;
    const FIELD_IS_SITTING: u32 = 0x24;
    const FIELD_IS_STANDING: u32 = 0x25;
    const FIELD_POSITION_X: u32 = 0x30;
    const FIELD_POSITION_Y: u32 = 0x34;
    const FIELD_POSITION_Z: u32 = 0x38;
    const FORMAT_SEAT_TYPE: u32 = 0x00EA0F40;
    const FORMAT_SEAT_DIRECTION: u32 = 0x00EA0F54;
    const FORMAT_HEADING: u32 = 0x00EA0F68;
    const FALSE_SITTING: u32 = 0x00EA0F88;
    const TRUE_SITTING: u32 = 0x00EA0F80;
    const FORMAT_SITTING: u32 = 0x00EA0F90;
    const FALSE_STANDING: u32 = 0x00EA0FAC;
    const TRUE_STANDING: u32 = 0x00EA0FA4;
    const FORMAT_STANDING: u32 = 0x00EA0FB4;
    const FORMAT_SEAT_POSITION: u32 = 0x00EA0FC8;
    let _ = lf_checker_rt::callee_thiscall!(1, u32, this, logger_context);
    let seat_type = read_u32(this, FIELD_SEAT_TYPE);
    let seat_direction = read_u32(this, FIELD_SEAT_DIRECTION);
    let heading = f32::from_bits(read_u32(this, FIELD_HEADING));
    let _ = lf_checker_rt::callee_cdecl!(2, u32, logger_context, 1u32, 1u32, lf_checker_rt::relocated(FORMAT_SEAT_TYPE), seat_type);
    let _ = lf_checker_rt::callee_cdecl!(2, u32, logger_context, 1u32, 1u32, lf_checker_rt::relocated(FORMAT_SEAT_DIRECTION), seat_direction);
    let (heading_low, heading_high) = double_words(heading);
    let _ = lf_checker_rt::callee_cdecl!(3, u32, logger_context, 1u32, 1u32, lf_checker_rt::relocated(FORMAT_HEADING), heading_low, heading_high);
    let sitting_text = if read_u8(this, FIELD_IS_SITTING) != 0 { TRUE_SITTING } else { FALSE_SITTING };
    let _ = lf_checker_rt::callee_cdecl!(2, u32, logger_context, 1u32, 1u32, lf_checker_rt::relocated(FORMAT_SITTING), lf_checker_rt::relocated(sitting_text));
    let standing_text = if read_u8(this, FIELD_IS_STANDING) != 0 { TRUE_STANDING } else { FALSE_STANDING };
    let _ = lf_checker_rt::callee_cdecl!(2, u32, logger_context, 1u32, 1u32, lf_checker_rt::relocated(FORMAT_STANDING), lf_checker_rt::relocated(standing_text));
    let x = f32::from_bits(read_u32(this, FIELD_POSITION_X));
    let y = f32::from_bits(read_u32(this, FIELD_POSITION_Y));
    let z = f32::from_bits(read_u32(this, FIELD_POSITION_Z));
    let (x_low, x_high) = double_words(x);
    let (y_low, y_high) = double_words(y);
    let (z_low, z_high) = double_words(z);
    let _ = lf_checker_rt::callee_cdecl!(4, u32, logger_context, 1u32, 1u32, lf_checker_rt::relocated(FORMAT_SEAT_POSITION), x_low, x_high, y_low, y_high, z_low, z_high);
});
