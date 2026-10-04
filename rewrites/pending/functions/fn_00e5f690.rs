// original: 0x00E5F690 prepend_static_node_10f10c
// Prepend the static node at 0x0110F10C to the global list at 0x017ACD24.
//
// Reads the list head, stores it as the node's successor word
// (node + 4) and makes the node the new head. Returns the previous
// head, which is the value the original leaves in EAX.
lf_checker_rt::export!(cdecl, rw_00e5f690() -> u32 {
    unsafe {
        let head = lf_checker_rt::global::<u32>(0x017ACD24);
        let prev = *head;
        *lf_checker_rt::global::<u32>(0x0110F110) = prev;
        *head = lf_checker_rt::relocated(0x0110F10C);
        prev
    }
});
