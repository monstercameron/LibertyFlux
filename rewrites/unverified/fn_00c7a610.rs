// original: 0x00c7a610 ctask_driving_scenario_dtor

/// Destroy a scenario task, releasing the member at `+0x24`.
/// Installs `VTABLE`, then when the member word is non-zero runs the
/// member destructor callee on the slot (value in ECX) and nulls it.
/// Tail-calls the base destructor and returns its result.
/// Original: 0x00c7a610 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00c7a610(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00ED5B34;
        const OFF_MEMBER: u32 = 0x24;
        const MEMBER_DTOR: u32 = 1;
        const BASE_DTOR: u32 = 2;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        wr32(this, lf_checker_rt::relocated(VTABLE));
        let held = rd32(this.wrapping_add(OFF_MEMBER));
        if held != 0 {
            lf_checker_rt::callee_thiscall!(MEMBER_DTOR, u32, held, this.wrapping_add(OFF_MEMBER));
            wr32(this.wrapping_add(OFF_MEMBER), 0);
        }
        lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, this)
    }
});
