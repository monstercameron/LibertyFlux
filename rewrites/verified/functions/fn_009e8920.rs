// original: 0x009e8920 list_tail
/// Last node of the `+8`-linked list starting at `head`; null in
/// gives null out. (cdecl, one argument.)
lf_checker_rt::export!(cdecl, rw_009e8920(head: u32) -> u32 {
    unsafe {
        const NEXT_OFF: u32 = 8;
        let mut node = head;
        if node == 0 {
            return 0;
        }
        loop {
            let next = (node.wrapping_add(NEXT_OFF) as *const u32).read_unaligned();
            if next == 0 {
                return node;
            }
            node = next;
        }
    }
});
