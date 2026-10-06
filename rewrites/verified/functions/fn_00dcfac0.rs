// original: 0x00dcfac0 rb_tree_count_matches (proposed)

/// Count the chain of matches for the key at `*keyptr`: locate it with the
/// lower-bound walk, then step through successor items while the key still
/// matches. Returns the count.
///
/// The walk over the tree rooted at `this` is the usual one (unsigned key
/// compares; right child colour bit masked off). A miss returns 0 with no
/// calls. On a hit the step callee (thiscall: object, aux frame pointer as
/// arg0, node as arg1) answers a three-word item pointer per call; the
/// first word is the next key, the second the next node (the third is copied
/// to scratch by the original and never observed). Each answer adds one to
/// the count; a null next node or a key mismatch ends the walk.
///
/// Original: 0x00DCFAC0 (thiscall, one stack word, integer result).
lf_checker_rt::export!(thiscall, rw_00dcfac0(this: u32, keyptr: u32) -> u32 {
    unsafe {
        /// Successor-step callee id: (this, aux, node).
        const STEP: u32 = 1;
        const RIGHT: u32 = 0x8;
        const LEFT: u32 = 0xc;
        /// Node key (compared UNSIGNED).
        const KEY: u32 = 0x14;
        let want = (keyptr as *const u32).read_unaligned();
        let mut node = (this as *const u32).read_unaligned();
        let mut cand: u32 = 0;
        while node != 0 {
            let nk = ((node + KEY) as *const u32).read_unaligned();
            if nk >= want {
                cand = node;
                node = ((node + LEFT) as *const u32).read_unaligned();
            } else {
                node = ((node + RIGHT) as *const u32).read_unaligned() & 0xffff_fffe;
            }
        }
        let mut key: u32 = 0;
        let mut cur: u32 = 0;
        if cand != 0 {
            let ck = ((cand + KEY) as *const u32).read_unaligned();
            if want >= ck {
                key = ck;
                cur = cand;
            }
        }
        let mut count: u32 = 0;
        if cur == 0 {
            return 0;
        }
        let mut aux = [0u32; 4];
        loop {
            if want != key {
                break;
            }
            if cur == 0 {
                break;
            }
            // Stack order: the aux pointer is pushed last, so it is arg0.
            let item: u32 = lf_checker_rt::callee_thiscall!(
                STEP,
                u32,
                this,
                aux.as_mut_ptr() as u32,
                cur
            );
            key = (item as *const u32).read_unaligned();
            let next = (((item + 4)) as *const u32).read_unaligned();
            count = count.wrapping_add(1);
            if next == 0 {
                break;
            }
            cur = next;
        }
        count
    }
});
