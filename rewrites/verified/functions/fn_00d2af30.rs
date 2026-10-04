// original: 0x00d2af30 task_ctor_simple (proposed)
/// Constructor: chain the base constructor with (`arg`, 0, 0, 8.0),
/// stamp vtable slot `+0`, and return `this`.
///
/// Thiscall, one stack word.
lf_checker_rt::export!(thiscall, rw_00d2af30(this: u32, arg: u32) -> u32 {
    unsafe {
        const VT0V: u32 = 0x00ee1f3c;
        const EIGHT: u32 = 0x41000000;
        const BASE: u32 = 1;
        lf_checker_rt::callee_thiscall!(BASE, u32, this, arg, 0, 0, EIGHT);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VT0V));
        this
    }
});
