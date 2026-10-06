// original: 0x00dcfd90 rb_tree_find (proposed)

/// Find the tree node carrying exactly `key`, or null. The membership
/// callee (thiscall: object, key) is consulted first; a zero byte answer
/// returns null without walking.
///
/// On a nonzero answer the same lower-bound walk as the membership test
/// runs inline over the tree rooted at `this`: right child at `+0x8` with
/// the colour bit masked off, left child at `+0xc`, key at `+0x14`, both
/// compares UNSIGNED. The candidate whose key is at least the search key is
/// returned only when the search key is still at least the candidate's key
/// (exact match); otherwise null.
///
/// Original: 0x00DCFD90 (thiscall, one stack word, node pointer or null).
lf_checker_rt::export!(thiscall, rw_00dcfd90(this: u32, key: u32) -> u32 {
    unsafe {
        /// Membership-test callee id.
        const CONTAINS: u32 = 1;
        const RIGHT: u32 = 0x8;
        const LEFT: u32 = 0xc;
        /// Node key (compared UNSIGNED).
        const KEY: u32 = 0x14;
        let hit: u32 = lf_checker_rt::callee_thiscall!(CONTAINS, u32, this, key);
        if (hit as u8) == 0 {
            return 0;
        }
        let mut node = (this as *const u32).read_unaligned();
        let mut cand: u32 = 0;
        while node != 0 {
            let nk = ((node + KEY) as *const u32).read_unaligned();
            if nk >= key {
                cand = node;
                node = ((node + LEFT) as *const u32).read_unaligned();
            } else {
                node = ((node + RIGHT) as *const u32).read_unaligned() & 0xffff_fffe;
            }
        }
        if cand == 0 {
            return 0;
        }
        let ck = ((cand + KEY) as *const u32).read_unaligned();
        if key < ck {
            return 0;
        }
        cand
    }
});
