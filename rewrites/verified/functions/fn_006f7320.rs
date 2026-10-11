// original: 0x006F7320 mainloop_timer_tree_lower_bound

/// Search the binary tree rooted at `this + 0` for the first node whose
/// 16-bit key has a nonnegative wrapped difference from `key`. The original
/// uses the sign bit of the 16-bit subtraction (not a 32-bit comparison):
/// it remembers a matching candidate and continues through the left link,
/// or follows the tagged right link when the difference's sign bit is set.
/// It writes the candidate key and node to the first stack argument, writes
/// the tree object pointer there as well, and returns no meaningful value.
lf_checker_rt::export!(thiscall, rw_006f7320(this: u32, result: u32, key_ptr: u32, _unused: u32) -> u32 {
    unsafe {
        const ROOT: u32 = 0x00;
        const RIGHT_TAGGED: u32 = 0x0c;
        const LEFT: u32 = 0x10;
        const KEY: u32 = 0x18;

        let target = (key_ptr as *const u16).read_unaligned();
        let mut current = ((this + ROOT) as *const u32).read_unaligned();
        let mut candidate = 0u32;
        while current != 0 {
            let node_key = ((current + KEY) as *const u16).read_unaligned();
            let difference = node_key.wrapping_sub(target);
            if difference & 0x8000 == 0 {
                candidate = current;
                current = ((current + LEFT) as *const u32).read_unaligned();
            } else {
                current = ((current + RIGHT_TAGGED) as *const u32).read_unaligned() & !1;
            }
        }

        (result as *mut u16).write_unaligned(0);
        ((result + 4) as *mut u32).write_unaligned(0);
        ((result + 8) as *mut u32).write_unaligned(this);
        if candidate != 0 {
            let candidate_key = ((candidate + KEY) as *const u16).read_unaligned();
            (result as *mut u16).write_unaligned(candidate_key);
            ((result + 4) as *mut u32).write_unaligned(candidate);
        }
    }
    0
});
