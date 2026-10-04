// original: 0x005B4ED0 mode_select_and_apply
/// Selects the active mode id from the mode switch and its option bytes,
/// stores it to the mode slots, invalidates the cached selection, and applies
/// the change through the mode helpers. Optionally refreshes the subsystem
/// first when asked.
export!(thiscall, rw_005B4ED0(flag: u32) -> u32 {
    unsafe {
        if flag as u8 != 0 {
            callee_thiscall!(1, u32, flag);
        }
        let mode = *global::<u32>(0x011D6FD4);
        let opts = global::<u8>(0x01160D0C) as *const u8;
        let al = match mode {
            1 => opts.add(1).read(),
            2 => opts.add(2).read(),
            3 => opts.add(3).read(),
            _ => opts.add(0).read(),
        };
        let mut ecx = al as u32;
        if mode == 0 {
            let bits = *global::<u32>(0x018B6EA4);
            if (bits >> 1) & 1 != 0 {
                ecx = 0x1B;
            } else if bits & 1 != 0 {
                ecx = if al == 0 { 0x1A } else { 0x19 };
            } else {
                ecx = if al == 0 { 0x18 } else { 0x17 };
            }
        } else {
            ecx = if al == 0 { 0x18 } else { 0x17 };
        }
        let arg = *global::<u32>(0x01160C0C);
        *global::<u32>(0x01160C40) = ecx;
        *global::<u32>(0x01030BA8) = 0xFFFF_FFFF;
        *global::<u32>(0x018E51E4) = ecx;
        *global::<u8>(0x018E51CD) = 0;
        let r = callee_cdecl!(2, u32, arg);
        *global::<u8>(0x018B6E80) = r as u8;
        callee_cdecl!(3, u32, 9)
    }
});
