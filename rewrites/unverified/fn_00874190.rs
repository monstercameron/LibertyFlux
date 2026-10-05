// original: 0x00874190 crmt_pair_link_init

/// Link two freshly allocated blocks: stash `a0`/`a1`/`a2` at +0x14/+0x18/+0x1c; allocate a 0x28-byte node, stamp vtable 0xfe8584 with a one-count and a null first slot, into `[this+0x10]` (null when allocation failed); allocate a 0xa0-byte record, initialise it through 0x877760 (which returns the block, so the stub preserves entry eax), into `[this+0xc]`; then wire the record at +0x18/+0x1c/+0x20/+0x14 from the stashed words and the node (the +0x18/+0x20 links only when their word is nonzero). Returns the node.
///
/// Original: 0x00874190 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00874190(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    const MANAGER_OFF: u32 = 8;
    const FREE_SLOT: u32 = 0x0c;
    const ALLOC_SLOT: u32 = 8;
    const INIT: u32 = 2;
    unsafe {
        ((this + 0x14) as *mut u32).write_unaligned(a0);
        ((this + 0x18) as *mut u32).write_unaligned(a1);
        ((this + 0x1c) as *mut u32).write_unaligned(a2);
        let tls0 = lf_checker_rt::tls_slot(0);
        let manager = ((tls0 + MANAGER_OFF) as *const u32).read_unaligned();
        let vtable = (manager as *const u32).read_unaligned();
        let target = ((vtable as *const u8).add(ALLOC_SLOT as usize) as *const u32)
            .read_unaligned();
        let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        let b1 = alloc(manager, 0x28, 0x10, 0);
        let blk1 = if b1 != 0 {
            (b1 as *mut u32).write_unaligned(lf_checker_rt::relocated(0x00fe8584));
            ((b1 + 0x24) as *mut u32).write_unaligned(0);
            let c = ((b1 + 0x24) as *const u32).read_unaligned();
            ((b1 + 0x24) as *mut u32).write_unaligned(c.wrapping_add(1));
            ((b1 + c * 4 + 4) as *mut u32).write_unaligned(0);
            b1
        } else { 0 };
        ((this + 0x10) as *mut u32).write_unaligned(blk1);
        let b2 = alloc(manager, 0xa0, 0x10, 0);
        let blk2 = if b2 != 0 {
            lf_checker_rt::callee_thiscall!(INIT, u32, b2)
        } else { 0 };
        ((this + 0xc) as *mut u32).write_unaligned(blk2);
        let c0 = ((this + 0x14) as *const u32).read_unaligned();
        if c0 != 0 {
            ((blk2 + 0x18) as *mut u32).write_unaligned(c0);
        }
        let c1 = ((this + 0xc) as *const u32).read_unaligned();
        let c2 = ((this + 0x18) as *const u32).read_unaligned();
        ((c1 + 0x1c) as *mut u32).write_unaligned(c2);
        let c3 = ((this + 0x1c) as *const u32).read_unaligned();
        if c3 != 0 {
            let c4 = ((this + 0xc) as *const u32).read_unaligned();
            ((c4 + 0x20) as *mut u32).write_unaligned(c3);
        }
        let c5 = ((this + 0xc) as *const u32).read_unaligned();
        let c6 = ((this + 0x10) as *const u32).read_unaligned();
        ((c5 + 0x14) as *mut u32).write_unaligned(c6);
        blk1
    }
});
