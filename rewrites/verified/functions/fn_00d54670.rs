// original: 0x00d54670 ccam_init_float_params

/// Write the camera's default float parameter block and mode flag.
///
/// `this` points to the object. Ten tuning constants (stored as their exact
/// 32-bit patterns), one zero word at `+0x14c` and a mode byte of 1 at
/// `+0x18c` are written; nothing is read and no value is returned (EAX
/// passes through untouched, so the return channel is not compared).
///
/// Original: 0x00d54670 (thiscall, no stack arguments, no calls).
lf_checker_rt::export!(thiscall, rw_00d54670(this: u32) -> u32 {
    unsafe {
        const P160: u32 = 0x160;
        const P14C: u32 = 0x14c;
        const MODE: u32 = 0x18c;
        ((this + 0x164) as *mut u32).write_unaligned(0x3f800000);
        ((this + P160) as *mut u32).write_unaligned(0x447a0000);
        ((this + 0x168) as *mut u32).write_unaligned(0x41200000);
        ((this + 0x16c) as *mut u32).write_unaligned(0x4788b800);
        ((this + 0x170) as *mut u32).write_unaligned(0x41200000);
        ((this + 0x174) as *mut u32).write_unaligned(0x41200000);
        ((this + 0x178) as *mut u32).write_unaligned(0x4788b800);
        ((this + 0x17c) as *mut u32).write_unaligned(0x3f800000);
        ((this + 0x180) as *mut u32).write_unaligned(0x3f800000);
        ((this + 0x184) as *mut u32).write_unaligned(0x3f000000);
        ((this + P14C) as *mut u32).write_unaligned(0);
        ((this + MODE) as *mut u8).write(1);
        0
    }
});
