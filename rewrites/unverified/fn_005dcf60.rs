// original: 0x005dcf60 CTaskComplexDriveFireTruck::CTaskComplexDriveFireTruck
/// Clone a drive-fire-truck task: allocate, run the base constructor, copy
/// the vehicle handles and flag byte, attach to each handle when set.
export!(thiscall, rw_005dcf60(this_ptr: *mut u8) -> u32 {
    unsafe {
        let pool = *global::<u32>(0x167E2A0);
        let new = callee_thiscall!(1, u32, pool);
        if new == 0 {
            return 0;
        }
        let flag = *this_ptr.add(0x18);
        let h1c = *((this_ptr as *const u8).add(0x1c) as *const u32);
        let h14 = *((this_ptr as *const u8).add(0x14) as *const u32);
        callee_thiscall!(2, u32, new);
        *(new as *mut u32) = relocated(0xFE0E84);
        *((new as *mut u8).add(0x14) as *mut u32) = h14;
        *((new as *mut u8).add(0x1c) as *mut u32) = h1c;
        *((new as *mut u8).add(0x20) as *mut u32) = 0;
        *(new as *mut u8).add(0x18) = flag;
        if h14 != 0 {
            callee_thiscall!(3, u32, h14, (new as u32).wrapping_add(0x14));
        }
        if *((new as *const u8).add(0x1c) as *const u32) != 0 {
            callee_thiscall!(3, u32, *((new as *const u8).add(0x1c) as *const u32),
                (new as u32).wrapping_add(0x1c));
        }
        new
    }
});
