// original: 0x008BF490 ui_element_init_core (proposed)

/// Initialise the core fields of an input-ui element.
///
/// Sets the active byte at the object base, clears the dword at +0x0c and the
/// byte at +0x10. Called by the fuller element initialisers after they have
/// written their own fields; the caller passes the object in ecx and cleans
/// nothing (thiscall, no stack words). Returns nothing meaningful: the
/// original is a plain `ret` that leaves the incoming eax in place, so the
/// contract compares no return channel.
lf_checker_rt::export!(thiscall, rw_008BF490(this: u32) -> u32 {
    unsafe {
        /// Active flag at the object base.
        const ACTIVE: u32 = 0x00;
        /// Counter/state dword.
        const STATE: u32 = 0x0C;
        /// Secondary flag byte.
        const FLAG: u32 = 0x10;
        ((this + ACTIVE) as *mut u8).write(1);
        ((this + STATE) as *mut u32).write_unaligned(0);
        ((this + FLAG) as *mut u8).write(0);
        0
    }
});
