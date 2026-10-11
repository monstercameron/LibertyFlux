// original: 0x00A6CEE0 task_object_constructor_with_base_flag

/// Runs the base constructor with flag `1`, installs two vtable entries, and
/// returns the task object.
///
/// `this` is passed in ECX and the base call is intercepted as a thiscall
/// with one stack word. The object fields at `+0` and `+0x14` receive the two
/// relocated vtable addresses. This constructor has no stack arguments of
/// its own.
lf_checker_rt::export!(thiscall, rw_00a6cee0(this: u32) -> u32 {
    unsafe {
        const BASE_CTOR: u32 = 1;
        const BASE_VTABLE: u32 = 0x00E9_EC1C;
        const MEMBER_VTABLE: u32 = 0x00E9_EC70;

        let _ = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this, 1u32);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(BASE_VTABLE));
        ((this + 0x14) as *mut u32).write_unaligned(lf_checker_rt::relocated(MEMBER_VTABLE));
        this
    }
});
