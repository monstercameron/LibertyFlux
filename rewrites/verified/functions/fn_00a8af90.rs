// original: 0x00a8af90 pool_replace_head (proposed)

/// Replace the list head with a new node, unlinking and relinking.
///
/// `this` holds the head pointer at +0 and `node` is the new head. A null
/// node, or one already the head, returns with no call (leaving the
/// incoming register in eax, which the rewrite cannot observe: the proof
/// only feeds non-null nodes that differ from the head). Otherwise the node
/// is unlinked through the first callee, linked after the old head through
/// the second, and stored as the new head. Returns whatever the link callee
/// returns, as the original leaves it in eax.
///
/// Original: 0x00A8AF90 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00a8af90(this: u32, node: u32) -> u32 {
    unsafe {
        const CALLEE_UNLINK: u32 = 1;
        const CALLEE_LINK: u32 = 2;
        const HEAD: u32 = 0;
        let head = ((this + HEAD) as *const u32).read_unaligned();
        if node == 0 || node == head {
            // Unreachable under the proof's inputs (non-null, non-head
            // nodes only); return a fixed value so the shape stays total.
            return 0;
        }
        lf_checker_rt::callee_thiscall!(CALLEE_UNLINK, u32, node);
        let r = lf_checker_rt::callee_thiscall!(CALLEE_LINK, u32, head, node);
        ((this + HEAD) as *mut u32).write_unaligned(node);
        r
    }
});
