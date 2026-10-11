// original: 0x00D31310 CTaskComplexNewExitVehicle::~CTaskComplexNewExitVehicle
/// Tears down a new-exit-vehicle task. It releases and clears a non-null owned resource, destroys
/// the embedded animation object, installs the base virtual table, and delegates to the common
/// complex-task destructor.
lf_checker_rt::export!(thiscall, rw_00d31310(this: u32) -> u32 {
    const VTABLE_VA: u32 = 0x00EE31DC;
    const RESOURCE_SLOT: u32 = 0x48;
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
        let _ = lf_checker_rt::callee_thiscall!(2, (), this.wrapping_add(0x60));
        lf_checker_rt::callee_thiscall!(3, u32, this)
    }
});
