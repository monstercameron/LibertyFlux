// original: 0x00d390d0 owner_scan_3slot
/// Owner scan: probe up to three slots for a live owner in state 2 whose
/// epoch triple agrees, and report whether its latched flag is set.
export!(cdecl, rw_00d390d0(a1: u32, a2: u32) -> u32 {
    unsafe fn word(addr: u32) -> u32 {
        *(addr as *const u32)
    }
    unsafe fn byte(addr: u32) -> u8 {
        *(addr as *const u8)
    }
    let mut slot = 1u32;
    loop {
        let esi = callee_thiscall!(1, u32, a2, slot);
        let mut check = false;
        if esi != 0 && unsafe { byte(esi + 0xa60) } == 2 {
            let c = unsafe { word(esi + 0x224) };
            let s2 = callee_thiscall!(2, u32, c, a1);
            if s2 & 0xff != 0 {
                check = true;
            } else {
                let s3a = callee_thiscall!(3, u32, esi);
                if s3a != 0 {
                    let e = callee_thiscall!(4, u32, esi);
                    let g = callee_thiscall!(5, u32, a1);
                    if e == g {
                        check = true;
                    }
                }
            }
        }
        if check && unsafe { word(esi + 0x260) } & 0x1000_0000 != 0 {
            return 1;
        }
        slot += 1;
        if slot > 3 {
            return 0;
        }
    }
});
