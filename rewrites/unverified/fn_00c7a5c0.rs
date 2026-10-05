// original: 0x00c7a5c0 task_scenario_ctor_default

/// Construct a scenario task with defaulted trailing arguments.
/// Forwards (`a0`, 0, 0) to the base constructor callee, installs `VTABLE`
/// and returns `this`.
/// Original: 0x00c7a5c0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00c7a5c0(this: u32, a0: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00ED5A64;
        const BASE_CTOR: u32 = 1;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this, a0, 0, 0);
        wr32(this, lf_checker_rt::relocated(VTABLE));
        this
    }
});
