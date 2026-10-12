// original: 0x0066d5b0 rb_tree_rotate_right

/// Rotate a node above its tagged right child in the tree rooted at `this`.
/// The tree stores its root at byte offset zero. Nodes store a tagged right
/// link at `+4`, a left link at `+8`, and a parent pointer at `+12`; the low
/// bit of a right link is preserved as a tag. The node's right child's left
/// subtree moves to the node, and its parent link is repaired when present.
/// The new child takes the node's old parent, then replaces the root, left
/// link, or tagged right link according to the node's prior position. The
/// function is thiscall with one stack pointer argument and returns the
/// original parent on the root and left-link paths, or the updated tagged
/// parent's right link on the remaining path.
lf_checker_rt::export!(thiscall, rw_0066d5b0(this: u32, node: u32) -> u32 {
    unsafe {
        const TREE_ROOT: u32 = 0;
        const NODE_RIGHT_TAGGED: u32 = 4;
        const NODE_LEFT: u32 = 8;
        const NODE_PARENT: u32 = 12;
        const RIGHT_LINK_TAG: u32 = 1;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let tagged_right = read_word(node + NODE_RIGHT_TAGGED);
        let tag = tagged_right & RIGHT_LINK_TAG;
        let right_child = tagged_right & !RIGHT_LINK_TAG;
        let transferred_subtree = read_word(right_child + NODE_LEFT);
        write_word(node + NODE_RIGHT_TAGGED, tag | transferred_subtree);
        if transferred_subtree != 0 {
            write_word(transferred_subtree + NODE_PARENT, node);
        }

        let old_parent = read_word(node + NODE_PARENT);
        write_word(right_child + NODE_PARENT, old_parent);

        if read_word(this + TREE_ROOT) == node {
            write_word(this + TREE_ROOT, right_child);
            write_word(right_child + NODE_LEFT, node);
            write_word(node + NODE_PARENT, right_child);
            return old_parent;
        }

        if read_word(old_parent + NODE_LEFT) == node {
            write_word(old_parent + NODE_LEFT, right_child);
            write_word(right_child + NODE_LEFT, node);
            write_word(node + NODE_PARENT, right_child);
            return old_parent;
        }

        let old_parent_right = read_word(old_parent + NODE_RIGHT_TAGGED);
        let updated_parent_right = (old_parent_right & RIGHT_LINK_TAG) | right_child;
        write_word(old_parent + NODE_RIGHT_TAGGED, updated_parent_right);
        write_word(right_child + NODE_LEFT, node);
        write_word(node + NODE_PARENT, right_child);
        updated_parent_right
    }
});
