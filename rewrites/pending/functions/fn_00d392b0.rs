// original: 0x00d392b0 task_state_init_linked
/// Linked task-state initializer: stamp this variant's table, latch the
/// options, clear the payload, then hand the object to the link helper.
export!(thiscall, rw_00d392b0(this: u32, link: u32, opt0: u32, opt1: u32, limit: u32) -> u32 {
    callee_thiscall!(1, u32, this);
    unsafe {
        *(this as *mut u32) = relocated(0xee37bc);
        *((this + 0x10) as *mut u32) = 0;
        *((this + 0x20) as *mut u32) = 0;
        *((this + 0x24) as *mut u32) = 0;
        *((this + 0x28) as *mut u32) = 0;
        *((this + 0x30) as *mut u8) = opt0 as u8;
        *((this + 0x31) as *mut u8) = opt1 as u8;
        *((this + 0x34) as *mut u32) = limit;
        *((this + 0x38) as *mut u32) = 0;
        *((this + 0x3c) as *mut u16) = 0;
        *((this + 0x3e) as *mut u8) = 0;
    }
    callee_thiscall!(2, u32, this, link);
    this
});
