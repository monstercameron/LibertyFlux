// original: 0x00967390 prune_foreign_nodes
/// Drop every node of the list at `this + 0x30` not owned by `this`.
///
/// Each node's second word points at a flag word; a node is kept only
/// when that flag word holds `this` itself. Dropped nodes are unlinked
/// and pushed onto the global free list at file VA `0x01218474` with
/// their link word cleared. The guard lock is callee 1; the function
/// ends in a tail jump to the unlock helper (callee 2), whose answer is
/// scripted 0 because `eax` is 0 on every path reaching it. Returns 0.
///
/// Original: 0x00967390 (thiscall, no stack words; tail jump to unlock).

export!(thiscall, rw_00967390(this: u32) -> u32 {
    unsafe {
        const FREE_HEAD: u32 = 0x01218474;
        const LIST: u32 = 0x30;
        callee_cdecl!(1, u32,);
        let mut slot = this.wrapping_add(LIST);
        let mut node = (slot as *const u32).read_unaligned();
        while node != 0 {
            let flag_ptr = (node.wrapping_add(4) as *const u32).read_unaligned();
            if (flag_ptr as *const u32).read_unaligned() == this {
                slot = node;
                node = (node as *const u32).read_unaligned();
            } else {
                let next = (node as *const u32).read_unaligned();
                (slot as *mut u32).write_unaligned(next);
                let free = *global::<u32>(FREE_HEAD);
                (node as *mut u32).write_unaligned(free);
                *global::<u32>(FREE_HEAD) = node;
                (node.wrapping_add(4) as *mut u32).write_unaligned(0);
                node = (slot as *const u32).read_unaligned();
            }
        }
        callee_cdecl!(2, u32,);
        0
    }
});
