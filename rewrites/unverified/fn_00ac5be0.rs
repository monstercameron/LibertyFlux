// original: 0x00AC5BE0 stream_keyset_link_node (proposed)

/// Allocate a key node and link it into the streaming key set.
///
/// The original allocates a node through the allocator callee and fills it
/// from the key words: at the root (`node == this`) it copies the pair's key
/// and becomes root, first and last; deeper it copies the key and links as
/// the left child when the flag word is zero and the node time is strictly
/// above the key time, else as the right child (thiscall, five stack words:
/// an unread word, `node`, the in/out pair, `key`, `flag`). It then attaches
/// the node through the attach callee, bumps the count at `this + 0x10`,
/// writes the node into the pair and returns the pair.
lf_checker_rt::export!(thiscall, rw_00AC5BE0(
    this: u32, _u: u32, node: u32, pair: u32, key: u32, flag: u32,
) -> u32 {
    unsafe {
        const ALLOC: u32 = 1;
        const ATTACH: u32 = 2;
        const NODE_SIZE: u32 = 0x18;
        const KEY_OFF: u32 = 0x10;
        const TIME_OFF: u32 = 0x14;
        const COUNT: u32 = 0x10;
        /// Fresh node with key words `k0`/`k1` and clear links.
        unsafe fn fresh(k0: u32, k1: u32) -> u32 {
            unsafe {
                let p = lf_checker_rt::callee_cdecl!(ALLOC, u32, NODE_SIZE);
                (p.wrapping_add(KEY_OFF) as *mut u32).write_unaligned(k0);
                (p.wrapping_add(KEY_OFF + 4) as *mut u32).write_unaligned(k1);
                (p.wrapping_add(8) as *mut u32).write_unaligned(0);
                (p.wrapping_add(0x0c) as *mut u32).write_unaligned(0);
                p
            }
        }
        unsafe {
            let fresh_node: u32;
            if node == this {
                let k0 = (pair as *const u32).read_unaligned();
                let k1 = (pair.wrapping_add(4) as *const u32).read_unaligned();
                fresh_node = fresh(k0, k1);
                (node.wrapping_add(8) as *mut u32).write_unaligned(fresh_node);
                (this.wrapping_add(4) as *mut u32).write_unaligned(fresh_node);
                (this.wrapping_add(0x0c) as *mut u32).write_unaligned(fresh_node);
            } else {
                let k0 = (key as *const u32).read_unaligned();
                let k1 = (key.wrapping_add(4) as *const u32).read_unaligned();
                fresh_node = fresh(k0, k1);
                let nt = (node.wrapping_add(TIME_OFF) as *const f32).read_unaligned();
                let kt = (key.wrapping_add(4) as *const f32).read_unaligned();
                if flag == 0 && nt > kt {
                    (node.wrapping_add(8) as *mut u32).write_unaligned(fresh_node);
                    let first = (this.wrapping_add(8) as *const u32).read_unaligned();
                    if node == first {
                        (this.wrapping_add(8) as *mut u32).write_unaligned(fresh_node);
                    }
                } else {
                    (node.wrapping_add(0x0c) as *mut u32).write_unaligned(fresh_node);
                    let last = (this.wrapping_add(0x0c) as *const u32).read_unaligned();
                    if node == last {
                        (this.wrapping_add(0x0c) as *mut u32).write_unaligned(fresh_node);
                    }
                }
            }
            (fresh_node.wrapping_add(4) as *mut u32).write_unaligned(node);
            lf_checker_rt::callee_cdecl!(ATTACH, u32, fresh_node, this.wrapping_add(4));
            let c = this.wrapping_add(COUNT) as *mut u32;
            *c = c.read_unaligned().wrapping_add(1);
            (pair as *mut u32).write_unaligned(fresh_node);
            pair
        }
    }
});
