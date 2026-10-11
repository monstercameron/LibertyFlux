// original: 0x006F7F60 mainloop_timer_category_pointer

/// Map the selector in ECX to one of five relocated timer category objects.
/// Values above 4 return null using an unsigned comparison, so all high-bit
/// values take the same path. No memory is read or written.
lf_checker_rt::export!(thiscall, rw_006f7f60(selector: u32) -> u32 {
    match selector {
        0 => lf_checker_rt::relocated(0x00fae120),
        1 => lf_checker_rt::relocated(0x00fae118),
        2 => lf_checker_rt::relocated(0x00fae114),
        3 => lf_checker_rt::relocated(0x00fae10c),
        4 => lf_checker_rt::relocated(0x00fae104),
        _ => 0,
    }
});
