// original: 0x005e62d0 NativeImpl_CREATE_MOBILE_PHONE
// rw_005e62d0: create the mobile phone object and mark it ready.
//
// Raises the phone-ready flags, records the new phone handle, runs the two
// creation steps, and marks the looked-up phone record active. Returns the
// phone record (the original faults when the lookup fails).
export!(cdecl, rw_005e62d0(arg: u32) -> u32 {
    unsafe {
        let mgr = *global::<u32>(0x18B6EF0);
        *global::<u8>(0x118E909) = 1;
        *((mgr.wrapping_add(0x558)) as *mut u8) |= 1;
        *global::<u32>(0x18B6EE4) = arg;
        *global::<u8>(0x18B6ED7) = 1;
        callee_cdecl!(1, u32,);
        let r: u32 = callee_cdecl!(2, u32, 0);
        let phone = *((r.wrapping_add(0x228)) as *const u32);
        if phone != 0 {
            *((phone.wrapping_add(0x486)) as *mut u8) = 1;
        } else {
            // The original writes through null + 0x416 here and faults.
            *((0u32.wrapping_add(0x416)) as *mut u8) = 1;
        }
        phone
    }
});
