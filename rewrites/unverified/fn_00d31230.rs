// original: 0x00D31230 CTaskComplexOpenVehicleDoor::CTaskComplexOpenVehicleDoor
/// Constructs an open-vehicle-door task by forwarding its four inputs to the shared
/// vehicle-subtask constructor, then setting the derived virtual table and clearing the inherited
/// status and one byte-sized field.
lf_checker_rt::export!(thiscall, rw_00d31230(this: u32, resource: u32, task_code: u32, kind_bits: u32, field_3c: u32) -> u32 {
    const VTABLE_VA: u32 = 0x00EE303C;
    unsafe fn write_u32(base: u32, offset: u32, value: u32) {
        unsafe { (base.wrapping_add(offset) as *mut u32).write_unaligned(value) }
    }
    unsafe fn write_u8(base: u32, offset: u32, value: u8) {
        unsafe { (base.wrapping_add(offset) as *mut u8).write(value) }
    }
    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this, resource, task_code, field_3c, kind_bits);
        write_u32(this, 0, lf_checker_rt::relocated(VTABLE_VA));
        write_u32(this, 0x34, 0);
        write_u8(this, 0x38, 0);
        this
    }
});
