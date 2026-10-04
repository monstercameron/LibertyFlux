// original: 0x00e5ed60 static_node_list_prepend_00e5ed60
/// Prepend one statically allocated node to a global singly-linked list.
///
/// The node at `SLOT` is linked in front of the current list: its link field
/// takes the previous head value, then the head takes the node's address.
/// Returns the previous head value.
export!(cdecl, rw_00e5ed60() -> u32 {
    unsafe {
        let head = global::<u32>(0x017ACD24);
        let old = *head;
        *global::<u32>(0x0110EB0C) = old;
        *head = relocated(0x0110EB08);
        old
    }
});
