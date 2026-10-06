// original: 0x00dcf8e0 rb_tree_contains (proposed)

/// Membership test over the red-black tree rooted at `this`: true when a
/// node carries exactly `key`.
///
/// The root pointer is the first word of the object. Each node holds its
/// right child at `+0x8` (with the colour bit in bit 0, masked off on the
/// way down), its left child at `+0xc` and its key at `+0x14`. The walk is a
/// lower bound: a node whose key is at least the search key becomes the
/// candidate and the walk goes left, otherwise it goes right. Afterwards the
/// candidate's key must still be at most the search key, so only an exact
/// match answers true. Both comparisons are UNSIGNED (the original's `jae`
/// and `jb`). An empty tree answers false.
///
/// Original: 0x00DCF8E0 (thiscall, one stack word, byte result).
lf_checker_rt::export!(thiscall, rw_00dcf8e0(this: u32, key: u32) -> u32 {
    unsafe {
        /// Right child pointer with colour bit in bit 0.
        const RIGHT: u32 = 0x8;
        /// Left child pointer.
        const LEFT: u32 = 0xc;
        /// Node key (compared UNSIGNED: jae/jb).
        const KEY: u32 = 0x14;
        let mut node = (this as *const u32).read_unaligned();
        let mut cand: u32 = 0;
        // Lower-bound walk: key(node) >= key goes left, else right.
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
        // Original: (an instruction of the original); jb fail. Unsigned: key < ck fails.
        (key >= ck) as u32
    }
});
