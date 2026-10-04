// original: 0x00d2ac10 task_ctor_xyz_pair (proposed)
/// Constructor: chain the base constructor with `speed`, stamp the two
/// vtable slots (`+0`, `+0x14`), copy the three-dword point at `pos` to
/// `+0x20` and again to `+0x30`, store `extra` at `+0x40`, zero `+0x44`
/// through `+0x5c`, and return `this`.
///
/// Thiscall, three stack words (float bits, pointer, word).
lf_checker_rt::export!(thiscall, rw_00d2ac10(this: u32, speed: u32, pos: u32, extra: u32) -> u32 {
    unsafe {
        const VT0V: u32 = 0x00ee224c;
        const VT14V: u32 = 0x00ee22a4;
        const BASE: u32 = 1;
        lf_checker_rt::callee_thiscall!(BASE, u32, this, speed);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VT0V));
        ((this + 0x14) as *mut u32).write_unaligned(lf_checker_rt::relocated(VT14V));
        for i in 0..3u32 {
            let w = ((pos + i * 4) as *const u32).read_unaligned();
            ((this + 0x20 + i * 4) as *mut u32).write_unaligned(w);
            ((this + 0x30 + i * 4) as *mut u32).write_unaligned(w);
        }
        ((this + 0x40) as *mut u32).write_unaligned(extra);
        ((this + 0x44) as *mut u32).write_unaligned(0);
        ((this + 0x48) as *mut u32).write_unaligned(0);
        ((this + 0x4c) as *mut u16).write_unaligned(0);
        ((this + 0x50) as *mut u32).write_unaligned(0);
        ((this + 0x54) as *mut u32).write_unaligned(0);
        ((this + 0x58) as *mut u32).write_unaligned(0);
        ((this + 0x5c) as *mut u8).write(0);
        this
    }
});
