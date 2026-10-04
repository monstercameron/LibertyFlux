// original: 0x005dcea0 CTaskComplexMedicTreatInjuredPed::vf1
/// Clone a medic-treat task: allocate, then construct from the source's
/// patient, position and flag fields. Returns the constructor result or null.
export!(thiscall, rw_005dcea0(this_ptr: *mut u8) -> u32 {
    unsafe {
        let pool = *global::<u32>(0x167E2A0);
        let new = callee_thiscall!(1, u32, pool);
        if new == 0 {
            return 0;
        }
        let p14 = *((this_ptr as *const u8).add(0x14) as *const u32);
        let p18 = *((this_ptr as *const u8).add(0x18) as *const u32);
        let f1c = *this_ptr.add(0x1c) as u32;
        let p30 = (this_ptr as u32).wrapping_add(0x30);
        let f42 = *this_ptr.add(0x42) as u32;
        callee_thiscall!(2, u32, new, p14, p18, f1c, p30, f42)
    }
});
