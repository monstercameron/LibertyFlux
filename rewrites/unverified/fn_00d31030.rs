// original: 0x00D31030 CTaskComplexClearVehicleSeat::CTaskComplexClearVehicleSeat
/// Constructs a clear-vehicle-seat task by forwarding four selected constructor arguments to the
/// shared vehicle-subtask constructor, then setting this object's virtual table, one task field,
/// and a cleared status word. The original accepts five stack words; the fourth is stored after
/// the base call.
lf_checker_rt::export!(thiscall, rw_00d31030(this: u32, resource: u32, task_code: u32, kind_bits: u32, field_value: u32, extra: u32) -> u32 {
    const VTABLE_VA: u32 = 0x00EE310C;
    unsafe fn write_u32(base: u32, offset: u32, value: u32) {
        unsafe { (base.wrapping_add(offset) as *mut u32).write_unaligned(value) }
    }
    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this, resource, task_code, extra, kind_bits);
        write_u32(this, 0, lf_checker_rt::relocated(VTABLE_VA));
        write_u32(this, 0x34, 0);
        write_u32(this, 0x38, field_value);
        this
    }
});
