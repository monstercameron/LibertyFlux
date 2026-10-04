// original: 0x00d38dd0 CTaskComplexNewGetInVehicle::vf5
/// Enter-vehicle task slot 5: accept an already-seated occupant outright,
/// otherwise re-validate through the task chain and clear the cruise flag on
/// the inner state when the chain accepts. Virtual helpers are reached through
/// object tables exactly like the original.
export!(thiscall, rw_00d38dd0(this: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe fn word(addr: u32) -> u32 {
        *(addr as *const u32)
    }
    unsafe fn byte(addr: u32) -> u8 {
        *(addr as *const u8)
    }
    if a3 != 0 {
        let vt = unsafe { word(a3) };
        let slot = unsafe { word(vt + 4) };
        let probe: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(slot as usize) };
        if probe(a3) == 0x36 && unsafe { word(a3 + 0xc) } == unsafe { word(this + 0x18) } {
            let n = unsafe { word(a3 + 4) };
            unsafe { *((a3 + 4) as *mut u32) = n.wrapping_add(1) };
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
    if unsafe { byte(a1 + 0x1e2) } & 0x0f != 0 {
        let vt = unsafe { word(a1) };
        let slot = unsafe { word(vt + 0x114) };
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            unsafe { core::mem::transmute(slot as usize) };
        f(a1, 1);
    }
    let t = unsafe { word(this + 0x18) };
    if t != 0 && unsafe { word(t + 0x1304) } == 1 {
        unsafe { *((t + 0x1310) as *mut u8) &= 0xf7 };
    }
    1
});
