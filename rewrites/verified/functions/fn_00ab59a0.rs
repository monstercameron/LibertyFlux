// original: 0x00ab59a0 idmap_insert_small (proposed)

/// Insert a key into the small-node id map, chaining the 12-byte node.
///
/// Lazily initialises the map through the init callee when the 16-bit
/// bucket count at `+4` is zero, grows it through the sizing and grow
/// callees when the used count at `+6` reaches the bucket count, then
/// links a fresh 12-byte node (`+0` key, `+4` value word from `*valptr`,
/// `+8` previous head) at bucket `key % count`. A null allocation stores
/// null in the bucket and returns null.
///
/// Callees: 1 = lazy init (thiscall, two words), 2 = sizing (cdecl, one
/// word), 3 = grow (thiscall, one word), 4 = allocator (cdecl, one word).
///
/// Original: 0x00ab59a0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00ab59a0(this: u32, keyptr: u32, valptr: u32) -> u32 {
    unsafe {
        const INIT: u32 = 1;
        const SIZING: u32 = 2;
        const GROW: u32 = 3;
        const ALLOC: u32 = 4;
        const TABLE_OFF: u32 = 0;
        const COUNT_OFF: u32 = 4;
        const USED_OFF: u32 = 6;
        const INIT_BUCKETS: u32 = 0x0A;
        const NODE_BYTES: u32 = 0x0C;
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
        let table = ((this + TABLE_OFF) as *const u32).read_unaligned();
        let cell = table.wrapping_add(slot.wrapping_mul(4));
        let old = (cell as *const u32).read_unaligned();
        let node = lf_checker_rt::callee_cdecl!(ALLOC, u32, NODE_BYTES);
        if node != 0 {
            (node as *mut u32).write_unaligned(key);
            let val = (valptr as *const u32).read_unaligned();
            ((node + 4) as *mut u32).write_unaligned(val);
            ((node + 8) as *mut u32).write_unaligned(old);
            (cell as *mut u32).write_unaligned(node);
            node
        } else {
            (cell as *mut u32).write_unaligned(0);
            0
        }
    }
});
