// original: 0x00D310B0 CTaskComplexCloseVehicleDoor::CTaskComplexCloseVehicleDoor
/// Constructs a close-vehicle-door task, forwarding the entity and task code plus two zero
/// defaults to the shared vehicle-subtask constructor. It then stores the requested field,
/// installs the derived virtual table, clears its status, and retains a non-null entity through
/// the object's pointer slot.
lf_checker_rt::export!(thiscall, rw_00d310b0(this: u32, entity: u32, task_code: u32, field_value: u32) -> u32 {
    const VTABLE_VA: u32 = 0x00EE30A4;
    unsafe fn write_u32(base: u32, offset: u32, value: u32) {
        unsafe { (base.wrapping_add(offset) as *mut u32).write_unaligned(value) }
    }
    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this, entity, task_code, 0, 0);
        write_u32(this, 0, lf_checker_rt::relocated(VTABLE_VA));
        write_u32(this, 0x34, field_value);
        write_u32(this, 0x38, 0);
        write_u32(this, 0x3c, entity);
        if entity != 0 {
            let slot = this.wrapping_add(0x3c);
            let _ = lf_checker_rt::callee_thiscall!(2, u32, entity, slot);
        }
        this
    }
});
