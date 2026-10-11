// original: 0x00B10840 heli_release_physics_resource

/// Release the physics resource stored at `this + 0x78`, if present. The
/// resource destructor runs first, followed by the manager's release method
/// with the manager in ECX and the resource as its stack argument. The slot
/// is cleared even when it was already null. This is a thiscall method.
lf_checker_rt::export!(thiscall, rw_00b10840(this: u32) -> () {
    const RESOURCE_SLOT: u32 = 0x78;
    const MANAGER_VA: u32 = 0x0166_9D70;

    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let resource = read_u32(this.wrapping_add(RESOURCE_SLOT));
        if resource != 0 {
            let _ = lf_checker_rt::callee_thiscall!(1, u32, resource);
            let manager = read_u32(lf_checker_rt::global::<u32>(MANAGER_VA) as u32);
            let _ = lf_checker_rt::callee_thiscall!(2, u32, manager, resource);
        }
        write_u32(this.wrapping_add(RESOURCE_SLOT), 0);
    }
});
