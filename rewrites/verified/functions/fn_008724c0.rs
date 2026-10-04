// original: 0x008724C0 rage::crmtManager::vf3
//! Refresh the manager's active child: fetch it through slot 0x10 of the
//! manager table (callee 1), guard it with a reference, run the unlink step
//! (callee 2) and the slot-4 handoff, then release the guard. A guard that
//! reaches zero deletes the child through the owner hook (callee 4) or its
//! own table slot 0. Returns 0 when there is no child, the child address
//! when its guard slot is missing, 0xFFFF when the guard survives, else the
//! deleter's answer.
export!(thiscall, rw_008724C0(this: *mut u8) -> u32 {
    unsafe {
        let vt = *(this as *const u32);
        let fetch: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(*((vt.wrapping_add(0x10)) as *const u32) as usize);
        let p = fetch(this as u32);
        if p == 0 {
            return 0;
        }
        let child = *((p.wrapping_add(8)) as *const u32);
        if child == 0 {
            return p;
        }
        let guard = child.wrapping_add(4) as *mut u16;
        *guard = guard.read().wrapping_add(1);
        callee_thiscall!(2, u32, child);
        let vt2 = *(child as *const u32);
        let handoff: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(*((vt2.wrapping_add(4)) as *const u32) as usize);
        handoff(child);
        let left = guard.read().wrapping_add(0xFFFF);
        *guard = left;
        if left != 0 {
            return 0xFFFF;
        }
        let owner = *((child.wrapping_add(0x18)) as *const u32);
        if owner != 0 {
            callee_stdcall!(4, u32, child)
        } else {
            let vt3 = *(child as *const u32);
            let delete: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute((*(vt3 as *const u32)) as usize);
            delete(child, 1)
        }
    }
});
