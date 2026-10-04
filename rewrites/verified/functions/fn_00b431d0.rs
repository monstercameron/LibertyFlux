// original: 0x00b431d0 unlink_chain_node
/// Unlink a node from its doubly linked chain.
///
/// When the node heads its parent's list the parent is repointed at the
/// next node, neighbours are stitched past it, and the node's own links
/// are cleared. No defined return value.
export!(cdecl, rw_b431d0(node: u32) -> u32 {
    unsafe {
        let parent = (node as *const u32).byte_add(0x25c).read();
        if (parent as *const u32).byte_add(0x18).read() == node {
            (parent as *mut u32)
                .byte_add(0x18)
                .write((node as *const u32).byte_add(0x258).read());
        }
        let prev = (node as *const u32).byte_add(0x254).read();
        if prev != 0 {
            (prev as *mut u32)
                .byte_add(0x258)
                .write((node as *const u32).byte_add(0x258).read());
        }
        let next = (node as *const u32).byte_add(0x258).read();
        if next != 0 {
            (next as *mut u32)
                .byte_add(0x254)
                .write((node as *const u32).byte_add(0x254).read());
        }
        (node as *mut u32).byte_add(0x25c).write(0);
        (node as *mut u32).byte_add(0x254).write(0);
        (node as *mut u32).byte_add(0x258).write(0);
        0
    }
});
