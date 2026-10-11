// original: 0x00A34080 CEntity::set_state_timing

/// Updates an entity's timing fields according to a signed start, mode and
/// limit. Negative starts select state 3; positive modes store a byte mode
/// and select state 5; otherwise the signed limit selects state 6 or a
/// fallback based on the four-bit entity type in flags at +0x28. This is a
/// thiscall method with three stack words. The proof pins entry EAX to zero,
/// which is the value preserved by the negative-start branch.
lf_checker_rt::export!(thiscall, rw_00a34080(this: u32, start: u32, mode: u32, limit: u32) -> u32 {
    const FLAGS: u32 = 0x28;
    const STATE: u32 = 0x41;
    const START_TIME: u32 = 0x48;
    const MODE_BYTE: u32 = 0x40;
    const LIMIT_WORD: u32 = 0x44;
    const FALLBACK_WORD: u32 = 0x40;
    const FALLBACK_VALUE: u16 = 0x0700;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn write_u8(address: u32, value: u8) {
        unsafe { (address as *mut u8).write(value) }
    }
    #[inline(always)]
    unsafe fn write_u16(address: u32, value: u16) {
        unsafe { (address as *mut u16).write_unaligned(value) }
    }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        if (start as i32) < 0 {
            write_u8(this + STATE, 3);
            return 0;
        }

        if (mode as i32) > 0 {
            write_u32(this + START_TIME, start);
            write_u8(this + MODE_BYTE, mode as u8);
            write_u8(this + STATE, 5);
            return mode;
        }

        if (limit as i32) > -1 {
            write_u32(this + START_TIME, start);
            write_u16(this + LIMIT_WORD, limit as u16);
            write_u8(this + STATE, 6);
            return mode;
        }

        if mode != 0 {
            write_u8(this + STATE, 3);
            return mode;
        }

        let entity_type = (read_u32(this + FLAGS) >> 6) & 0x0f;
        if entity_type == 4 || entity_type == 5 {
            return entity_type;
        }

        write_u32(this + START_TIME, start);
        write_u16(this + FALLBACK_WORD, FALLBACK_VALUE);
        entity_type
    }
});
