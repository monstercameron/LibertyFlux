// original: 0x00C85C20 physics_shape_child_constructor

/// Run the child-object base initializer, install the child vtable, set the
/// state word at `+4` to `3`, and return the receiver. The routine is thiscall
/// The routine has no stack arguments.
lf_checker_rt::export!(thiscall, rw_00c85c20(this: u32) -> u32 {
    const CHILD_VTABLE: u32 = 0x00ED_6364;
    const BASE_INITIALIZER: u32 = 1;
    const STATE_OFFSET: u32 = 4;
    const INITIAL_STATE: u32 = 3;

    let _ = lf_checker_rt::callee_thiscall!(BASE_INITIALIZER, u32, this);
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(CHILD_VTABLE));
        (this.wrapping_add(STATE_OFFSET) as *mut u32).write_unaligned(INITIAL_STATE);
    }
    this
});
