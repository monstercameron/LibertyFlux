// original: 0x00B10610 heli_assign_physics_resource

/// Look up the physics manager from a relocated global and, when it is
/// available, ask it to create a resource from this object and the method's
/// stack argument. Bit 0 of the flag byte is set on both paths, and the
/// resulting pointer is stored at offset 0x78 and returned. A missing manager
/// stores and returns null. This is a thiscall method with one stack argument.
lf_checker_rt::export!(thiscall, rw_00b10610(this: u32, argument: u32) -> u32 {
    const FLAGS: u32 = 0xF4;
    const RESOURCE: u32 = 0x78;
    const MANAGER_VA: u32 = 0x0166_9D70;

    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    unsafe fn write_u8(address: u32, value: u8) {
        unsafe { (address as *mut u8).write(value) }
    }
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let manager = read_u32(lf_checker_rt::global::<u32>(MANAGER_VA) as u32);
        let physics_context = lf_checker_rt::callee_thiscall!(1, u32, manager);
        let resource = if physics_context == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(2, u32, physics_context, this, argument)
        };
        let flag_address = this.wrapping_add(FLAGS);
        write_u8(flag_address, read_u8(flag_address) | 1);
        write_u32(this.wrapping_add(RESOURCE), resource);
        resource
    }
});
