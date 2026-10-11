// original: 0x00D312E0 CTaskComplexCloseVehicleDoor::~CTaskComplexCloseVehicleDoor
/// Tears down a close-vehicle-door task. It installs the base-class virtual table, releases a
/// non-null owned resource through its field, clears that field, then delegates to the common
/// complex-task destructor and returns its result.
lf_checker_rt::export!(thiscall, rw_00d312e0(this: u32) -> u32 {
    const VTABLE_VA: u32 = 0x00EE30A4;
    const RESOURCE_SLOT: u32 = 0x3c;
    unsafe fn read_u32(base: u32, offset: u32) -> u32 {
        unsafe { (base.wrapping_add(offset) as *const u32).read_unaligned() }
    }
    unsafe fn write_u32(base: u32, offset: u32, value: u32) {
        unsafe { (base.wrapping_add(offset) as *mut u32).write_unaligned(value) }
    }
    unsafe {
        let resource = read_u32(this, RESOURCE_SLOT);
        write_u32(this, 0, lf_checker_rt::relocated(VTABLE_VA));
        if resource != 0 {
            let slot = this.wrapping_add(RESOURCE_SLOT);
            let _ = lf_checker_rt::callee_thiscall!(1, u32, resource, slot);
            write_u32(this, RESOURCE_SLOT, 0);
        }
        lf_checker_rt::callee_thiscall!(2, u32, this)
    }
});
