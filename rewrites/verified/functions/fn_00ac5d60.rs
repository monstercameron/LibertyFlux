// original: 0x00AC5D60 stream_keyset_insert (proposed)

/// Insert a timed key into the streaming key set, or find its neighbour.
///
/// The original walks the tree rooted at `this + 4` with the time at
/// `key + 4` (stepping left while the node time is strictly above the key,
/// right otherwise) and then either inserts through the insert callee
/// (passing the address of the incoming key slot for the new node) when the
/// walk ends left of the first node or past a node above the key, or reports
/// the neighbour (thiscall, two stack pointers: `out`, `key`). It writes the
/// node and a 1/0 inserted flag to `out` and returns `out`.
lf_checker_rt::export!(thiscall, rw_00AC5D60(this: u32, out: u32, key: u32) -> u32 {
    unsafe {
        const INSERT: u32 = 1;
        const TOUCH: u32 = 2;
        const ROOT: u32 = 4;
        const FIRST: u32 = 8;
        const LEFT: u32 = 8;
        const RIGHT: u32 = 0x0c;
        const TIME: u32 = 0x14;
        let root = (this.wrapping_add(ROOT) as *const u32).read_unaligned();
        let want = (key.wrapping_add(4) as *const f32).read_unaligned();
        let mut found = this;
        let mut cur = root;
        let mut above = true;
        if cur != 0 {
            loop {
                let t = (cur.wrapping_add(TIME) as *const f32).read_unaligned();
                found = cur;
                if t > want {
                    above = true;
                    cur = (cur.wrapping_add(LEFT) as *const u32).read_unaligned();
                } else {
                    above = false;
                    cur = (cur.wrapping_add(RIGHT) as *const u32).read_unaligned();
                }
                if cur == 0 {
                    break;
                }
            }
        }
        /// Insert through the callee and report the new node. The slot's
        /// content matches the original's incoming key slot (the key
        /// pointer); the node is read back from the key word the callee
        /// filled.
        unsafe fn insert(this: u32, out: u32, key: u32, found: u32, edge: u32) -> u32 {
            unsafe {
                let mut slot: u32 = key;
                lf_checker_rt::callee_thiscall!(
                    1, u32, this, &mut slot as *mut u32 as u32, found, key, edge, 0u32
                );
                let node = (key as *const u32).read_unaligned();
                (out as *mut u32).write_unaligned(node);
                (out.wrapping_add(4) as *mut u8).write(1);
                out
            }
        }
        unsafe {
            if above {
                let first = (this.wrapping_add(FIRST) as *const u32).read_unaligned();
                if found == first {
                    return insert(this, out, key, found, found);
                }
                lf_checker_rt::callee_cdecl!(TOUCH, u32, found);
            }
            let ft = (found.wrapping_add(TIME) as *const f32).read_unaligned();
            if want > ft {
                return insert(this, out, key, found, 0);
            }
            (out as *mut u32).write_unaligned(found);
            (out.wrapping_add(4) as *mut u8).write(0);
            out
        }
    }
});
