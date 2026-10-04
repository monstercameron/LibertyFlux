// original: 0x00e600c0 link_static_node_1102d4
/// Pushes one static node onto the global singly linked list.
///
/// Reads the head at `0x017ad16c`, stores the old head as the
/// node's next link (`0x011102e4`) and installs the node
/// (`0x011102d4`) as the new head. Returns the old head.
export!(cdecl, rw_00e600c0() -> u32 {
    unsafe {
        let head = global::<u32>(0x017AD16C);
        let old = *head;
        *global::<u32>(0x011102E4) = old;
        *head = relocated(0x011102D4);
        old
    }
});
