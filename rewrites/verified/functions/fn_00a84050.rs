// original: 0x00A84050 CSimpleClimbTaskInfo::append_climb_state

/// Forward the base report, then log the climb-type byte at +0x18, three zero/nonzero status bytes at +0x19 through +0x1B, and the three f32 coordinates at +0x20, +0x24 and +0x28 after exact f32-to-f64 widening.
lf_checker_rt::export!(thiscall, rw_00a84050(this: u32, logger_context: u32) -> () {

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

    const FIELD_CLIMB_TYPE: u32 = 0x18;
    const FIELD_SHIMMY_LEFT: u32 = 0x19;
    const FIELD_SHIMMY_RIGHT: u32 = 0x1A;
    const FIELD_PULLING_UP: u32 = 0x1B;
    const FORMAT_CLIMB_TYPE: u32 = 0x00EA0784;
    let _ = lf_checker_rt::callee_thiscall!(1, u32, this, logger_context);
    let climb_type = read_u8(this, FIELD_CLIMB_TYPE) as u32;
    let _ = lf_checker_rt::callee_cdecl!(2, u32, logger_context, 1u32, 1u32, lf_checker_rt::relocated(FORMAT_CLIMB_TYPE), climb_type);
    const FORMAT_STATE_0: u32 = 0x00EA07AC;
    const FALSE_STATE_0: u32 = 0x00EA07A4;
    const TRUE_STATE_0: u32 = 0x00EA079C;
    let state_0 = read_u8(this, FIELD_SHIMMY_LEFT);
    let text_0 = if state_0 != 0 { TRUE_STATE_0 } else { FALSE_STATE_0 };
    let _ = lf_checker_rt::callee_cdecl!(2, u32, logger_context, 1u32, 1u32, lf_checker_rt::relocated(FORMAT_STATE_0), lf_checker_rt::relocated(text_0));
    const FORMAT_STATE_1: u32 = 0x00EA07E4;
    const FALSE_STATE_1: u32 = 0x00EA07DC;
    const TRUE_STATE_1: u32 = 0x00EA07D4;
    let state_1 = read_u8(this, FIELD_SHIMMY_RIGHT);
    let text_1 = if state_1 != 0 { TRUE_STATE_1 } else { FALSE_STATE_1 };
    let _ = lf_checker_rt::callee_cdecl!(2, u32, logger_context, 1u32, 1u32, lf_checker_rt::relocated(FORMAT_STATE_1), lf_checker_rt::relocated(text_1));
    const FORMAT_STATE_2: u32 = 0x00EA080C;
    const FALSE_STATE_2: u32 = 0x00EA0804;
    const TRUE_STATE_2: u32 = 0x00EA07FC;
    let state_2 = read_u8(this, FIELD_PULLING_UP);
    let text_2 = if state_2 != 0 { TRUE_STATE_2 } else { FALSE_STATE_2 };
    let _ = lf_checker_rt::callee_cdecl!(2, u32, logger_context, 1u32, 1u32, lf_checker_rt::relocated(FORMAT_STATE_2), lf_checker_rt::relocated(text_2));
    const FIELD_X: u32 = 0x20;
    const FIELD_Y: u32 = 0x24;
    const FIELD_Z: u32 = 0x28;
    const FORMAT_GRAB_POSITION: u32 = 0x00EA0824;
    let x = f32::from_bits(read_u32(this, FIELD_X));
    let y = f32::from_bits(read_u32(this, FIELD_Y));
    let z = f32::from_bits(read_u32(this, FIELD_Z));
    let (x_low, x_high) = double_words(x);
    let (y_low, y_high) = double_words(y);
    let (z_low, z_high) = double_words(z);
    let _ = lf_checker_rt::callee_cdecl!(3, u32, logger_context, 1u32, 1u32, lf_checker_rt::relocated(FORMAT_GRAB_POSITION), x_low, x_high, y_low, y_high, z_low, z_high);
});
