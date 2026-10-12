// original: 0x00AAC000 node_move_between_queues
/// Removes the first node from the object's source queue. If a node exists,
/// prepends it to the destination queue and returns that node; otherwise it
/// returns zero. Both queue addresses are derived from the 32-bit owner.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00aac000(owner: u32) -> u32 {
    const SOURCE_QUEUE_OFFSET: u32 = 0xD010;
    const DESTINATION_QUEUE_OFFSET: u32 = 0x143044;
    let source_queue = owner.wrapping_add(SOURCE_QUEUE_OFFSET);
    let destination_queue = owner.wrapping_add(DESTINATION_QUEUE_OFFSET);
    let item = lf_checker_rt::callee_thiscall!(1, u32, source_queue);
    if item != 0 {
        lf_checker_rt::callee_thiscall!(2, u32, destination_queue, item);
    }
    item
}
