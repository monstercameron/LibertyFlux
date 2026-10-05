// original: 0x008739A0 crmt_cached_alloc_tag13

/// Allocate-or-reuse a tagged link node: when the word at `obj+0x20` is non-null ask the slot checker (0x874820) for a live node and link it (`[node+8] = obj`, `[node+6] = tag 0xd`); otherwise allocate a 0x128-byte block through the thread manager, initialise it through 0x873650 (which preserves ECX: verified by reading it), stamp vtable 0xfe7fe8 and link it the same way. A null allocation falls through to the link stores and faults, exactly like the original.
///
/// Original: 0x008739A0 (cdecl, one stack word; the caller cleans up).
lf_checker_rt::export!(cdecl, rw_008739a0(obj: u32) -> u32 {
    const CHECK: u32 = 1;
    const ALLOC: u32 = 2;
    const INIT: u32 = 3;
    const TAG: u32 = 0x0d;
    const ALLOC_SIZE: u32 = 0x128;
    const ALLOC_ALIGN: u32 = 0x10;
    const ALLOC_FLAGS: u32 = 0;
    const MANAGER_OFF: u32 = 8;
    const ALLOC_SLOT: u32 = 8;
    const VTABLE: u32 = 0x00fe7fe8;
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
            let _: u32 = lf_checker_rt::callee_thiscall!(INIT, u32, fresh, TAG);
            (fresh as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
            fresh
        } else {
            0
        };
        ((target + 8) as *mut u32).write_unaligned(obj);
        ((target + 6) as *mut u16).write_unaligned(TAG as u16);
        target
    }
});
