// original: 0x00635570 bst_successor (proposed)

/// Next node in key order after `node` in the string-keyed binary search
/// tree (node links `+PARENT`/`+LEFT`/`+RIGHT`; keys are not read).
///
/// When the node has a right child the successor is the leftmost node of
/// that subtree; otherwise it is the nearest ancestor of which the node is
/// in the left subtree, or null when the node is the greatest in the tree
/// (the walk reaches a null parent).
///
/// Returns the successor node or null.
///
/// Original: 0x00635570 (stdcall, one stack word, no calls, no globals).
lf_checker_rt::export!(stdcall, rw_00635570(node: u32) -> u32 {
    unsafe {
        const PARENT: u32 = 0x34;
        const LEFT: u32 = 0x38;
        const RIGHT: u32 = 0x3c;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let right = rd32(node + RIGHT);
        if right != 0 {
            let mut p = right;
            let mut l = rd32(p + LEFT);
            while l != 0 {
                p = l;
                l = rd32(p + LEFT);
            }
            return p;
        }
        let mut cur = node;
        loop {
            let par = rd32(cur + PARENT);
            if par == 0 {
                return 0;
            }
            if rd32(par + LEFT) == cur {
                return par;
            }
            cur = par;
        }
    }
});
