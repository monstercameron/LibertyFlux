// original: 0x0061CF90 net_init_endpoint

/// Initialize an endpoint record.
///
/// Zeroes the header, runs the two sub-initializers over `this+8`, and
/// stamps the state words: -1 markers at 0x48/0x4C/0x78/0x80/0x88,
/// zeroed words at 0x50-0x5F, 0x7C, 0x84 and 0x8C. Returns `this`.
/// Original: 0x0061CF90 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_0061CF90(this: u32) -> u32 {
    unsafe {
        const SUB_INIT_A: u32 = 1;
        const SUB_INIT_B: u32 = 2;
        let w = |off: u32, v: u32| ((this + off) as *mut u32).write_unaligned(v);
        w(0x00, 0);
        w(0x04, 0);
        lf_checker_rt::callee_thiscall!(SUB_INIT_A, u32, this + 8);
        w(0x48, 0xFFFF_FFFF);
        w(0x4C, 0xFFFF_FFFF);
        w(0x50, 0);
        w(0x54, 0);
        w(0x58, 0);
        w(0x5C, 0);
        ((this + 0x58) as *mut u16).write_unaligned(0);
        lf_checker_rt::callee_thiscall!(SUB_INIT_B, u32, this + 8);
        w(0x78, 0xFFFF_FFFF);
        w(0x80, 0xFFFF_FFFF);
        ((this + 0x7C) as *mut u16).write_unaligned(0);
        w(0x78, 0xFFFF_FFFF);
        ((this + 0x84) as *mut u16).write_unaligned(0);
        w(0x80, 0xFFFF_FFFF);
        w(0x8C, 0);
        w(0x88, 0xFFFF_FFFF);
        this
    }
});
