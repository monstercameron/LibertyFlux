// original: 0x00878D90 rage::crmtComposerOptimized::vf30

/// Drain the pending list back into the free-node pool.
///
/// `this` points to a composer object. The pool-sync virtual (slot
/// `+0x74`) runs first; then every node of the pending list at `+0x80`
/// runs through the node-release virtual (slot `+0x80`, with the node as
/// its argument) and is pushed onto the free list at `+0x88`. Finally
/// the counters at `+0x90` and `+0x94` are cleared.
///
/// Original: 0x00878D90 (thiscall, no stack arguments). No return value.
lf_checker_rt::export!(thiscall, rw_00878D90(this: u32) -> u32 {
    unsafe {
        const HEAD_OFF: u32 = 0x80;
        const FREE_OFF: u32 = 0x88;
        const COUNT0_OFF: u32 = 0x90;
        const COUNT1_OFF: u32 = 0x94;
        const NEXT_OFF: u32 = 4;
        const SYNC_VT_SLOT: u32 = 0x74;
        const RELEASE_VT_SLOT: u32 = 0x80;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let vtable = rd32(this);
        let sync: extern "thiscall" fn(u32) -> u32 = unsafe {
            core::mem::transmute(rd32(vtable.wrapping_add(SYNC_VT_SLOT)) as usize)
        };
        sync(this);
        let release: extern "thiscall" fn(u32, u32) -> u32 = unsafe {
            core::mem::transmute(rd32(vtable.wrapping_add(RELEASE_VT_SLOT)) as usize)
        };
        let mut node = rd32(this.wrapping_add(HEAD_OFF));
        while node != 0 {
            let next = rd32(node.wrapping_add(NEXT_OFF));
            release(this, node);
            let free = rd32(this.wrapping_add(FREE_OFF));
            wr32(node.wrapping_add(NEXT_OFF), free);
            wr32(this.wrapping_add(FREE_OFF), node);
            node = next;
        }
        wr32(this.wrapping_add(COUNT1_OFF), 0);
        wr32(this.wrapping_add(COUNT0_OFF), 0);
        0
    }
});
