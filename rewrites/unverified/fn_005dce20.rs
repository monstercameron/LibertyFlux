// original: 0x005dce20 CTaskSimpleAssessInjuredPed::vf1
/// Clone an assess-injured-ped task: allocate, then construct from the
/// source's target handle. Returns the constructor's result or null.
export!(thiscall, rw_005dce20(this_ptr: *mut u8) -> u32 {
    unsafe {
        let pool = *global::<u32>(0x167E2A0);
        let new = callee_thiscall!(1, u32, pool);
        if new == 0 {
            return 0;
        }
        let target = *((this_ptr as *const u8).add(0x1c) as *const u32);
        callee_thiscall!(2, u32, new, target)
    }
});
