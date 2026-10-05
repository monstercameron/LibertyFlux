// original: 0x00870980 crmt_sync_pair_create (proposed)

/// Create the synchronisation pair and link the 255-node free list at `this`.
///
/// Calls the mutex creator (callee 1) with three zero arguments and stores
/// the handle at `this` + 0x100C, then the semaphore creator (callee 2) with
/// (0, 0, 0x7FFF, 0) and stores that handle at `this` + 0x1010. The counter
/// at +0x1014 and the list anchors at +0x1008, +0x1004 and +0x1000 are
/// cleared then the free list is linked: 255 nodes of 16 bytes from `this` +
/// 4, each holding the address of the next, terminated by clearing the last
/// link and pointing the anchor at +0x1008 back at `this`. The loop counter
/// runs 255 down to 1 (exact-zero comparison). Returns `this`.
///
/// Original: 0x00870980 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00870980(this: u32) -> u32 {
    const MUTEX_SLOT: u32 = 0xe73180;
    const SEMAPHORE_SLOT: u32 = 0xe73194;
    const NODE_COUNT: u32 = 255;
    const NODE_STRIDE: u32 = 0x10;
    unsafe {
        let create_mutex = (lf_checker_rt::global::<u32>(MUTEX_SLOT).read_unaligned()) as usize;
        let create_mutex: extern "stdcall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(create_mutex);
        let mutex = create_mutex(0, 0, 0);
        ((this + 0x100c) as *mut u32).write_unaligned(mutex);
        let create_semaphore =
            (lf_checker_rt::global::<u32>(SEMAPHORE_SLOT).read_unaligned()) as usize;
        let create_semaphore: extern "stdcall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(create_semaphore);
        let semaphore = create_semaphore(0, 0, 0x7fff, 0);
        ((this + 0x1010) as *mut u32).write_unaligned(semaphore);
        ((this + 0x1014) as *mut u32).write_unaligned(0);
        ((this + 0x1008) as *mut u32).write_unaligned(0);
        ((this + 0x1004) as *mut u32).write_unaligned(0);
        ((this + 0x1000) as *mut u32).write_unaligned(0);
        let mut node = this + 0x10;
        let mut remaining = NODE_COUNT;
        while remaining != 0 {
            ((node - 0x0c) as *mut u32).write_unaligned(node);
            node += NODE_STRIDE;
            remaining -= 1;
        }
        ((this + 0x0ff4) as *mut u32).write_unaligned(0);
        ((this + 0x1008) as *mut u32).write_unaligned(this);
        this
    }
});
