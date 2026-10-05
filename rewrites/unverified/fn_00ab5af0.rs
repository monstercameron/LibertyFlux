// original: 0x00ab5af0 idmap_insert_named (proposed)

/// Insert a key into the named id map, cloning the old head's table.
///
/// Same lazy-init and grow protocol as its siblings, then links a fresh
/// 0x18-byte node: `+0` key, `+4`/`+5` the first two bytes behind the value
/// pointer, `+8` a table cloned from four bytes past the value pointer
/// through the clone callee, `+0x14` the old head itself. Note the slot
/// confusion the original embodies: it saves the old head over the incoming
/// key-pointer slot, then reads the tag bytes and the clone source from the
/// value-pointer slot instead (the head slot is only read back for `+0x14`).
/// A null allocation stores null in the bucket and returns null.
///
/// Callees: 1 = lazy init (thiscall, two words), 2 = sizing (cdecl, one
/// word), 3 = grow (thiscall, one word), 4 = allocator (cdecl, one word),
/// 5 = table clone (thiscall, one word).
///
/// Original: 0x00ab5af0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00ab5af0(this: u32, keyptr: u32, valptr: u32) -> u32 {
    unsafe {
        const INIT: u32 = 1;
        const SIZING: u32 = 2;
        const GROW: u32 = 3;
        const ALLOC: u32 = 4;
        const CLONE: u32 = 5;
        const COUNT_OFF: u32 = 4;
        const USED_OFF: u32 = 6;
        const INIT_BUCKETS: u32 = 0x0A;
        const NODE_BYTES: u32 = 0x18;
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
            ((node + 4) as *mut u8).write((valptr as *const u8).read());
            ((node + 5) as *mut u8).write(((valptr + 1) as *const u8).read());
            lf_checker_rt::callee_thiscall!(
                CLONE,
                u32,
                node.wrapping_add(8),
                valptr.wrapping_add(4)
            );
            ((node + 0x14) as *mut u32).write_unaligned(old);
            (cell as *mut u32).write_unaligned(node);
            node
        } else {
            (cell as *mut u32).write_unaligned(0);
            0
        }
    }
});
