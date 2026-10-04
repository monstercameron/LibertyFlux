// original: 0x005dd010 CTaskComplexWanderMedic::CTaskComplexWanderMedic
/// Clone a wander-medic task: allocate, then construct from the source's
/// position, flag and the shared default radius. Returns the new task or null.
export!(thiscall, rw_005dd010(this_ptr: *mut u8) -> u32 {
    unsafe {
        let pool = *global::<u32>(0x167E2A0);
        let new = callee_thiscall!(1, u32, pool);
        if new == 0 {
            return 0;
        }
        let radius = *global::<u32>(0xED7EE4);
        let flag = (*this_ptr.add(0x54) & 1) as u32;
        let fx = *((this_ptr as *const u8).add(0x18) as *const u32);
        let fy = *((this_ptr as *const u8).add(0x14) as *const u32);
        callee_thiscall!(2, u32, new, fy, fx, flag, radius, 1);
        *(new as *mut u32) = relocated(0xFE0CB4);
        new
    }
});
