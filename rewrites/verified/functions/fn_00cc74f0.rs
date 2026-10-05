// original: 0x00CC74F0 euphoria_widget_ctor_args (proposed)

/// Argument constructor: forward three words to the base, store the fourth.
///
/// Calls the base constructor on `this` with (`b1`, `b2`, `b3`), stores `a0`
/// at `+0xa0`, writes the primary vtable at `+0` and the secondary vtable at
/// `+0x50`, and clears `+0xa4` and `+0xc`. Returns `this`.
///
/// Original: 0x00CC74F0 (thiscall, four stack words).
lf_checker_rt::export!(thiscall, rw_00cc74f0(this: u32, a0: u32, b1: u32, b2: u32, b3: u32) -> u32 {
    unsafe {
        const BASE_CALLEE: u32 = 1;
        const VTABLE_MAIN: u32 = 0x00ED9814;
        const VTABLE_SUB: u32 = 0x00ED991C;
        const SUB_VTABLE_AT: u32 = 0x50;
        const ARG_AT: u32 = 0xa0;
        const STATE_AT: u32 = 0xa4;
        const TAG_AT: u32 = 0x0c;
        lf_checker_rt::callee_thiscall!(BASE_CALLEE, u32, this, b1, b2, b3);
        (this.wrapping_add(ARG_AT) as *mut u32).write_unaligned(a0);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE_MAIN));
        (this.wrapping_add(SUB_VTABLE_AT) as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(VTABLE_SUB));
        (this.wrapping_add(STATE_AT) as *mut u32).write_unaligned(0);
        (this.wrapping_add(TAG_AT) as *mut u32).write_unaligned(0);
        this
    }
});
