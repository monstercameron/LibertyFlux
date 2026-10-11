// original: 0x00D31070 CTaskComplexClimbIntoVehicle::CTaskComplexClimbIntoVehicle
/// Constructs a climb-into-vehicle task by forwarding its four inputs to the shared
/// vehicle-subtask constructor, then installing the derived virtual table and clearing two
/// fields. The method uses thiscall with four stack words and returns the task pointer.
lf_checker_rt::export!(thiscall, rw_00d31070(this: u32, resource: u32, task_code: u32, field_3c: u32, kind_bits: u32) -> u32 {
    const VTABLE_VA: u32 = 0x00EE3174;
    unsafe fn write_u32(base: u32, offset: u32, value: u32) {
        unsafe { (base.wrapping_add(offset) as *mut u32).write_unaligned(value) }
    }
    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this, resource, task_code, kind_bits, field_3c);
        write_u32(this, 0, lf_checker_rt::relocated(VTABLE_VA));
        write_u32(this, 0x34, 0);
        write_u32(this, 0x38, 0);
        this
    }
});
