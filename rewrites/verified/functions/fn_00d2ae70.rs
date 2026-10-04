// original: 0x00d2ae70 task_ctor_mid (proposed)
/// Constructor: chain the base constructor, stamp vtable slot `+0`, copy
/// the point at `p1` to `+0x20`, store `f2` at `+0x30`, `a3` at `+0x34`
/// and `f0` at `+0x38`, retain the slot at `+0x34` when non-null, stamp
/// the global time at `+0x3c`, 0x7530 at `+0x40`, and the flag byte at
/// `+0x44`. Returns `this`.
///
/// Thiscall, four stack words.
lf_checker_rt::export!(thiscall, rw_00d2ae70(this: u32, f0: u32, p1: u32, f2: u32, a3: u32) -> u32 {
    unsafe {
        const VT0V: u32 = 0x00ee2164;
        const TIME_GLOB: u32 = 0x011735b4;
        const SPAN: u32 = 0x7530;
        const BASE: u32 = 1;
        const RETAIN: u32 = 2;
        lf_checker_rt::callee_thiscall!(BASE, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VT0V));
        for i in 0..3u32 {
            ((this + 0x20 + i * 4) as *mut u32).write_unaligned(((p1 + i * 4) as *const u32).read_unaligned());
        }
        ((this + 0x30) as *mut u32).write_unaligned(f2);
        ((this + 0x38) as *mut u32).write_unaligned(f0);
        ((this + 0x34) as *mut u32).write_unaligned(a3);
        ((this + 0x3c) as *mut u32).write_unaligned(0);
        ((this + 0x40) as *mut u32).write_unaligned(0);
        ((this + 0x44) as *mut u16).write_unaligned(0);
        if ((this + 0x34) as *const u32).read_unaligned() != 0 {
            lf_checker_rt::callee_stdcall!(RETAIN, u32, this + 0x34);
        }
        let t = lf_checker_rt::global::<u32>(TIME_GLOB).read_unaligned();
        ((this + 0x3c) as *mut u32).write_unaligned(t);
        ((this + 0x40) as *mut u32).write_unaligned(SPAN);
        ((this + 0x44) as *mut u8).write(1);
        this
    }
});
