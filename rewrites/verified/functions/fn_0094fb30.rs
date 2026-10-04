// original: 0x0094fb30 list_insert_after
/// Splice a node into a doubly-linked list after a given position.
///
/// Links `node` between `pos` and its current successor (fixing the
/// successor's back-link unless there is none) and returns the old
/// successor, or null when the node became the last one.
export!(thiscall, rw_0094fb30(pos: *mut u32, node: u32) -> u32 {
    unsafe {
        let next = *pos.add(2);
        let n = node as *mut u32;
        *n.add(2) = next;
        *n.add(3) = pos as u32;
        if next != 0 {
            *((next + 0xC) as *mut u32) = node;
        }
        *pos.add(2) = node;
        next
    }
});
