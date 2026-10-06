// original: 0x00693870 comp_ctor_forward5 (proposed)

/// Initialises the object at `this` (dwords at +0, +4, +8, +0xc and the
/// word at +0x10 all zero) and forwards its five arguments unchanged to
/// the intercepted initialiser with `this` in ECX. Returns `this`.
///
/// Original: 0x00693870 (thiscall, five stack arguments, callee-cleanup).
lf_checker_rt::export!(thiscall, rw_00693870(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        const INIT: u32 = 1;

        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }

        wr32(this, 0);
        wr32(this.wrapping_add(4), 0);
        wr32(this.wrapping_add(8), 0);
        wr32(this.wrapping_add(0x0c), 0);
        wr16(this.wrapping_add(0x10), 0);
        lf_checker_rt::callee_thiscall!(INIT, u32, this, a0, a1, a2, a3, a4);
        this
    }
});
