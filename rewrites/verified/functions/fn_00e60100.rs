// original: 0x00e60100 link_static_node_110304
/// Pushes one static node onto the global singly linked list.
///
/// Reads the head at `0x017ad16c`, stores the old head as the
/// node's next link (`0x01110314`) and installs the node
/// (`0x01110304`) as the new head. Returns the old head.
export!(cdecl, rw_00e60100() -> u32 {
    unsafe {
        let head = global::<u32>(0x017AD16C);
        let old = *head;
        *global::<u32>(0x01110314) = old;
        *head = relocated(0x01110304);
        old
    }
});
