// original: 0x00E60B90 timer_state_reset (proposed)

/// Timer-state reset plus callback registration.
///
/// Behaviour: writes 0 to the two state words at `STATE`, reads both back
/// (both zero), clears bit 0 of the flag byte at `FLAG`, writes 0 to the
/// five counter words at `COUNTERS` and 100 to the limit word after them,
/// then calls the cdecl registrar with the callback address `CALLBACK`
/// and returns its answer. The stores happen in exactly this order
/// (counters after the registrar argument is pushed, limit last).
///
/// Original: cdecl, no stack arguments; EAX/ECX hold the two re-read
/// zeroes across the registrar call. Return value is the registrar's
/// answer (EAX is not otherwise modified after the re-read).
lf_checker_rt::export!(cdecl, rw_00e60b90() -> u32 {
    unsafe {
        const LIMIT: u32 = 100;
        let s0 = lf_checker_rt::relocated(0x0019F39E4) as *mut u32;
        s0.write_unaligned(0);
        (s0.add(1)).write_unaligned(0);
        let _a: u32 = s0.read_unaligned();
        let _c: u32 = (s0.add(1)).read_unaligned();
        let flag = lf_checker_rt::relocated(0x0019F3A04) as *mut u8;
        flag.write_unaligned(flag.read_unaligned() & 0xFE);
        let c0 = lf_checker_rt::relocated(0x0019F39EC) as *mut u32;
        c0.write_unaligned(0);
        (c0.add(1)).write_unaligned(0);
        (c0.add(2)).write_unaligned(0);
        (c0.add(3)).write_unaligned(0);
        (c0.add(4)).write_unaligned(0);
        (c0.add(5)).write_unaligned(LIMIT);
        lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(0x00E6FC30))
    }
});

