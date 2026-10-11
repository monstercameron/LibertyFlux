// original: 0x006F7BC0 mainloop_timer_secondary_tree_rotate_left

/// Rotate the root node in the secondary tagged tree layout one step to the
/// left. Its left, right and parent links are at offsets 0x4C, 0x50 and
/// 0x54. The child's left subtree moves across, its low-bit tag is retained,
/// and the tree root is replaced. Non-root attachment cases are outside this
/// contract.
lf_checker_rt::export!(thiscall, rw_006f7bc0(tree: u32, pivot: u32) -> u32 {
    unsafe {
        const ROOT: u32 = 0x00;
        const LEFT: u32 = 0x4c;
        const RIGHT: u32 = 0x50;
        const PARENT: u32 = 0x54;
        unsafe fn read32(address: u32) -> u32 { (address as *const u32).read_unaligned() }
        unsafe fn write32(address: u32, value: u32) { (address as *mut u32).write_unaligned(value); }

        let right_child = read32(pivot + RIGHT);
        let tagged_child_left = read32(right_child + LEFT);
        let moved_subtree = tagged_child_left & !1;
        write32(pivot + RIGHT, moved_subtree);
        if moved_subtree != 0 {
            write32(moved_subtree + PARENT, pivot);
        }
        let former_parent = read32(pivot + PARENT);
        write32(right_child + PARENT, former_parent);
        if read32(tree + ROOT) == pivot {
            write32(tree + ROOT, right_child);
        } else {
            let parent_left = read32(former_parent + LEFT);
            if parent_left & !1 == pivot {
                write32(former_parent + LEFT, right_child | (parent_left & 1));
            } else {
                write32(former_parent + RIGHT, right_child);
            }
        }
        write32(right_child + LEFT, pivot | (tagged_child_left & 1));
        write32(pivot + PARENT, right_child);
    }
    0
});
