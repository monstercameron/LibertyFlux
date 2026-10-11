// original: 0x00D31350 CTaskComplexNewGetInVehicle::~CTaskComplexNewGetInVehicle
/// Tears down a new-get-in-vehicle task. If its entity pointer is non-null and the pointed entity
/// has the selected status, it clears one flag bit. It then delegates to the common complex-task
/// destructor and returns that result.
lf_checker_rt::export!(thiscall, rw_00d31350(this: u32) -> u32 {
    const ENTITY_SLOT: u32 = 0x18;
    const ENTITY_STATUS: u32 = 0x1304;
    const ENTITY_FLAGS: u32 = 0x1310;
    const ENTITY_FLAG_MASK: u8 = 0xf7;
    const VTABLE_VA: u32 = 0x00EE2FD4;
    unsafe fn read_u32(base: u32, offset: u32) -> u32 {
        unsafe { (base.wrapping_add(offset) as *const u32).read_unaligned() }
    }
    unsafe fn read_u8(base: u32, offset: u32) -> u8 {
        unsafe { (base.wrapping_add(offset) as *const u8).read() }
    }
    unsafe fn write_u8(base: u32, offset: u32, value: u8) {
        unsafe { (base.wrapping_add(offset) as *mut u8).write(value) }
    }
    unsafe fn write_u32(base: u32, offset: u32, value: u32) {
        unsafe { (base.wrapping_add(offset) as *mut u32).write_unaligned(value) }
    }
    unsafe {
        let entity = read_u32(this, ENTITY_SLOT);
        write_u32(this, 0, lf_checker_rt::relocated(VTABLE_VA));
        if entity != 0 && read_u32(entity, ENTITY_STATUS) == 1 {
            let flags = read_u8(entity, ENTITY_FLAGS);
            write_u8(entity, ENTITY_FLAGS, flags & ENTITY_FLAG_MASK);
        }
        lf_checker_rt::callee_thiscall!(1, u32, this)
    }
});
