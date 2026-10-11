// original: 0x005E0BE0 CTaskComplexWanderCriminal::CTaskComplexWanderCriminal

/// Construct the wander base with the two stack parameters unchanged and
/// forward the low word of XMM2 as its second stack argument. The helper also
/// receives the fixed 0.5f value and two integer flags. Then install the
/// derived vtable, clear the counters at +0x80 through +0x8c, and return
/// `this`. The XMM2 float is transported bit for bit; this routine does no
/// arithmetic on it.
lf_checker_rt::export!(thiscall, rw_005e0be0(this: u32, _arg0: u32, _arg1: u32) -> u32 {
    unsafe {
        const DERIVED_VTABLE: u32 = 0x00fe_0f34;
        const FIRST_COUNTER: u32 = 0x80;
        const SECOND_COUNTER: u32 = 0x84;
        const THIRD_COUNTER: u32 = 0x88;
        const FOURTH_COUNTER: u32 = 0x8c;
        const HALF_BITS: u32 = 0x3f00_0000;
        const WANDER_BASE_CONSTRUCTOR: u32 = 1;

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        #[inline(always)]
        unsafe fn write_u16(address: u32, value: u16) {
            unsafe { (address as *mut u16).write_unaligned(value) }
        }

        let xmm2_float_bits = lf_checker_rt::xmm_word(2, 0);
        let _ = lf_checker_rt::callee_thiscall!(
            WANDER_BASE_CONSTRUCTOR,
            u32,
            this,
            2,
            xmm2_float_bits,
            1,
            HALF_BITS,
            1
        );
        write_u32(this, lf_checker_rt::relocated(DERIVED_VTABLE));
        write_u32(this.wrapping_add(FIRST_COUNTER), 0);
        write_u32(this.wrapping_add(SECOND_COUNTER), 0);
        write_u16(this.wrapping_add(THIRD_COUNTER), 0);
        write_u32(this.wrapping_add(FOURTH_COUNTER), 0);
        this
    }
});
