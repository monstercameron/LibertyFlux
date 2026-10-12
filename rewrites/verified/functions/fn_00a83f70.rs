// original: 0x00A83F70 CSimpleCarDriveTaskInfo::append_drive_flags

/// Forward the base task report, then log door, engine, horn, siren, air-horn and full-beam booleans from the six documented bits of the byte at +0x1C.
lf_checker_rt::export!(thiscall, rw_00a83f70(this: u32, logger_context: u32) -> () {

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

    const FIELD_DRIVE_FLAGS: u32 = 0x1C;
    let flags = read_u8(this, FIELD_DRIVE_FLAGS);
    const DOOR_CLOSED_MASK: u8 = 0x01;
    const DOOR_CLOSED_FORMAT: u32 = 0x00EA0E64;
    const DOOR_CLOSED_FALSE: u32 = 0x00EA0E5C;
    const DOOR_CLOSED_TRUE: u32 = 0x00EA0E54;
    const ENGINE_ON_MASK: u8 = 0x02;
    const ENGINE_ON_FORMAT: u32 = 0x00EA0E88;
    const ENGINE_ON_FALSE: u32 = 0x00EA0E80;
    const ENGINE_ON_TRUE: u32 = 0x00EA0E78;
    const HORN_ON_MASK: u8 = 0x08;
    const HORN_ON_FORMAT: u32 = 0x00EA0EAC;
    const HORN_ON_FALSE: u32 = 0x00EA0EA4;
    const HORN_ON_TRUE: u32 = 0x00EA0E9C;
    const SIREN_ON_MASK: u8 = 0x08;
    const SIREN_ON_FORMAT: u32 = 0x00EA0ED0;
    const SIREN_ON_FALSE: u32 = 0x00EA0EC8;
    const SIREN_ON_TRUE: u32 = 0x00EA0EC0;
    const AIR_HORN_ON_MASK: u8 = 0x10;
    const AIR_HORN_ON_FORMAT: u32 = 0x00EA0EF4;
    const AIR_HORN_ON_FALSE: u32 = 0x00EA0EEC;
    const AIR_HORN_ON_TRUE: u32 = 0x00EA0EE4;
    const FULL_BEAM_MASK: u8 = 0x20;
    const FULL_BEAM_FORMAT: u32 = 0x00EA0F18;
    const FULL_BEAM_FALSE: u32 = 0x00EA0F10;
    const FULL_BEAM_TRUE: u32 = 0x00EA0F08;
    let _ = lf_checker_rt::callee_thiscall!(1, u32, this, logger_context);
    let text_door_closed = if flags & DOOR_CLOSED_MASK != 0 { DOOR_CLOSED_TRUE } else { DOOR_CLOSED_FALSE };
    let _ = lf_checker_rt::callee_cdecl!(2, u32, logger_context, 1u32, 1u32, lf_checker_rt::relocated(DOOR_CLOSED_FORMAT), lf_checker_rt::relocated(text_door_closed));
    let text_engine_on = if flags & ENGINE_ON_MASK != 0 { ENGINE_ON_TRUE } else { ENGINE_ON_FALSE };
    let _ = lf_checker_rt::callee_cdecl!(2, u32, logger_context, 1u32, 1u32, lf_checker_rt::relocated(ENGINE_ON_FORMAT), lf_checker_rt::relocated(text_engine_on));
    let text_horn_on = if flags & HORN_ON_MASK != 0 { HORN_ON_TRUE } else { HORN_ON_FALSE };
    let _ = lf_checker_rt::callee_cdecl!(2, u32, logger_context, 1u32, 1u32, lf_checker_rt::relocated(HORN_ON_FORMAT), lf_checker_rt::relocated(text_horn_on));
    let text_siren_on = if flags & SIREN_ON_MASK != 0 { SIREN_ON_TRUE } else { SIREN_ON_FALSE };
    let _ = lf_checker_rt::callee_cdecl!(2, u32, logger_context, 1u32, 1u32, lf_checker_rt::relocated(SIREN_ON_FORMAT), lf_checker_rt::relocated(text_siren_on));
    let text_air_horn_on = if flags & AIR_HORN_ON_MASK != 0 { AIR_HORN_ON_TRUE } else { AIR_HORN_ON_FALSE };
    let _ = lf_checker_rt::callee_cdecl!(2, u32, logger_context, 1u32, 1u32, lf_checker_rt::relocated(AIR_HORN_ON_FORMAT), lf_checker_rt::relocated(text_air_horn_on));
    let text_full_beam = if flags & FULL_BEAM_MASK != 0 { FULL_BEAM_TRUE } else { FULL_BEAM_FALSE };
    let _ = lf_checker_rt::callee_cdecl!(2, u32, logger_context, 1u32, 1u32, lf_checker_rt::relocated(FULL_BEAM_FORMAT), lf_checker_rt::relocated(text_full_beam));
});
