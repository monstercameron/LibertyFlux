// original: 0x00B80BC0 ped_task_manager_find_node_by_kind_from_list

/// Select one of three list heads at `this + 0x2c + head_index * 4`, walk its
/// linked nodes through `+0x08`, and call each node's kind method until the
/// first exact match for `desired_kind`. Return that node or null; after a
/// match, later nodes are traversed without further method calls.
lf_checker_rt::export!(thiscall, rw_b80bc0(this: u32, head_index: u32, desired_kind: u32) -> u32 {
    unsafe {
        const LISTS: u32 = 0x2c;
        const NODE_NEXT: u32 = 0x08;
        const KIND_METHOD: u32 = 1;

        let head_slot = this.wrapping_add(LISTS).wrapping_add(head_index.wrapping_mul(4));
        let mut node = (head_slot as *const u32).read_unaligned();
        let mut match_node = 0;
        while node != 0 {
            if match_node == 0 {
                let node_kind = lf_checker_rt::callee_thiscall!(KIND_METHOD, u32, node);
                if node_kind == desired_kind {
                    match_node = node;
                }
            }
            node = ((node.wrapping_add(NODE_NEXT)) as *const u32).read_unaligned();
        }
        match_node
    }
});
