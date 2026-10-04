// original: 0x005dcd10 CTaskSimpleHandsUp::vf1
/// Clone a hands-up task: allocate from the task pool, construct with the
/// source's parameter block, stamp the vtable. Returns the new task or null.
export!(thiscall, rw_005dcd10(this_ptr: *mut u8) -> u32 {
    unsafe {
        let pool = *global::<u32>(0x167E2A0);
        let new = callee_thiscall!(1, u32, pool);
        if new == 0 {
            return 0;
        }
        let param = *((this_ptr as *const u8).add(0x2c) as *const u32);
        callee_thiscall!(2, u32, new, 0, 2, 0x40800000, 0xC0800000, param, 0x19d,
            relocated(0xFE0D70), 0);
        *(new as *mut u32) = relocated(0xEB73CC);
        new
    }
});
