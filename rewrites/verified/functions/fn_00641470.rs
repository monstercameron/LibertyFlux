// original: 0x00641470 rage::ptxModel::vf15

/// Drain a source node list into the front of the model's destination list.
///
/// The model points to its destination-list header at byte offset `0x170`;
/// the source header is the explicit argument. Each header stores a head at
/// `0` and counters at `0x1c` and `0x6c`. Each node stores its next pointer
/// at `0` and a destination-list link at `4`. Nodes are pushed onto the
/// destination head in traversal order. After the move, the source head and
/// count are cleared, along with its per-thread count and first-node slot.
/// The return is zero for a null source pointer, the source pointer for an
/// empty list, or the model's thread-index byte for a non-empty list.
lf_checker_rt::export!(thiscall, rw_00641470(model: u32, source_list: u32) -> u32 {
    const DESTINATION_LIST_POINTER: u32 = 0x170;
    const QUEUE_HEAD: u32 = 0;
    const QUEUE_COUNT: u32 = 0x1c;
    const SECONDARY_COUNT: u32 = 0x6c;
    const NODE_NEXT: u32 = 0;
    const NODE_LINK: u32 = 4;

    if source_list == 0 {
        return 0;
    }

    unsafe {
        let mut node = (source_list.wrapping_add(QUEUE_HEAD) as *const u32).read();
        if node == 0 {
            return source_list;
        }

        let destination_list = (model.wrapping_add(DESTINATION_LIST_POINTER) as *const u32).read();
        while node != 0 {
            let next_node = (node.wrapping_add(NODE_NEXT) as *const u32).read();
            let destination_head = (destination_list.wrapping_add(QUEUE_HEAD) as *const u32).read();
            (node.wrapping_add(NODE_NEXT) as *mut u32).write(destination_head);
            (node.wrapping_add(NODE_LINK) as *mut u32).write(0);
            let destination_count = destination_list.wrapping_add(QUEUE_COUNT) as *mut u32;
            destination_count.write((*destination_count).wrapping_add(1));
            (destination_list.wrapping_add(QUEUE_HEAD) as *mut u32).write(node);
            let secondary_count = destination_list.wrapping_add(SECONDARY_COUNT) as *mut u32;
            secondary_count.write((*secondary_count).wrapping_sub(1));
            node = next_node;
        }

        (source_list.wrapping_add(QUEUE_HEAD) as *mut u32).write(0);
        (source_list.wrapping_add(QUEUE_COUNT) as *mut u32).write(0);
        let thread_index = (destination_list.wrapping_add(0x75) as *const u8).read() as u32;
        let index_bytes = thread_index.wrapping_mul(4);
        (source_list.wrapping_add(0x14).wrapping_add(index_bytes) as *mut u32).write(0);
        (source_list.wrapping_add(4).wrapping_add(index_bytes) as *mut u32).write(0);
        thread_index
    }
});
