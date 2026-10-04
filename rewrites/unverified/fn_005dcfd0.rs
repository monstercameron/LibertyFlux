// original: 0x005dcfd0 CTaskComplexUseWaterCannon::CTaskComplexUseWaterCannon
/// Clone a use-water-cannon task: allocate, run the base constructor, copy
/// the target handle, stamp the vtable. Returns the new task or null.
export!(thiscall, rw_005dcfd0(this_ptr: *mut u8) -> u32 {
    unsafe {
        let pool = *global::<u32>(0x167E2A0);
        let new = callee_thiscall!(1, u32, pool);
        if new == 0 {
            return 0;
        }
        let target = *((this_ptr as *const u8).add(0x14) as *const u32);
        callee_thiscall!(2, u32, new);
        *((new as *mut u8).add(0x14) as *mut u32) = target;
        *(new as *mut u32) = relocated(0xFE0EDC);
        new
    }
});
