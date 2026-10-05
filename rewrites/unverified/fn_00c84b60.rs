// original: 0x00c84b60 scenario_clock_format (proposed) — UNVERIFIED (deferred)

// NOTE: deferred with reason `other`: four absolute reads of writable game
// data in this range carry no relocations, so the relocated worker cannot
// serve them (the read-only shadow does not apply and is unavailable on this
// machine anyway). The rewrite mirrors the logic against the relocated image
// and is kept for a re-run if that gap ever closes; never passed, not verified.

/// Format the scenario clock as five characters into `dst`.
///
/// Hours and minutes each come from an override-or-live global pair (the
/// override unless it reads -1); each is rendered as two decimal digits, the
/// tens by strength-reduced division and the ones by remainder. Between them
/// goes ':' or ' ' depending on bit 5 of a scaled tick counter. Returns the
/// minutes quotient. All arithmetic is signed 32-bit.
///
/// Original: cdecl, one stack word (plain `ret`).
lf_checker_rt::export!(cdecl, rw_00c84b60(dst: u32) -> u32 {
    unsafe {
        const H_OVR: u32 = 0x1295854;
        const H_LIVE: u32 = 0x1295848;
        const M_OVR: u32 = 0x1295858;
        const M_LIVE: u32 = 0x129584c;
        const TICK: u32 = 0x11735b4;
        const NONE: i32 = -1;
        let g = |va: u32| lf_checker_rt::global::<i32>(va).read_unaligned();
        let h_ovr = g(H_OVR);
        let hours = if h_ovr != NONE { h_ovr } else { g(H_LIVE) };
        if hours < 10 {
            (dst as *mut u8).write(b' ');
        } else {
            (dst as *mut u8).write((hours / 10) as u8 + b'0');
        }
        ((dst + 1) as *mut u8).write((hours % 10) as u8 + b'0');
        let tick = lf_checker_rt::global::<u32>(TICK).read_unaligned();
        let blink = (tick.wrapping_mul(0x10624dd3) >> 5) & 1;
        ((dst + 2) as *mut u8).write(if blink == 0 { b':' } else { b' ' });
        let m_ovr = g(M_OVR);
        let mins = if m_ovr != NONE { m_ovr } else { g(M_LIVE) };
        ((dst + 3) as *mut u8).write((mins / 10) as u8 + b'0');
        ((dst + 4) as *mut u8).write((mins % 10) as u8 + b'0');
        (mins / 10) as u32
    }
});
