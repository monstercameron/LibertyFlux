// original: 0x00a7c9f0 bu_task_root
/// Follows parent links to the top of the tree and returns the root node.
export!(thiscall, rw_00a7c9f0(obj: *mut u8) -> u32 {
    unsafe {
        let mut cur = obj as u32;
        loop {
            let parent = *((cur + 0x118) as *const u32);
            if parent == 0 {
                return cur;
            }
            cur = parent;
        }
    }
});
