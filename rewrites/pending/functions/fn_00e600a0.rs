// original: 0x00e600a0 link_static_node_1102ec
/// Pushes one static node onto the global singly linked list.
///
/// Reads the head at `0x017ad16c`, stores the old head as the
/// node's next link (`0x011102fc`) and installs the node
/// (`0x011102ec`) as the new head. Returns the old head.
export!(cdecl, rw_00e600a0() -> u32 {
    unsafe {
        let head = global::<u32>(0x017AD16C);
        let old = *head;
        *global::<u32>(0x011102FC) = old;
        *head = relocated(0x011102EC);
        old
    }
});
