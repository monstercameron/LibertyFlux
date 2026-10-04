// original: 0x00e60120 link_static_node_11031c
/// Pushes one static node onto the global singly linked list.
///
/// Reads the head at `0x017ad16c`, stores the old head as the
/// node's next link (`0x0111032c`) and installs the node
/// (`0x0111031c`) as the new head. Returns the old head.
export!(cdecl, rw_00e60120() -> u32 {
    unsafe {
        let head = global::<u32>(0x017AD16C);
        let old = *head;
        *global::<u32>(0x0111032C) = old;
        *head = relocated(0x0111031C);
        old
    }
});
