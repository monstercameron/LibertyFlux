// original: 0x00CC5F20 list_advance_n (proposed)

/// Walk a singly linked list forward by `steps` links.
///
/// `this` points to a list head whose first word is the first node; each node
/// links to the next through the word at `+4`. Returns the node reached after
/// `steps` links, or the head's first node when `steps` is zero (which still
/// reads the head). No validity checks: a null or short chain faults on the
/// dereference, exactly as the original.
///
/// Original: 0x00CC5F20 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00cc5f20(this: u32, steps: u32) -> u32 {
    unsafe {
        const NEXT: u32 = 4;
        #[inline(always)]
        unsafe fn rd(node: u32) -> u32 {
            unsafe { (node as *const u32).read_unaligned() }
        }
        let mut node = rd(this);
        let mut left = steps;
        if left != 0 {
            loop {
                node = rd(node.wrapping_add(NEXT));
                left -= 1;
                if left == 0 {
                    break;
                }
            }
        }
        node
    }
});
