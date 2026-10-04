// original: 0x00d39170 task_state_init_timed
/// Timed task-state initializer: forward the payload to the shared body,
/// then stamp this variant's table and timeout slot.
export!(thiscall, rw_00d39170(this: u32, a1: u32, a2: u32, fbits: u32, a4: u32) -> u32 {
    // NOTE: the original takes 4 stack args (a1, a2, float, a4); the float
    // travels by value on the stack, so the signature models it as bits.
    callee_thiscall!(1, u32, this, a1, a2, fbits);
    unsafe {
        *((this + 0x1c) as *mut u32) = a4;
        *(this as *mut u32) = relocated(0xee388c);
        *((this + 0x20) as *mut u8) = 0;
    }
    this
});
