// original: 0x00d20800 CTaskComplexSeekCoverShooting::vf22
// Walks three nested subtasks through their vtables, expecting the exact
// type tags in order, and reports whether the cover slot is occupied.
// Returns 1 only when all three tags match and the slot is set, else 0.
export!(thiscall, rw_00d20800(this_ptr: u32) -> u32 {
    unsafe {
        let o1 = *((this_ptr.wrapping_add(8)) as *const u32);
        let v1 = *(o1 as *const u32);
        let t1 = *((v1.wrapping_add(0xC)) as *const u32);
        let tag: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(t1 as usize);
        if tag(o1) != 0x11D {
            return 0;
        }
        let o2 = *((o1.wrapping_add(8)) as *const u32);
        let v2 = *(o2 as *const u32);
        let t2 = *((v2.wrapping_add(0xC)) as *const u32);
        let tag2: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(t2 as usize);
        if tag2(o2) != 0x41A {
            return 0;
        }
        let o3 = *((o2.wrapping_add(8)) as *const u32);
        let v3 = *(o3 as *const u32);
        let t3 = *((v3.wrapping_add(0xC)) as *const u32);
        let tag3: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(t3 as usize);
        if tag3(o3) != 0x76D {
            return 0;
        }
        if *((this_ptr.wrapping_add(0xB0)) as *const u32) != 0 {
            1
        } else {
            0
        }
    }
});
