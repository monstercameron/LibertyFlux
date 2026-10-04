// original: 0x005dcf40 CTaskComplexMedicWandering::vf1
/// Clone a medic-wandering task: allocate, then tail-jump to the shared
/// constructor with the fresh slot. Returns the constructor result or null.
export!(thiscall, rw_005dcf40(this_ptr: *mut u8) -> u32 {
    unsafe {
        let _ = this_ptr;
        let pool = *global::<u32>(0x167E2A0);
        let new = callee_thiscall!(1, u32, pool);
        if new == 0 {
            return 0;
        }
        callee_thiscall!(2, u32, new)
    }
});
