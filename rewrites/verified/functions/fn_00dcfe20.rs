// original: 0x00dcfe20 rb_tree_erase_key (proposed)

/// Erase the node carrying `key` and return the match count for it. A zero
/// byte answer from the membership callee (thiscall: object, key) returns 0
/// without touching the tree.
///
/// Otherwise the lower-bound walk runs inline (unsigned key compares, as in
/// the find function). When a node with exactly `key` is found it is passed
/// in ECX to the teardown callee (thiscall, no stack words) and on the
/// stack to the deallocator callee (cdecl, one word). Finally the counter
/// callee (thiscall: object, pointer to the key) runs and its answer is the
/// result, whether or not a node was found.
///
/// Original: 0x00DCFE20 (thiscall, one stack word, integer result).
lf_checker_rt::export!(thiscall, rw_00dcfe20(this: u32, key: u32) -> u32 {
    unsafe {
        /// Membership-test callee id.
        const CONTAINS: u32 = 1;
        /// Node teardown callee id.
        const TEARDOWN: u32 = 2;
        /// Node deallocator callee id.
        const FREE: u32 = 3;
        /// Match-counter callee id.
        const COUNT: u32 = 4;
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
        let mut target: u32 = 0;
        if cand != 0 {
            let ck = ((cand + KEY) as *const u32).read_unaligned();
            if key >= ck {
                target = cand;
            }
        }
        if target != 0 {
            lf_checker_rt::callee_thiscall!(TEARDOWN, u32, target);
            lf_checker_rt::callee_cdecl!(FREE, u32, target);
        }
        // The original passes a pointer to its incoming key slot; the value
        // (the key) is what the callee reads, observed via snapshot.
        let mut key_slot = key;
        lf_checker_rt::callee_thiscall!(COUNT, u32, this, core::ptr::addr_of_mut!(key_slot) as u32)
    }
});
