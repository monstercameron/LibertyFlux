// original: 0x00a7c550 bu_task_unlink
/// Removes the node from its doubly linked sibling list, re-points the
/// parent's child link when it addressed this node, clears the node's own
/// links and returns the previous sibling.
export!(thiscall, rw_00a7c550(obj: *mut u8) -> u32 {
    unsafe {
        let prev = *((obj as *const u8).add(0x11c) as *const u32);
        let parent = *((obj as *const u8).add(0x118) as *const u32);
        let next = *((obj as *const u8).add(0x120) as *const u32);
        if prev != 0 {
            *((prev + 0x120) as *mut u32) = next;
        }
        if next != 0 {
            *((next + 0x11c) as *mut u32) = prev;
        }
        if parent != 0 && *((parent + 0x124) as *const u32) == obj as u32 {
            *((parent + 0x124) as *mut u32) = prev;
        }
        *((obj as *mut u8).add(0x11c) as *mut u32) = 0;
        *((obj as *mut u8).add(0x120) as *mut u32) = 0;
        *((obj as *mut u8).add(0x118) as *mut u32) = 0;
        prev
    }
});
