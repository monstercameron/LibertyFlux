// original: 0x00D311C0 CTaskComplexNewGetInVehicle::CTaskComplexNewGetInVehicle
/// Constructs a new-get-in-vehicle task. It forwards the resource, task code, a field value, and
/// a sentinel to the shared vehicle-subtask constructor, then stores its own virtual table, a raw
/// single-precision input, and default state values. The raw float bits are preserved without
/// arithmetic.
lf_checker_rt::export!(thiscall, rw_00d311c0(this: u32, resource: u32, task_code: u32, field_38: u32, kind_bits: u32, speed_bits: u32) -> u32 {
    const VTABLE_VA: u32 = 0x00EE2FD4;
    unsafe fn write_u32(base: u32, offset: u32, value: u32) {
        unsafe { (base.wrapping_add(offset) as *mut u32).write_unaligned(value) }
    }
    unsafe fn write_u8(base: u32, offset: u32, value: u8) {
        unsafe { (base.wrapping_add(offset) as *mut u8).write(value) }
    }
    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this, resource, task_code, kind_bits, u32::MAX);
        write_u32(this, 0x38, field_38);
        write_u32(this, 0, lf_checker_rt::relocated(VTABLE_VA));
        write_u32(this, 0x34, 0);
        write_u32(this, 0x3c, speed_bits);
        write_u32(this, 0x40, 0);
        write_u8(this, 0x44, 0);
        write_u32(this, 0x48, 0);
        write_u32(this, 0x4c, 0x3f400000);
        write_u32(this, 0x50, u32::MAX);
        write_u32(this, 0x54, u32::MAX);
        write_u32(this, 0x58, 0);
        this
    }
});
