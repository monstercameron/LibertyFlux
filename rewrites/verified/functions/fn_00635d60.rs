// original: 0x00635d60 list_unlink_c (proposed)

/// Remove `node` from the singly linked list headed by the global `HEAD_C`.
///
/// Identical to `rw_00635b30` except for the head global.
///
/// The original leaves an unspecified value in eax; the rewrite returns 0
/// and the contract compares no return channel.
///
/// Original: 0x00635d60 (thiscall, no stack words, no calls).
lf_checker_rt::export!(thiscall, rw_00635d60(node: u32) -> u32 {
    unsafe {
        const HEAD_C: u32 = 0x01bb67c0;
        const NEXT: u32 = 0x08;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let head = lf_checker_rt::global::<u32>(HEAD_C).read();
        if head == 0 {
            return 0;
        }
        if node == head {
            lf_checker_rt::global::<u32>(HEAD_C).write(rd32(node + NEXT));
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
