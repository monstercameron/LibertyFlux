// original: 0x005dcee0 CTaskComplexMedicDriver::vf1
/// Clone a medic-driver task: allocate, then construct from the source's
/// vehicle, route and flag fields. Returns the constructor result or null.
export!(thiscall, rw_005dcee0(this_ptr: *mut u8) -> u32 {
    unsafe {
        let pool = *global::<u32>(0x167E2A0);
        let new = callee_thiscall!(1, u32, pool);
        if new == 0 {
            return 0;
        }
        let p14 = *((this_ptr as *const u8).add(0x14) as *const u32);
        let p18 = *((this_ptr as *const u8).add(0x18) as *const u32);
        let p20 = (this_ptr as u32).wrapping_add(0x20);
        let f32 = *this_ptr.add(0x32) as u32;
        callee_thiscall!(2, u32, new, p14, p18, p20, f32)
    }
});
