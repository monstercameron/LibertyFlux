// original: 0x00d2aef0 task_ctor_tagged_float (proposed)
/// Constructor: chain the base constructor with (`a0`, `a1`, `a2`, `f3`),
/// stamp the two vtable slots (`+0`, `+0x14`), and return `this`.
///
/// Thiscall, four stack words (three words, float bits).
lf_checker_rt::export!(thiscall, rw_00d2aef0(this: u32, a0: u32, a1: u32, a2: u32, f3: u32) -> u32 {
    unsafe {
        const VT0V: u32 = 0x00ee2024;
        const VT14V: u32 = 0x00ee2078;
        const BASE: u32 = 1;
        lf_checker_rt::callee_thiscall!(BASE, u32, this, a0, a1, a2, f3);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VT0V));
        ((this + 0x14) as *mut u32).write_unaligned(lf_checker_rt::relocated(VT14V));
        this
    }
});
