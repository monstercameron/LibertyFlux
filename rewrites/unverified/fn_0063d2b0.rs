// original: 0x0063D2B0 rage::ptxModel::vf7

/// Mark every node reachable from a head-pointer slot with its ready byte.
///
/// `head_slot` points to the first node address. Each node stores the next
/// node address at offset `0` and a one-byte ready flag at offset `0x80`.
/// The traversal stops at a null link. The ECX value is part of the class
/// calling convention but is not read by this method. A normal traversal
/// leaves EAX zero because the final link value is null.
lf_checker_rt::export!(thiscall, rw_0063d2b0(_unused_this: u32, head_slot: u32) -> u32 {
    const NODE_NEXT: u32 = 0;
    const NODE_READY_FLAG: u32 = 0x80;

    unsafe {
        let mut node = (head_slot.wrapping_add(NODE_NEXT) as *const u32).read();
        while node != 0 {
            (node.wrapping_add(NODE_READY_FLAG) as *mut u8).write(1);
            node = (node.wrapping_add(NODE_NEXT) as *const u32).read();
        }
        0
    }
});
