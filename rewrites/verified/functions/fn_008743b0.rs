// original: 0x008743B0 crmt_cached_alloc_tag4

/// Allocate-or-reuse a tagged link node with tag 4: reuse a live node from the slot checker (0x874820) when one is offered, otherwise allocate a 0x3c-byte block through the thread manager, build the node through 0x874270 and link `[node+8] = obj`, `[node+6] = tag`. A null allocation falls through to the link stores and faults, exactly like the original.
///
/// Original: 0x008743B0 (cdecl, one stack word; the caller cleans up).
lf_checker_rt::export!(cdecl, rw_008743b0(obj: u32) -> u32 {
    const CHECK: u32 = 1;
    const ALLOC: u32 = 2;
    const INIT: u32 = 3;
    const TAG: u32 = 0x04;
    const ALLOC_SIZE: u32 = 0x3c;
    const ALLOC_ALIGN: u32 = 0x10;
    const ALLOC_FLAGS: u32 = 0;
    const MANAGER_OFF: u32 = 8;
    const ALLOC_SLOT: u32 = 8;
    unsafe {
        let slot = ((obj + 0x20) as *const u32).read_unaligned();
        if slot != 0 {
            let found: u32 = lf_checker_rt::callee_thiscall!(CHECK, u32, slot, TAG);
            if found != 0 {
                ((found + 8) as *mut u32).write_unaligned(obj);
                ((found + 6) as *mut u16).write_unaligned(TAG as u16);
                return found;
            }
        }
        let tls0 = lf_checker_rt::tls_slot(0);
        let manager = ((tls0 + MANAGER_OFF) as *const u32).read_unaligned();
        let vtable = (manager as *const u32).read_unaligned();
        let target = ((vtable as *const u8).add(ALLOC_SLOT as usize) as *const u32)
            .read_unaligned();
        let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        let fresh = alloc(manager, ALLOC_SIZE, ALLOC_ALIGN, ALLOC_FLAGS);
        let target = if fresh != 0 {
            lf_checker_rt::callee_thiscall!(INIT, u32, fresh)
        } else {
            0
        };
        ((target + 8) as *mut u32).write_unaligned(obj);
        ((target + 6) as *mut u16).write_unaligned(TAG as u16);
        target
    }
});
