// original: 0x00c7a530 task_scenario_ctor_value

/// Construct a scenario task value object holding two field words.
/// `this` is the object: `field_a` is stored at `+0x14` and `field_b` at
/// `+0x18` after the base object is constructed in place by the base
/// constructor callee. The vtable pointer `VTABLE` is installed, and when
/// `field_b` is non-zero the member constructor callee runs on the slot at
/// `+0x18` with the value in ECX. Returns `this`.
/// Original: 0x00c7a530 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00c7a530(this: u32, field_a: u32, field_b: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00ED5D34;
        const OFF_A: u32 = 0x14;
        const OFF_B: u32 = 0x18;
        const BASE_CTOR: u32 = 1;
        const MEMBER_CTOR: u32 = 2;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        wr32(this.wrapping_add(OFF_A), field_a);
        wr32(this, lf_checker_rt::relocated(VTABLE));
        let slot = this.wrapping_add(OFF_B);
        wr32(slot, field_b);
        if field_b != 0 {
            lf_checker_rt::callee_thiscall!(MEMBER_CTOR, u32, field_b, slot);
        }
        this
    }
});
