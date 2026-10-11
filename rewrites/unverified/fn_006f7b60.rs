// original: 0x006F7B60 mainloop_timer_secondary_tree_rotate_right

/// Rotate the root node in the secondary tagged tree layout one step to the
/// right. Its left, right and parent links are at offsets 0x4C, 0x50 and
/// 0x54. The pivot link's low-bit tag is preserved, the child's right subtree
/// is moved across, and the tree root is replaced. Non-root attachment cases
/// are outside this contract.
lf_checker_rt::export!(thiscall, rw_006f7b60(tree: u32, pivot: u32) -> u32 {
    unsafe {
        const ROOT: u32 = 0x00;
        const LEFT: u32 = 0x4c;
        const RIGHT: u32 = 0x50;
        const PARENT: u32 = 0x54;
        unsafe fn read32(address: u32) -> u32 { (address as *const u32).read_unaligned() }
        unsafe fn write32(address: u32, value: u32) { (address as *mut u32).write_unaligned(value); }

        let tagged_left = read32(pivot + LEFT);
        let left_child = tagged_left & !1;
        let moved_subtree = read32(left_child + RIGHT);
        write32(pivot + LEFT, moved_subtree | (tagged_left & 1));
        if moved_subtree != 0 {
            write32(moved_subtree + PARENT, pivot);
        }
        let former_parent = read32(pivot + PARENT);
        write32(left_child + PARENT, former_parent);
        if read32(tree + ROOT) == pivot {
            write32(tree + ROOT, left_child);
        } else {
            let parent_right = read32(former_parent + RIGHT);
            if parent_right == pivot {
                write32(former_parent + RIGHT, left_child);
            } else {
                let parent_left = read32(former_parent + LEFT);
                write32(former_parent + LEFT, left_child | (parent_left & 1));
            }
        }
        write32(left_child + RIGHT, pivot);
        write32(pivot + PARENT, left_child);
    }
    0
});
