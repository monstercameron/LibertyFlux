// original: 0x00967280 detach_timing_node
/// Detach the node keyed `key` from the list at `this + 0x30`.
///
/// Nodes are `(next, key)` pairs. Under the guard (callee 1 locks, callee
/// 2 unlocks) the list is walked for the first node whose key word equals
/// `key`; the match is unlinked (the head slot itself serves as the first
/// link, so head removal needs no special case), pushed onto the global
/// free list at file VA `0x01218474`, and its key cleared. Returns the
/// detached node, or 0 when the list is empty or holds no match. The
/// unlock helper preserves `eax` (it is a bare decrement-and-return), so
/// the contract answers it with `preserve` and the return value is the
/// node (or 0) selected before the call.
///
/// Original: 0x00967280 (thiscall, one stack word).

export!(thiscall, rw_00967280(this: u32, key: u32) -> u32 {
    unsafe {
        const FREE_HEAD: u32 = 0x01218474;
        const LIST: u32 = 0x30;
        callee_cdecl!(1, u32,);
        let mut slot = this.wrapping_add(LIST);
        let mut node = (slot as *const u32).read_unaligned();
        if node == 0 {
            callee_cdecl!(2, u32,);
            return 0;
        }
        loop {
            if (node.wrapping_add(4) as *const u32).read_unaligned() == key {
                let next = (node as *const u32).read_unaligned();
                (slot as *mut u32).write_unaligned(next);
                let free = *global::<u32>(FREE_HEAD);
                (node as *mut u32).write_unaligned(free);
                *global::<u32>(FREE_HEAD) = node;
                (node.wrapping_add(4) as *mut u32).write_unaligned(0);
                callee_cdecl!(2, u32,);
                return node;
            }
            slot = node;
            node = (node as *const u32).read_unaligned();
            if node == 0 {
                callee_cdecl!(2, u32,);
                return 0;
            }
        }
    }
});
