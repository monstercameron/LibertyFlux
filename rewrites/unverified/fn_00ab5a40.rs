// original: 0x00ab5a40 idmap_insert_large (proposed)

/// Insert a key into the large-node id map, cloning the old head.
///
/// Same lazy-init and grow protocol as the small-node sibling at
/// 0x00ab59a0, then links a fresh 0x90-byte node: `+0` key, `+4` a 34-word
/// copy of the previous head's body, `+0x8c` the previous head itself. A
/// null allocation stores null in the bucket and returns null.
///
/// Callees: 1 = lazy init (thiscall, two words), 2 = sizing (cdecl, one
/// word), 3 = grow (thiscall, one word), 4 = allocator (cdecl, one word).
///
/// Original: 0x00ab5a40 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00ab5a40(this: u32, keyptr: u32, valptr: u32) -> u32 {
    unsafe {
        const INIT: u32 = 1;
        const SIZING: u32 = 2;
        const GROW: u32 = 3;
        const ALLOC: u32 = 4;
        const COUNT_OFF: u32 = 4;
        const USED_OFF: u32 = 6;
        const INIT_BUCKETS: u32 = 0x0A;
        const NODE_BYTES: u32 = 0x90;
        const COPY_WORDS: u32 = 0x22;
        const HEAD_OFF: u32 = 0x8C;
        let _ = valptr;
        if ((this + COUNT_OFF) as *const u16).read_unaligned() == 0 {
            lf_checker_rt::callee_thiscall!(INIT, u32, this, INIT_BUCKETS, 1);
        }
        let count = ((this + COUNT_OFF) as *const u16).read_unaligned() as u32;
        let used = ((this + USED_OFF) as *const u16).read_unaligned().wrapping_add(1);
        ((this + USED_OFF) as *mut u16).write_unaligned(used);
        if used as u32 == count {
            let fresh = lf_checker_rt::callee_cdecl!(SIZING, u32, count) & 0xFFFF;
            lf_checker_rt::callee_thiscall!(GROW, u32, this, fresh);
        }
        let key = (keyptr as *const u32).read_unaligned();
        let slot = key % count;
        let table = (this as *const u32).read_unaligned();
        let cell = table.wrapping_add(slot.wrapping_mul(4));
        let old = (cell as *const u32).read_unaligned();
        let node = lf_checker_rt::callee_cdecl!(ALLOC, u32, NODE_BYTES);
        if node != 0 {
            (node as *mut u32).write_unaligned(key);
            let mut i = 0u32;
            while i < COPY_WORDS {
                let w = ((old + 4 * i) as *const u32).read_unaligned();
                (((node + 4) + 4 * i) as *mut u32).write_unaligned(w);
                i += 1;
            }
            ((node + HEAD_OFF) as *mut u32).write_unaligned(old);
            (cell as *mut u32).write_unaligned(node);
            node
        } else {
            (cell as *mut u32).write_unaligned(0);
            0
        }
    }
});
