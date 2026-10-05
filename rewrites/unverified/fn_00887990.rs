// original: 0x00887990 stream_hash_insert (proposed)

/// Insert a key into the stream hash table, growing it when full.
///
/// `this` points to the table (`[this]` buckets, word count at `+4`, word
/// load at `+6`). When the load reaches the count, a growth tick runs
/// (callees 2 and 3); the empty-table entry (callee 1) only runs when the
/// count is 0. The key word at `keyp` selects bucket `key % count`; a fresh
/// 12-byte node (callee 4) takes the key, the value `[valp]` and the old
/// head, and becomes the new head. A failed allocation stores a null head.
/// The answer is the new head.
///
/// The empty-table path divides by zero, so the contract keeps the count
/// non-zero and exempts callee 1 (it can only fire there).
///
/// Original: 0x00887990 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00887990(this: u32, keyp: u32, valp: u32) -> u32 {
    unsafe {
        const BUCKETS: u32 = 0x00;
        const COUNT: u32 = 0x04;
        const LOAD: u32 = 0x06;
        const NODE_KEY: u32 = 0x00;
        const NODE_VAL: u32 = 0x04;
        const NODE_NEXT: u32 = 0x08;
        const NODE_SIZE: u32 = 12;
        const INIT: u32 = 1;
        const GROW_TICK: u32 = 2;
        const GROW: u32 = 3;
        const ALLOC: u32 = 4;
        let count = ((this + COUNT) as *const u16).read_unaligned() as u32;
        if count == 0 {
            lf_checker_rt::callee_stdcall!(INIT, u32, 0x0a, 1);
        }
        let load = ((this + LOAD) as *const u16).read_unaligned();
        ((this + LOAD) as *mut u16).write_unaligned(load.wrapping_add(1));
        if load.wrapping_add(1) == count as u16 {
            let t = lf_checker_rt::callee_cdecl!(GROW_TICK, u32, count);
            let g = lf_checker_rt::callee_thiscall!(GROW, u32, this, t & 0xffff);
            let _ = g;
        }
        let key = ((keyp) as *const u16).read_unaligned() as u32;
        let b = key % count;
        let table = ((this + BUCKETS) as *const u32).read_unaligned();
        let slot = table.wrapping_add(b.wrapping_mul(4));
        let head = (slot as *const u32).read_unaligned();
        let node = lf_checker_rt::callee_cdecl!(ALLOC, u32, NODE_SIZE);
        if node == 0 {
            (slot as *mut u32).write_unaligned(0);
            0
        } else {
            let v = ((valp) as *const u32).read_unaligned();
            ((node + NODE_KEY) as *mut u16).write_unaligned(key as u16);
            ((node + NODE_VAL) as *mut u32).write_unaligned(v);
            ((node + NODE_NEXT) as *mut u32).write_unaligned(head);
            (slot as *mut u32).write_unaligned(node);
            node
        }
    }
});
