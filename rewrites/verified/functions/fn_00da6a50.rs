// original: 0x00DA6A50 task_vt_eefa6c_ctor (proposed)

/// Constructor of the task class whose virtual table is 0x00EEFA6C: build
/// the header, install the table, copy the position triple, fold two flag
/// bytes into the status word, set the mode word to 3, clear the timer
/// block, then stamp the timer when a deadline is given.
///
/// `pos` points at three words copied to `+0x30..+0x38`; `speed` is a
/// float kept at `+0x54`; `deadline` is kept at `+0x50` (unless -1, it
/// stamps `+0x5c` with the tick global, `+0x60` with itself and `+0x64`
/// with 1); bit 0 of `flags_lo` becomes bit 0 of `+0x6c`, bit 0 of
/// `flags_hi` becomes bit 3, the other bits of `+0x6c` are kept.
/// Returns `this`. Original: thiscall, five stack words, callee cleanup.
lf_checker_rt::export!(thiscall, rw_00da6a50(this: u32, pos: u32, flags_lo: u32, speed: u32, deadline: u32, flags_hi: u32) -> u32 {
    unsafe {
        const HEADER_CTOR: u32 = 1;
        const VTABLE: u32 = 0x00EEFA6C;
        const TICK_SLOT: u32 = 0x011735B4;
        const NO_DEADLINE: u32 = 0xFFFF_FFFF;

        lf_checker_rt::callee_thiscall!(HEADER_CTOR, u32, this);
        (this as *mut u32).write(lf_checker_rt::relocated(VTABLE));
        for i in 0..3u32 {
            let w = ((pos + i * 4) as *const u32).read();
            ((this + 0x30 + i * 4) as *mut u32).write(w);
        }
        ((this + 0x50) as *mut u32).write(deadline);
        let bits = ((flags_hi & 1) << 3) | (flags_lo & 1);
        ((this + 0x54) as *mut u32).write(speed);
        ((this + 0x58) as *mut u32).write(3);
        ((this + 0x5C) as *mut u32).write(0);
        ((this + 0x60) as *mut u32).write(0);
        ((this + 0x64) as *mut u16).write(0);
        let status = (this + 0x6C) as *mut u32;
        status.write((status.read() & 0xFFFF_FFE0) | bits);
        ((this + 0x68) as *mut u8).write(0);
        if deadline != NO_DEADLINE {
            let tick = (lf_checker_rt::relocated(TICK_SLOT) as *const u32).read();
            ((this + 0x5C) as *mut u32).write(tick);
            ((this + 0x60) as *mut u32).write(deadline);
            ((this + 0x64) as *mut u8).write(1);
        }
        this
    }
});
