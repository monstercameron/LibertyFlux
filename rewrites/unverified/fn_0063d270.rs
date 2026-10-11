// original: 0x0063D270 rage::ptxModel::vf24

/// Move the source queue's first node to the front of a destination queue.
///
/// `owner` contains a pointer to the source queue at byte offset `0x170`.
/// Each queue stores its head pointer at offset `0` and a node count at
/// offset `0x1c`; the source also stores a second count at `0x6c`. A node's
/// next pointer is at offset `0`, and the destination-list link is at offset
/// `4`. If the source is empty, return zero without changing either queue.
/// Otherwise detach the first node, decrement the source count, increment
/// its secondary count, insert the node before the destination head, clear
/// the node's destination-list link, increment the destination count, and
/// return the moved node. All counter arithmetic wraps as a 32-bit word.
lf_checker_rt::export!(thiscall, rw_0063d270(owner: u32, destination: u32) -> u32 {
    const SOURCE_QUEUE_POINTER: u32 = 0x170;
    const QUEUE_HEAD: u32 = 0;
    const QUEUE_COUNT: u32 = 0x1c;
    const SOURCE_SECONDARY_COUNT: u32 = 0x6c;
    const NODE_NEXT: u32 = 0;
    const NODE_DESTINATION_LINK: u32 = 4;

    unsafe {
        let source_queue = (owner.wrapping_add(SOURCE_QUEUE_POINTER) as *const u32).read();
        let source_head = (source_queue.wrapping_add(QUEUE_HEAD) as *const u32).read();
        if source_head == 0 {
            return 0;
        }

        let source_next = (source_head.wrapping_add(NODE_NEXT) as *const u32).read();
        let source_count = source_queue.wrapping_add(QUEUE_COUNT) as *mut u32;
        source_count.write((*source_count).wrapping_sub(1));
        let secondary_count = source_queue.wrapping_add(SOURCE_SECONDARY_COUNT) as *mut u32;
        secondary_count.write((*secondary_count).wrapping_add(1));
        (source_queue.wrapping_add(QUEUE_HEAD) as *mut u32).write(source_next);

        let destination_head = (destination.wrapping_add(QUEUE_HEAD) as *const u32).read();
        if destination_head != 0 {
            (destination_head.wrapping_add(NODE_DESTINATION_LINK) as *mut u32).write(source_head);
        }
        (source_head.wrapping_add(NODE_NEXT) as *mut u32).write(destination_head);
        (source_head.wrapping_add(NODE_DESTINATION_LINK) as *mut u32).write(0);
        let destination_count = destination.wrapping_add(QUEUE_COUNT) as *mut u32;
        destination_count.write((*destination_count).wrapping_add(1));
        (destination.wrapping_add(QUEUE_HEAD) as *mut u32).write(source_head);
        source_head
    }
});
