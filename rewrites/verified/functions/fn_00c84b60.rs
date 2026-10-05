// original: 0x00c84b60 scenario_clock_format
/// Format the scenario clock as five characters into `dst`.
///
/// Hours and minutes each come from an override-or-live global pair (the
/// override unless it reads -1). The hours tens digit is a blank when hours
/// is below 10, otherwise the quotient of hours by 10; every other digit is a
/// plain quotient or remainder by 10 (signed 32-bit division, low byte kept).
/// The middle character is ':' or ' ' from bit 5 of the high dword of the
/// unsigned product `tick * 0x10624DD3` (clear gives ':', set gives ' ').
/// Returns the minutes quotient (minutes divided by 10).
///
/// Original: cdecl, one stack word (`dst`).
lf_checker_rt::export!(cdecl, rw_00c84b60(dst: u32) -> u32 {
    unsafe {
        const H_OVR: u32 = 0x1295854;
        const H_LIVE: u32 = 0x1295848;
        const M_OVR: u32 = 0x1295858;
        const M_LIVE: u32 = 0x129584c;
        const TICK: u32 = 0x11735b4;
        const NONE: i32 = -1;
        const BLINK_MUL: u64 = 0x10624dd3;
        let g = |va: u32| lf_checker_rt::global::<i32>(va).read_unaligned();
        let h_ovr = g(H_OVR);
        let hours = if h_ovr != NONE { h_ovr } else { g(H_LIVE) };
        if hours < 10 {
            (dst as *mut u8).write(b' ');
        } else {
            (dst as *mut u8).write(((hours / 10) as u8).wrapping_add(b'0'));
        }
        ((dst + 1) as *mut u8).write(((hours % 10) as u8).wrapping_add(b'0'));
        let tick = lf_checker_rt::global::<u32>(TICK).read_unaligned();
        let prod = (tick as u64).wrapping_mul(BLINK_MUL);
        let blink = ((prod >> 37) & 1) as u8;
        ((dst + 2) as *mut u8).write(if blink == 0 { b':' } else { b' ' });
        let m_ovr = g(M_OVR);
        let mins = if m_ovr != NONE { m_ovr } else { g(M_LIVE) };
        ((dst + 3) as *mut u8).write(((mins / 10) as u8).wrapping_add(b'0'));
        ((dst + 4) as *mut u8).write(((mins % 10) as u8).wrapping_add(b'0'));
        (mins / 10) as u32
    }
});

/// Wrong version of rw_00c84b60: the blink separator is inverted
/// (a blank where the original writes ':' and vice versa).
lf_checker_rt::export!(cdecl, mut_00c84b60(dst: u32) -> u32 {
    unsafe {
        const H_OVR: u32 = 0x1295854;
        const H_LIVE: u32 = 0x1295848;
        const M_OVR: u32 = 0x1295858;
        const M_LIVE: u32 = 0x129584c;
        const TICK: u32 = 0x11735b4;
        const NONE: i32 = -1;
        const BLINK_MUL: u64 = 0x10624dd3;
        let g = |va: u32| lf_checker_rt::global::<i32>(va).read_unaligned();
        let h_ovr = g(H_OVR);
        let hours = if h_ovr != NONE { h_ovr } else { g(H_LIVE) };
        if hours < 10 {
            (dst as *mut u8).write(b' ');
        } else {
            (dst as *mut u8).write(((hours / 10) as u8).wrapping_add(b'0'));
        }
        ((dst + 1) as *mut u8).write(((hours % 10) as u8).wrapping_add(b'0'));
        let tick = lf_checker_rt::global::<u32>(TICK).read_unaligned();
        let prod = (tick as u64).wrapping_mul(BLINK_MUL);
        let blink = ((prod >> 37) & 1) as u8;
        // MUTANT: inverted separator.
        ((dst + 2) as *mut u8).write(if blink == 0 { b' ' } else { b':' });
        let m_ovr = g(M_OVR);
        let mins = if m_ovr != NONE { m_ovr } else { g(M_LIVE) };
        ((dst + 3) as *mut u8).write(((mins / 10) as u8).wrapping_add(b'0'));
        ((dst + 4) as *mut u8).write(((mins % 10) as u8).wrapping_add(b'0'));
        (mins / 10) as u32
    }
});
