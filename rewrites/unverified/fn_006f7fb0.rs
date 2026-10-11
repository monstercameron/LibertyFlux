// original: 0x006F7FB0 mainloop_timer_default_pointer

/// Return the fixed timer object for selector 1, otherwise return the global
/// default object only for selector 0. The global is a pointer-sized game
/// word; every other selector returns null.
lf_checker_rt::export!(thiscall, rw_006f7fb0(selector: u32) -> u32 {
    if selector == 1 {
        lf_checker_rt::relocated(0x00fae0f8)
    } else if selector == 0 {
        unsafe { lf_checker_rt::global::<u32>(0x011104ac).read_unaligned() }
    } else {
        0
    }
});
