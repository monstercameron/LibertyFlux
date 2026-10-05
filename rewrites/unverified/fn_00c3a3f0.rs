// original: 0x00c3a3f0 train_tail_car_via_prev (proposed)
/// Follow the `+0x14d0` link chain to its last node and return it.
///
/// `node` points to a train car; each car's dword at `+0x14d0` points at
/// the previous car or is null. Walks while the link is non-null and
/// returns the final node (the input itself when its link is already
/// null). The chain must be null-terminated. No calls.
///
/// Original: 0x00c3a3f0 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00c3a3f0(node: u32) -> u32 {
    unsafe {
        const PREV: u32 = 0x14d0;
        let mut cur = node;
        let mut next = ((cur + PREV) as *const u32).read_unaligned();
        while next != 0 {
            cur = next;
            next = ((cur + PREV) as *const u32).read_unaligned();
        }
        cur
    }
});
