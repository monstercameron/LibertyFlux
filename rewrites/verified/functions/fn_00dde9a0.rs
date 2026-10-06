// original: 0x00DDE9A0 UITextField mode reset
/// UITextField helper (input-ui subsystem). `this` is the field object.
/// Reset the field mode. When the low byte of `reset` is nonzero, clear the
/// mode word at `+0x210` and reload the parameter at `+0x200` from the shared
/// configuration word; otherwise, when the current mode is 0, 2 or 3, force it
/// to 4 (any other mode is left alone). Returns nothing.
lf_checker_rt::export!(thiscall, rw_00DDE9A0(this: u32, reset: u32) -> u32 {
    unsafe {
        const MODE: u32 = 0x210;
        const PARAM: u32 = 0x200;
        const CONFIG_WORD: u32 = 0x01173594;
        if (reset as u8) != 0 {
            ((this + MODE) as *mut u32).write_unaligned(0);
            let cfg = (lf_checker_rt::global::<u32>(CONFIG_WORD) as *const u32).read_unaligned();
            ((this + PARAM) as *mut u32).write_unaligned(cfg);
        } else {
            let mode = ((this + MODE) as *const u32).read_unaligned();
            if mode == 0 || mode == 2 || mode == 3 {
                ((this + MODE) as *mut u32).write_unaligned(4);
            }
        }
        0
    }
});
