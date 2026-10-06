// original: 0x00635b30 list_unlink_a (proposed)

/// Remove `node` from the singly linked list headed by the global `HEAD_A`.
///
/// Links live at `+NEXT` in each node. When the head is null there is
/// nothing to do; when the node is the head, the head takes the node's
/// successor; otherwise the chain is walked from the head and the
/// predecessor (or the head, when the walk ends without finding the node)
/// takes the node's successor. The removed node's own link is cleared.
/// When the node is not in the chain the heap is untouched. (When the node
/// value itself is null the equality test at the end of the chain still
/// matches, exactly as in the original.)
///
/// The original leaves an unspecified value in eax; the rewrite returns 0
/// and the contract compares no return channel.
///
/// Original: 0x00635b30 (thiscall, no stack words, no calls).
lf_checker_rt::export!(thiscall, rw_00635b30(node: u32) -> u32 {
    unsafe {
        const HEAD_A: u32 = 0x01bb67c4;
        const NEXT: u32 = 0x08;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let head = lf_checker_rt::global::<u32>(HEAD_A).read();
        if head == 0 {
            return 0;
        }
        if node == head {
            lf_checker_rt::global::<u32>(HEAD_A).write(rd32(node + NEXT));
            wr32(node + NEXT, 0);
            return 0;
        }
        let mut prev = head;
        let mut cur = rd32(prev + NEXT);
        loop {
            if cur == 0 || cur == node {
                break;
            }
            prev = cur;
            cur = rd32(prev + NEXT);
        }
        if cur != node {
            return 0;
        }
        wr32(prev + NEXT, rd32(node + NEXT));
        wr32(node + NEXT, 0);
        0
    }
});
