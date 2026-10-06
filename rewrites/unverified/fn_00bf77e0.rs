// original: 0x00bf77e0 emit_init_32_wrap
/// Initialise a tag-0x32 record by wrapping the tag-0x31 initialiser.
///
/// Calls the sibling initialiser (thiscall on `this`) with (`a0`, `a1`, 0,
/// `a2`, `a3`) - a fixed 0 in the third slot - then overwrites the tag byte
/// at this `+0` with 0x32 and raises the ready flag 1 at this `+2`.
/// Returns the sibling call's answer. The sibling is intercepted, so this
/// proof needs no dependency on its rewrite.
///
/// Original: 0x00BF77E0 (thiscall, four stack words).
lf_checker_rt::export!(thiscall, rw_00bf77e0(this: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        const TAG: u8 = 0x32;
        const READY: u8 = 1;
        let r = lf_checker_rt::callee_thiscall!(1, u32, this, a0, a1, 0, a2, a3);
        wr8(this, TAG);
        wr8(this + 2, READY);
        r
    }
});
