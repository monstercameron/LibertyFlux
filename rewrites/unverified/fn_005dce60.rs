// original: 0x005dce60 CTaskComplexTreatAccident::CTaskComplexTreatAccident
/// Clone a treat-accident task: allocate, run the base constructor, copy the
/// patient handle, stamp the vtable, attach to the patient when set.
export!(thiscall, rw_005dce60(this_ptr: *mut u8) -> u32 {
    unsafe {
        let pool = *global::<u32>(0x167E2A0);
        let new = callee_thiscall!(1, u32, pool);
        if new == 0 {
            return 0;
        }
        let patient = *((this_ptr as *const u8).add(0x14) as *const u32);
        callee_thiscall!(2, u32, new);
        *(new as *mut u32) = relocated(0xFE0DD4);
        *((new as *mut u8).add(0x14) as *mut u32) = patient;
        if patient != 0 {
            callee_thiscall!(3, u32, patient, (new as u32).wrapping_add(0x14));
        }
        new
    }
});
