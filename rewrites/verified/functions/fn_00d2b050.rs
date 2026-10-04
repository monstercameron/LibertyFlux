// original: 0x00d2b050 task_dtor_tail_a (proposed)
/// Destructor tail: stamp the two vtable slots (`+0`, `+0x14`) then tail-jump
/// to the shared base destructor, forwarding `this` and its result.
///
/// Thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00d2b050(this: u32) -> u32 {
    unsafe {
        const VT0V: u32 = 0x00ee21bc;
        const VT14V: u32 = 0x00ee2214;
        const TAIL: u32 = 1;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VT0V));
        ((this + 0x14) as *mut u32).write_unaligned(lf_checker_rt::relocated(VT14V));
        lf_checker_rt::callee_thiscall!(TAIL, u32, this)
    }
});
