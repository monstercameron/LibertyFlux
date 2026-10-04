// original: 0x00e5f2a0 link_regnode_eee8
/// Prepend this unit's node to the shared registry list.
///
/// Reads the list head, stores the previous head into this node's link slot
/// (the word after the node base), and swings the head to the node base.
/// Returns the previous head, matching the value the original leaves in EAX.
export!(cdecl, rw_00e5f2a0() -> u32 {
    unsafe {
        const HEAD: u32 = 0x17ACD24;
        const NODE: u32 = 0x110EEE8;
        let head = global::<u32>(HEAD);
        let prev = head.read();
        (global::<u32>(NODE + 4)).write(prev);
        head.write(relocated(NODE));
        prev
    }
});
