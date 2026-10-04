// original: 0x005B4F80 target_attach_preset
/// Presets the mode slots to the attach id, resolves the target object either
/// through the fresh-lookup helper or the id table, initialises it when it
/// accepts initialisation, then applies the change through the mode helpers.
export!(cdecl, rw_005B4F80() -> u32 {
    unsafe {
        *global::<u32>(0x01160C40) = 0x32;
        *global::<u32>(0x01030BA8) = 0xFFFF_FFFF;
        *global::<u32>(0x018E51E4) = 0x32;
        *global::<u8>(0x018E51CD) = 0;
        let fresh = *global::<u8>(0x01160C35) != 0 || *global::<u8>(0x01160C36) != 0;
        let mut target: u32 = 0;
        if fresh {
            target = callee_cdecl!(1, u32,);
        } else {
            let idx = *global::<u32>(0x01036F14);
            if idx != 0xFFFF_FFFF {
                let tab = relocated(0x011A8808) as *const u32;
                target = tab.add(idx as usize).read();
            }
        }
        if target != 0 {
            let r = callee_thiscall!(2, u32, target);
            if r as u8 != 0 {
                callee_thiscall!(3, u32, target);
            }
        }
        let arg = *global::<u32>(0x01160C0C);
        let r = callee_cdecl!(4, u32, arg);
        *global::<u8>(0x018B6E80) = r as u8;
        callee_cdecl!(5, u32, 9)
    }
});
