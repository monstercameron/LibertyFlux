// original: 0x0061DA60 net_init_peer_slots

/// Initialize a peer block and clear its slot arrays.
///
/// Zeroes the header, runs the sub-initializer over `this+8`, then for
/// 32 slots zeroes the word at `this+0x10+i*4` and the flag byte at
/// `this+0x90+i*0x18`. Returns `this`.
/// Original: 0x0061DA60 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_0061DA60(this: u32) -> u32 {
    unsafe {
        const SUB_INIT: u32 = 1;
        const SLOTS: u32 = 0x20;
        ((this) as *mut u32).write_unaligned(0);
        ((this + 4) as *mut u32).write_unaligned(0);
        ((this + 0xC) as *mut u32).write_unaligned(0);
        lf_checker_rt::callee_thiscall!(SUB_INIT, u32, this + 8);
        for i in 0..SLOTS {
            ((this + 0x10 + i * 4) as *mut u32).write_unaligned(0);
            ((this + 0x90 + i * 0x18) as *mut u8).write(0);
        }
        this
    }
});
