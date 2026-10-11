// original: 0x00c44f50 CCamIntermezzo::append_event
/// Allocate a 16-byte node from the intermezzo event list, store the supplied
/// event pointer at node offset zero, and return that pointer. The allocator
/// receives this+0x148 in ECX and one stack argument, the node size.
lf_checker_rt::export!(thiscall, rw_00c44f50(this: u32, event: u32) -> u32 {
    const EVENT_LIST: u32 = 0x148;
    const NODE_SIZE: u32 = 0x10;
    const ALLOCATE_NODE: u32 = 1;
    let list = this + EVENT_LIST;
    let node = lf_checker_rt::callee_thiscall!(ALLOCATE_NODE, u32, list, NODE_SIZE);
    unsafe { (node as *mut u32).write_unaligned(event); }
    event
});
