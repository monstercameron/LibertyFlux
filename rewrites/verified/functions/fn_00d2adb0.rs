// original: 0x00d2adb0 task_ctor_full_b (proposed)
/// Constructor: same shape as 0x00d2acf0 with another vtable: chain the
/// base constructor, copy the points at `p1`/`p3`/`p6`, store the scalars,
/// retain the slot at `+0x44` when non-null, stamp the global time at
/// `+0x4c`, set the flag byte, and store `a5` (or 0x2710 when negative) at
/// `+0x50`. Returns `this`.
///
/// Thiscall, seven stack words.
lf_checker_rt::export!(thiscall, rw_00d2adb0(this: u32, f0: u32, p1: u32, f2: u32, p3: u32, a4: u32, a5: u32, p6: u32) -> u32 {
    unsafe {
        const VT0V: u32 = 0x00ee210c;
        const TIME_GLOB: u32 = 0x011735b4;
        const NEG_FALLBACK: u32 = 0x2710;
        const BASE: u32 = 1;
        const RETAIN: u32 = 2;
        lf_checker_rt::callee_thiscall!(BASE, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VT0V));
        for i in 0..3u32 {
            ((this + 0x20 + i * 4) as *mut u32).write_unaligned(((p1 + i * 4) as *const u32).read_unaligned());
            ((this + 0x30 + i * 4) as *mut u32).write_unaligned(((p3 + i * 4) as *const u32).read_unaligned());
        }
        ((this + 0x40) as *mut u32).write_unaligned(f2);
        ((this + 0x44) as *mut u32).write_unaligned(a4);
        ((this + 0x48) as *mut u32).write_unaligned(f0);
        ((this + 0x4c) as *mut u32).write_unaligned(0);
        ((this + 0x50) as *mut u32).write_unaligned(0);
        ((this + 0x54) as *mut u16).write_unaligned(0);
        ((this + 0x58) as *mut u32).write_unaligned(a5);
        for i in 0..3u32 {
            ((this + 0x60 + i * 4) as *mut u32).write_unaligned(((p6 + i * 4) as *const u32).read_unaligned());
        }
        if ((this + 0x44) as *const u32).read_unaligned() != 0 {
            lf_checker_rt::callee_stdcall!(RETAIN, u32, this + 0x44);
        }
        let t = lf_checker_rt::global::<u32>(TIME_GLOB).read_unaligned();
        ((this + 0x4c) as *mut u32).write_unaligned(t);
        ((this + 0x54) as *mut u8).write(1);
        let v = ((this + 0x58) as *const u32).read_unaligned();
        ((this + 0x50) as *mut u32).write_unaligned(if (v as i32) >= 0 { v } else { NEG_FALLBACK });
        this
    }
});
