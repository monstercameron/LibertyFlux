// original: 0x00d2b100 task_dtor_tail_b (proposed)
/// Destructor tail: stamp the two vtable slots (`+0`, `+0x14`) then tail-jump
/// to its base destructor, forwarding `this` and its result.
///
/// Thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00d2b100(this: u32) -> u32 {
    unsafe {
        const VT0V: u32 = 0x00ee2024;
        const VT14V: u32 = 0x00ee2078;
        const TAIL: u32 = 1;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VT0V));
        ((this + 0x14) as *mut u32).write_unaligned(lf_checker_rt::relocated(VT14V));
        lf_checker_rt::callee_thiscall!(TAIL, u32, this)
    }
});
