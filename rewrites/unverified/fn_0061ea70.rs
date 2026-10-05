// original: 0x0061EA70 net_copy_addr_rec

/// Copy an address record and its three trailing arguments.
///
/// Copies the dword/word/dword/word address block at `a0` to `this`,
/// stores `a1`/`a2`/`a3` at `this+0x10`/`+0x14`/`+0x18`, stores `a4` at
/// `this+0x220` and zeroes `this+0x21C`. Returns `this`.
/// Original: 0x0061EA70 (thiscall, five stack words).
lf_checker_rt::export!(thiscall, rw_0061EA70(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        (this as *mut u32).write_unaligned((a0 as *const u32).read_unaligned());
        ((this + 4) as *mut u16).write_unaligned(((a0 + 4) as *const u16).read_unaligned());
        ((this + 8) as *mut u32).write_unaligned(((a0 + 8) as *const u32).read_unaligned());
        ((this + 0xC) as *mut u16).write_unaligned(((a0 + 0xC) as *const u16).read_unaligned());
        ((this + 0x10) as *mut u32).write_unaligned(a1);
        ((this + 0x14) as *mut u32).write_unaligned(a2);
        ((this + 0x18) as *mut u32).write_unaligned(a3);
        ((this + 0x220) as *mut u32).write_unaligned(a4);
        ((this + 0x21C) as *mut u32).write_unaligned(0);
        this
    }
});
