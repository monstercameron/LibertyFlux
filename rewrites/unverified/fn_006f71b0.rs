// original: 0x006F71B0 mainloop_timer_iterator_advance

/// Advance the cursor through the secondary tagged tree in-order. When a
/// current node has a left subtree, select that subtree's rightmost node;
/// otherwise climb parent links until leaving a left child. At a tagged end
/// marker, clear its tag and select the tree root's rightmost node. Copy the
/// updated 16-bit key, current node, and end pointer to the caller's result.
/// The method uses thiscall and one pointer argument on the stack.
lf_checker_rt::export!(thiscall, rw_006f71b0(cursor: u32, result: u32) -> u32 {
    const CURSOR_KEY: u32 = 0x00;
    const CURSOR_NODE: u32 = 0x04;
    const CURSOR_END: u32 = 0x08;
    const NODE_LEFT: u32 = 0x4c;
    const NODE_RIGHT: u32 = 0x50;
    const NODE_PARENT: u32 = 0x54;
    const NODE_KEY: u32 = 0x58;
    const TAG_MASK: u32 = 1;
    const POINTER_MASK: u32 = !TAG_MASK;
    unsafe fn read32(address: u32) -> u32 { (address as *const u32).read_unaligned() }
    unsafe fn write32(address: u32, value: u32) { (address as *mut u32).write_unaligned(value); }
    unsafe fn read16(address: u32) -> u16 { (address as *const u16).read_unaligned() }
    unsafe fn write16(address: u32, value: u16) { (address as *mut u16).write_unaligned(value); }

    unsafe {
        let mut current = read32(cursor + CURSOR_NODE);
        if current != 0 {
            let left_subtree = read32(current + NODE_LEFT) & POINTER_MASK;
            if left_subtree != 0 {
                let mut successor = left_subtree;
                loop {
                    let right_child = read32(successor + NODE_RIGHT);
                    if right_child == 0 { break; }
                    successor = right_child;
                }
                current = successor;
            } else {
                let mut child = current;
                let mut parent = read32(child + NODE_PARENT);
                while parent != 0 && (read32(parent + NODE_LEFT) & POINTER_MASK) == child {
                    child = parent;
                    parent = read32(parent + NODE_PARENT);
                }
                current = parent;
            }
            write32(cursor + CURSOR_NODE, current);
        } else {
            let tagged_end = read32(cursor + CURSOR_END);
            if tagged_end & TAG_MASK != 0 && tagged_end & POINTER_MASK != 0 {
                let tree = tagged_end & POINTER_MASK;
                write32(cursor + CURSOR_END, tree);
                let mut rightmost = read32(tree);
                loop {
                    let right_child = read32(rightmost + NODE_RIGHT);
                    if right_child == 0 { break; }
                    rightmost = right_child;
                }
                current = rightmost;
                write32(cursor + CURSOR_NODE, current);
            }
        }

        if current != 0 {
            write16(cursor + CURSOR_KEY, read16(current + NODE_KEY));
        }
        write16(result, read16(cursor + CURSOR_KEY));
        write32(result + 4, read32(cursor + CURSOR_NODE));
        write32(result + 8, read32(cursor + CURSOR_END));
    }
    result
});
