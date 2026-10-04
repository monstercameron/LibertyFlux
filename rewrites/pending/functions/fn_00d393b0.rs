// original: 0x00d393b0 task_state_init_payload
/// Payload task-state initializer: store the two header words and the
/// threshold, then stamp this variant's table.
export!(thiscall, rw_00d393b0(this: u32, h0: u32, h1: u32, fbits: u32) -> u32 {
    callee_thiscall!(1, u32, this);
    unsafe {
        *((this + 0x10) as *mut u32) = h0;
        *((this + 0x14) as *mut u32) = h1;
        *(this as *mut u32) = relocated(0xee3858);
        *((this + 0x18) as *mut u32) = fbits;
    }
    this
});
