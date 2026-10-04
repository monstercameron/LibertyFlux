// original: 0x00d39150 task_state_init_base
/// Base task-state initializer: stamp the table address and zero the header.
export!(thiscall, rw_00d39150(this: u32) -> u32 {
    unsafe {
        *(this as *mut u32) = relocated(0xee3720);
        *((this + 4) as *mut u32) = 0;
        *((this + 8) as *mut u32) = 0;
        *((this + 0xc) as *mut u32) = 0;
    }
    this
});
