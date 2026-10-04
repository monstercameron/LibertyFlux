// original: 0x00d2aca0 task_ctor_xyz_float (proposed)
/// Constructor: chain the base constructor with `speed`, stamp the two
/// vtable slots (`+0`, `+0x14`), copy the three-dword point at `pos` to
/// `+0x20`, store `limit` at `+0x30`, and return `this`.
///
/// Thiscall, three stack words (float bits, pointer, float bits).
lf_checker_rt::export!(thiscall, rw_00d2aca0(this: u32, speed: u32, pos: u32, limit: u32) -> u32 {
    unsafe {
        const VT0V: u32 = 0x00ee21bc;
        const VT14V: u32 = 0x00ee2214;
        const BASE: u32 = 1;
        lf_checker_rt::callee_thiscall!(BASE, u32, this, speed);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VT0V));
        ((this + 0x14) as *mut u32).write_unaligned(lf_checker_rt::relocated(VT14V));
        for i in 0..3u32 {
            let w = ((pos + i * 4) as *const u32).read_unaligned();
            ((this + 0x20 + i * 4) as *mut u32).write_unaligned(w);
        }
        ((this + 0x30) as *mut u32).write_unaligned(limit);
        this
    }
});
