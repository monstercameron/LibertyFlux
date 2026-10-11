// original: 0x00D31260 CTaskComplexVehicleSubtask::CTaskComplexVehicleSubtask
/// Initializes a vehicle subtask object after delegating common task setup. It installs the task
/// virtual table, stores the resource pointer and task fields, derives one flag from bits 4
/// through 7 of the kind word, and asks the reference helper to retain a non-null resource
/// through the stored slot. The thiscall method receives four stack words and returns the object
/// pointer.
lf_checker_rt::export!(thiscall, rw_00d31260(this: u32, resource: u32, task_code: u32, kind_bits: u32, flags: u32) -> u32 {
    const RESOURCE_SLOT: u32 = 0x18;
    const VTABLE_VA: u32 = 0x00EE2F6C;
    const TASK_CODE: u32 = 0x14;
    const KIND_VALUE: u32 = 0x24;
    const DEFAULT_TASK_PRIORITY: u32 = 3;
    unsafe fn write_u32(base: u32, offset: u32, value: u32) {
        unsafe { (base.wrapping_add(offset) as *mut u32).write_unaligned(value) }
    }
    unsafe fn write_u16(base: u32, offset: u32, value: u16) {
        unsafe { (base.wrapping_add(offset) as *mut u16).write_unaligned(value) }
    }
    unsafe fn write_u8(base: u32, offset: u32, value: u8) {
        unsafe { (base.wrapping_add(offset) as *mut u8).write(value) }
    }
    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
        write_u32(this, TASK_CODE, kind_bits);
        write_u32(this, RESOURCE_SLOT, resource);
        write_u32(this, KIND_VALUE, task_code);
        write_u32(this, 0x20, flags);
        write_u32(this, 0, lf_checker_rt::relocated(VTABLE_VA));
        write_u32(this, 0x2c, DEFAULT_TASK_PRIORITY);
        write_u8(this, 0x1c, 0);
        write_u8(this, 0x30, ((kind_bits >> 4) & 1) as u8);
        write_u16(this, 0x31, 0);
        write_u8(this, 0x33, 0);
        if resource != 0 {
            let slot = this.wrapping_add(RESOURCE_SLOT);
            let _ = lf_checker_rt::callee_thiscall!(2, u32, resource, slot);
        }
        this
    }
});
