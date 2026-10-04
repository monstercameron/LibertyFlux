// original: 0x00d38d40 CTaskComplexNewExitVehicle::vf5
/// Exit-vehicle task slot 5: re-validate the target through the task chain and
/// report whether the exit sequence may proceed. Three virtual helpers are
/// reached through object tables exactly like the original (the checker plants
/// scripted answers there).
export!(thiscall, rw_00d38d40(this: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe fn word(addr: u32) -> u32 {
        *(addr as *const u32)
    }
    unsafe fn byte(addr: u32) -> u8 {
        *(addr as *const u8)
    }
    if a2 != 2 {
        let probe = if a3 == 0 {
            0
        } else {
            let vt = unsafe { word(a3) };
            let slot = unsafe { word(vt + 0x18) };
            let f: extern "thiscall" fn(u32) -> u32 =
                unsafe { core::mem::transmute(slot as usize) };
            f(a3) & 0xff
        };
        if probe == 0 && unsafe { word(this + 0x34) } == 0x18 {
            return 0;
        }
    }
    let inner = unsafe { word(this + 8) };
    if unsafe { byte(inner + 0xc) } & 1 == 0 {
        let vt = unsafe { word(inner) };
        let slot = unsafe { word(vt + 0x14) };
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            unsafe { core::mem::transmute(slot as usize) };
        if f(inner, a1, a2, a3) & 0xff == 0 {
            return 0;
        }
        unsafe { *((inner + 0xc) as *mut u32) |= 2 };
    }
    if unsafe { byte(a1 + 0x26c) } & 4 != 0 {
        return 1;
    }
    if unsafe { byte(a1 + 0x1e2) } & 0x0f == 0 {
        return 1;
    }
    let vt = unsafe { word(a1) };
    let slot = unsafe { word(vt + 0x114) };
    let f: extern "thiscall" fn(u32, u32) -> u32 =
        unsafe { core::mem::transmute(slot as usize) };
    f(a1, 1);
    1
});
