// original: 0x00d393e0 task_state_init_default
/// Default task-state initializer: stamp this variant's table and the
/// all-set header with the default threshold.
export!(thiscall, rw_00d393e0(this: u32) -> u32 {
    callee_thiscall!(1, u32, this);
    unsafe {
        *(this as *mut u32) = relocated(0xee3858);
        *((this + 0x10) as *mut u32) = 0xffff_ffff;
        *((this + 0x14) as *mut u32) = 0xffff_ffff;
        *((this + 0x18) as *mut u32) = 0x4180_0000;
    }
    this
});
