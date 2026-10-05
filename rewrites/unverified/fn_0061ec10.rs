// original: 0x0061EC10 net_init_peer_rec

/// Initialize a peer record.
///
/// Marks the header, runs the sub-initializer over `this+8`, stamps
/// the -1/0 state pattern over 0x48-0x74, and sets flag bit 1 (clearing
/// bit 0) in the mode byte at `this+0x78`. Returns `this`.
/// Original: 0x0061EC10 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_0061EC10(this: u32) -> u32 {
    unsafe {
        const SUB_INIT: u32 = 1;
        let w = |off: u32, v: u32| ((this + off) as *mut u32).write_unaligned(v);
        let wh = |off: u32, v: u16| ((this + off) as *mut u16).write_unaligned(v);
        w(0x00, 0xFFFF_FFFF);
        lf_checker_rt::callee_thiscall!(SUB_INIT, u32, this + 8);
        w(0x48, 0xFFFF_FFFF);
        w(0x50, 0xFFFF_FFFF);
        w(0x48, 0xFFFF_FFFF);
        wh(0x4C, 0);
        wh(0x54, 0);
        w(0x50, 0xFFFF_FFFF);
        w(0x58, 0xFFFF_FFFF);
        w(0x60, 0xFFFF_FFFF);
        wh(0x5C, 0);
        w(0x58, 0xFFFF_FFFF);
        wh(0x64, 0);
        w(0x60, 0xFFFF_FFFF);
        w(0x6C, 0);
        w(0x70, 0);
        w(0x74, 0);
        let mode = ((this + 0x78) as *const u8).read();
        ((this + 0x78) as *mut u8).write((mode & 0xFE) | 2);
        this
    }
});
