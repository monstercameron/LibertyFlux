// original: 0x0066d610 rb_tree_rotate_left

/// Rotate a node above its left child in the tree rooted at `this`. The tree
/// stores its root at byte offset zero. Nodes store a tagged right link at
/// `+4`, a left link at `+8`, and a parent pointer at `+12`; the low bit on a
/// right link is preserved as a tag. The left child's right subtree moves to
/// the node, and its parent link is repaired when the subtree is non-null.
/// The promoted child inherits the node's prior parent, then replaces the
/// root, right link, or left link according to the node's prior position.
/// Thiscall takes one stack pointer and returns the promoted child's updated
/// tagged right link.
lf_checker_rt::export!(thiscall, rw_0066d610(this: u32, node: u32) -> u32 {
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

        let left_child = read_word(node + NODE_LEFT);
        let tagged_transfer = read_word(left_child + NODE_RIGHT_TAGGED);
        let transferred_subtree = tagged_transfer & !RIGHT_LINK_TAG;
        write_word(node + NODE_LEFT, transferred_subtree);
        if transferred_subtree != 0 {
            write_word(transferred_subtree + NODE_PARENT, node);
        }

        let old_parent = read_word(node + NODE_PARENT);
        write_word(left_child + NODE_PARENT, old_parent);

        if read_word(this + TREE_ROOT) == node {
            write_word(this + TREE_ROOT, left_child);
        } else {
            let old_parent_right = read_word(old_parent + NODE_RIGHT_TAGGED);
            if node == old_parent_right & !RIGHT_LINK_TAG {
                write_word(
                    old_parent + NODE_RIGHT_TAGGED,
                    (old_parent_right & RIGHT_LINK_TAG) | left_child,
                );
            } else {
                write_word(old_parent + NODE_LEFT, left_child);
            }
        }

        let child_right = read_word(left_child + NODE_RIGHT_TAGGED);
        let updated_child_right = (child_right & RIGHT_LINK_TAG) | node;
        write_word(left_child + NODE_RIGHT_TAGGED, updated_child_right);
        write_word(node + NODE_PARENT, left_child);
        updated_child_right
    }
});
