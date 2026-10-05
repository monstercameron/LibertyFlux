// original: 0x0061D230 net_init_small_rec

/// Initialize a small record: two zero words, a -1 marker, three zeros.
/// Returns `this`.
/// Original: 0x0061D230 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_0061D230(this: u32) -> u32 {
    unsafe {
        ((this) as *mut u32).write_unaligned(0);
        ((this + 4) as *mut u32).write_unaligned(0);
        ((this + 8) as *mut u32).write_unaligned(0xFFFF_FFFF);
        ((this + 0xC) as *mut u32).write_unaligned(0);
        ((this + 0x10) as *mut u32).write_unaligned(0);
        ((this + 0x14) as *mut u32).write_unaligned(0);
        this
    }
});
