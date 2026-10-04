// original: 0x00d39360 task_state_init_shared_body
/// Shared-body task-state initializer: forward the payload to the shared
/// body, then stamp this variant's table.
export!(thiscall, rw_00d39360(this: u32, a1: u32, a2: u32, fbits: u32) -> u32 {
    callee_thiscall!(1, u32, this, a1, a2, fbits);
    unsafe {
        *(this as *mut u32) = relocated(0xee38c0);
    }
    this
});
